//! What the Component shelf offers, and what a click on one of its rows does.
//!
//! Three claims carry most of these. A row's identity survives storage — the
//! pin set and the recent history hold opaque keys, so what a reader pinned
//! outlives the session, the project, and a detached library. A placement is
//! credited to the shelf only when the shelf offered it, so a paste, an
//! import, or a script never writes the reader's history. And every glyph in
//! the identity column is painted rather than typeset, because the bundled
//! faces do not hold the marks the mockup's catalog uses.

use super::super::shelf::*;
use super::*;
use crate::schematic::component_palette;
use crate::schematic::view::SchematicShelfDragPayload;
use crate::state::model_hub::{ModelHubPartRow, PartPlacement, PartProvenance, PartState};
use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};
use crate::state::{
    ComponentType, Tool, builtin_xspice_library_binding, builtin_xspice_vector_ports,
    engine_only_xspice_devices,
};

#[test]
fn mockup_primitive_groups_cover_every_placeable_palette_entry_once() {
    let entries = PRIMITIVE_GROUPS
        .iter()
        .flat_map(|(_, _, _, sections)| primitive_entries(sections))
        .collect::<Vec<_>>();
    let unique = entries
        .iter()
        .map(|entry| entry.kind)
        .collect::<HashSet<_>>();

    assert_eq!(entries.len(), primitive_entry_count());
    assert_eq!(unique.len(), entries.len());
}

#[test]
fn shelf_search_matches_labels_case_insensitively() {
    assert!(matches_query("nmos", &["NMOS", "Semiconductors"]));
    assert!(!matches_query("nmos", &["Resistor", "Passives"]));
}

#[test]
fn shelf_match_count_drives_a_truthful_filtered_empty_state() {
    let app = RSpiceApp::test_instance();
    assert!(component_shelf_match_count(&app.state, "resistor") > 0);
    assert_eq!(
        component_shelf_match_count(&app.state, "no-such-component-or-cell"),
        0
    );
}

#[test]
fn palette_placement_cancels_every_unfinished_conductor_route() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .schematic
        .start_wire(crate::state::Point::origin());
    app.state
        .schematic
        .start_bus(crate::state::Point::new(2, 3), None)
        .unwrap();

    arm_primitive(&mut app, ComponentType::Resistor, &egui::Context::default());

    assert_eq!(
        app.state.schematic.tool,
        Tool::Place(ComponentType::Resistor)
    );
    assert!(!app.state.schematic.wire_drawing.active);
    assert!(!app.state.schematic.bus_drawing.active);
}

#[test]
fn port_shelf_entry_uses_the_typed_place_pin_transaction() {
    let mut app = RSpiceApp::test_instance();

    arm_primitive(&mut app, ComponentType::Port, &egui::Context::default());

    assert!(app.state.dialogs.pin_port.open);
    assert_eq!(app.state.schematic.tool, Tool::Select);
    assert!(app.state.schematic.pending_port.is_none());
    assert!(app.state.schematic.components.is_empty());
}

/// A library of pinned bytes the project retained: one macromodel, one card a
/// device family draws, one card no family does, and one section-scoped
/// subcircuit key a netlist cannot reference by name.
fn retained_model_library(name: &str) -> ModelLibrary {
    use crate::state::model_library::{DeviceModel, ModelSubcircuitInterface, ModelType};

    let mut library = ModelLibrary::new(name);
    library.source_authority = ModelSourceAuthority::External;
    for key in ["PROVING_DIV", "SECTIONED\u{1f}LOCAL"] {
        library.subcircuits.insert(
            key.to_owned(),
            ModelSubcircuitInterface {
                name: key.to_owned(),
                ports: vec!["IN".to_owned(), "OUT".to_owned(), "GND".to_owned()],
                parameter_defaults: std::collections::BTreeMap::new(),
                description: None,
                file_path: None,
                source_line: None,
                section: None,
            },
        );
    }
    let mut zener = DeviceModel::new("RSPICE_ZENER", ModelType::Diode);
    zener.spice_type = Some("D".to_owned());
    library.add_model(zener);
    library.add_model(DeviceModel::new("VENDOR_PRIVATE", ModelType::Other));
    library
}

/// The canvas-side shelf can place what the project already holds: a retained
/// part arms the cursor directly instead of re-raising the pack confirmation.
/// This is the road that used to dead-end — the row was filtered out of this
/// section, and the Project-library section below reads the symbol library,
/// not the model-library manager, so an adopted pack part had no shelf door.
#[test]
fn a_part_the_project_retained_arms_from_the_component_shelf() {
    let mut app = RSpiceApp::test_instance();
    app.state
        .model_library_manager
        .add_library(retained_model_library("proving_parts"));

    let rows = library_part_rows(&app, "proving_parts");
    let row = |name: &str| {
        rows.iter()
            .find(|row| row.part_id == name)
            .unwrap_or_else(|| panic!("no shelf row for '{name}' in {rows:?}"))
    };

    let macromodel = row("PROVING_DIV");
    // The catalog's word for a subcircuit, not a second one: the same column
    // carries pack rows, and the class chip that narrows it spells it this way.
    assert_eq!(macromodel.meta, "subckt · in project");
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

/// A compiled-in foundation part arms from its built-in library, and its meta
/// says it is built in rather than "installed".
#[test]
fn a_foundation_part_arms_from_its_built_in_library() {
    use crate::state::model_library::{DeviceModel, ModelType};

    let mut library = ModelLibrary::new("rspice_foundation_probe");
    let mut card = DeviceModel::new("RSPICE_PROBE_NPN", ModelType::Npn);
    card.spice_type = Some("NPN".to_owned());
    library.add_model(card);
    let libraries = vec![&library];
    let index = crate::state::model_hub::provider::part_index(
        &libraries,
        &[],
        None,
        &crate::state::model_hub::Recalls::default(),
        None,
    );

    let rows = shelf_rows(&index, &libraries, "");
    let row = rows
        .iter()
        .find(|row| row.part_id == "RSPICE_PROBE_NPN")
        .expect("the foundation card is on the shelf");
    assert!(row.meta.ends_with("built in"), "{}", row.meta);
    assert!(
        matches!(&row.action, LibraryPartAction::Arm(placement)
        if matches!(placement.as_ref(), PartPlacement::NativeDevice {
            component_type: ComponentType::NpnBjt,
            ..
        })),
        "{:?}",
        row.action
    );
}

/// A part the project adopted is offered once, as the armable retained row —
/// not a second time as its pack's "review and add" row.
#[test]
fn an_adopted_part_is_offered_once_as_the_retained_row() {
    use rspice_pack::PartKind;

    let library = retained_model_library("proving_parts");
    let libraries = vec![&library];
    let mut index = crate::state::model_hub::provider::part_index(
        &libraries,
        &[],
        None,
        &crate::state::model_hub::Recalls::default(),
        None,
    );
    index.push(ModelHubPartRow {
        part_id: "RSPICE_ZENER".to_owned(),
        kind: PartKind::Model,
        device: "diode".to_owned(),
        terminals: vec!["A".to_owned(), "K".to_owned()],
        provenance: PartProvenance::InstalledPack {
            pack_id: "rspice-diodes".to_owned(),
            version: "1.0.0".to_owned(),
        },
        state: PartState::Installed,
        pack_name: Some("RSpice diodes".to_owned()),
        source: None,
    });

    let rows = shelf_rows(&index, &libraries, "");
    let zener = rows
        .iter()
        .filter(|row| row.part_id == "RSPICE_ZENER")
        .collect::<Vec<_>>();
    assert_eq!(zener.len(), 1, "{rows:?}");
    assert!(matches!(zener[0].action, LibraryPartAction::Arm(_)));
}

/// Pack releases the project has not adopted keep their two states: review
/// for one this engine can run, and a refusal naming the missing capability
/// for one it cannot.
#[test]
fn an_unadopted_release_reviews_and_an_incompatible_one_refuses() {
    use rspice_pack::PartKind;

    let release = |part: &str, state: PartState| ModelHubPartRow {
        part_id: part.to_owned(),
        kind: PartKind::Subckt,
        device: "opamp".to_owned(),
        terminals: Vec::new(),
        provenance: PartProvenance::RemoteRelease {
            pack_id: "vendor-amps".to_owned(),
            version: "2.1.0".to_owned(),
        },
        state,
        pack_name: Some("Vendor amplifiers".to_owned()),
        source: None,
    };
    let index = vec![
        release("VENDOR_OA1", PartState::Available),
        release(
            "VENDOR_RF9",
            PartState::Incompatible {
                missing: vec!["harmonic-balance-2".to_owned()],
            },
        ),
    ];

    let rows = shelf_rows(&index, &[], "");
    assert_eq!(
        rows[0].action,
        LibraryPartAction::Review {
            pack_id: "vendor-amps".to_owned(),
            version: "2.1.0".to_owned(),
            pack_name: "Vendor amplifiers".to_owned(),
        }
    );
    let LibraryPartAction::Refused(reason) = &rows[1].action else {
        panic!("an incompatible release is refused: {:?}", rows[1].action);
    };
    assert!(
        reason.contains("harmonic-balance-2") && reason.contains("Vendor amplifiers"),
        "{reason}"
    );
    assert_eq!(rows[1].meta, "opamp · needs harmonic-balance-2");
}

// ------------------------------------------------------------ shelf identity

/// Every glyph the Component shelf can paint, deduplicated: each placeable
/// palette entry's, each primitive group band's, and the marks the XSPICE,
/// Verilog-A and library sections use.
#[cfg(not(target_arch = "wasm32"))]
fn every_shelf_glyph() -> Vec<ShelfGlyph> {
    let mut glyphs = vec![
        // Built-in XSPICE rows and their band.
        ShelfGlyph::Event,
        // Generated Verilog-A rows and their band.
        ShelfGlyph::Text("VA"),
        // Library parts and project-library bands.
        ShelfGlyph::Icon(WorkbenchIcon::Models),
    ];
    glyphs.extend(PRIMITIVE_GROUPS.iter().map(|(_, glyph, _, _)| *glyph));
    for section in component_palette() {
        for entry in section.entries {
            glyphs.push(primitive_shelf_glyph(entry.kind));
        }
    }
    let mut distinct = Vec::new();
    for glyph in glyphs {
        if !distinct.contains(&glyph) {
            distinct.push(glyph);
        }
    }
    distinct
}

/// Every glyph in the shelf table puts real ink in the glyph slot, and no
/// text glyph is the replacement box the text layouter substitutes for a
/// character the bundled faces lack.
///
/// The tofu reference is rendered through the same paint path from two
/// characters no bundled face holds; that the pair renders identically is
/// what proves a missing glyph has one shape, which every table glyph must
/// then differ from. This is the gate that catches a future table entry
/// reaching for ◯, ▷, ⊞, Σ, Ω or anything else outside the Latin-subset
/// Plex cuts: it would compile, paint a plausible box of ink, and fail here.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn every_shelf_glyph_paints_ink_and_no_text_glyph_is_a_tofu_box() {
    let pattern = |glyph: ShelfGlyph| -> Vec<egui::Color32> {
        let canvas = glyph_canvas(glyph);
        canvas
            .pixels_in(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(24.0, 24.0),
            ))
            .collect()
    };
    let background = glyph_canvas(ShelfGlyph::Text(" ")).background();
    let tofu = pattern(ShelfGlyph::Text("\u{2603}"));
    assert_eq!(
        tofu,
        pattern(ShelfGlyph::Text("\u{E001}")),
        "two characters the faces lack must share one replacement shape for \
         the tofu comparison to mean anything"
    );

    let glyphs = every_shelf_glyph();
    assert!(
        glyphs.len() >= 20,
        "the table lost its vocabulary: {glyphs:?}"
    );
    for glyph in glyphs {
        let painted = pattern(glyph);
        assert!(
            painted.iter().any(|pixel| *pixel != background),
            "{glyph:?} paints no ink in the glyph slot"
        );
        if let ShelfGlyph::Text(text) = glyph {
            assert_ne!(
                painted, tofu,
                "'{text}' is not in the bundled faces and would reach the \
                 shelf as a tofu box"
            );
        }
    }
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
                    primitive_catalog(ui, state);
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

    for passive in ["Resistor", "Capacitor", "Transmission Line"] {
        assert!(
            text.contains(passive),
            "a fresh shelf shows the Passives rows, missing {passive}: {text}"
        );
    }
    for folded in ["Voltage Source", "NMOS", "AND Gate", "Op-Amp"] {
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
    for band in ["Sources", "Analog", "Mixed signal / XSPICE"] {
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

/// The shelf's meta column states the designator prefix and the default the
/// placed instance will carry, spelled through the crate's engineering
/// formatter — and only what a part actually has: a modelled device keeps its
/// prefix alone, a structural row says nothing.
#[test]
fn shelf_meta_states_the_prefix_and_the_placed_default() {
    for (kind, meta) in [
        (ComponentType::Resistor, Some("R \u{00b7} 1k")),
        (ComponentType::Capacitor, Some("C \u{00b7} 1u")),
        (ComponentType::Inductor, Some("L \u{00b7} 1m")),
        (ComponentType::VoltageSource, Some("V \u{00b7} 5")),
        (ComponentType::OpAmp, Some("E \u{00b7} 100k")),
        // The one default not authored in the formatter's spelling: the
        // formatter's decade wins, as it does in the property editor.
        (ComponentType::CoupledInductor, Some("K \u{00b7} 990m")),
        // A default no engineering parser reads is stated as authored.
        (ComponentType::BehavioralSource, Some("B \u{00b7} V=0")),
        // Modelled devices have no meaningful default value.
        (ComponentType::Diode, Some("D")),
        (ComponentType::Nmos, Some("M")),
        (ComponentType::Memristor, Some("MR")),
        // Structural objects own neither a designator nor a value.
        (ComponentType::Ground, None),
        (ComponentType::Port, None),
    ] {
        assert_eq!(
            primitive_shelf_meta(kind).as_deref(),
            meta,
            "meta column of {kind:?}"
        );
    }
}

/// Rows of different families paint different identity glyphs.
///
/// The label and meta are pinned to one string so the glyph slot is the only
/// thing that can differ — which is exactly what one shared icon, or two
/// missing characters both rendering as the replacement box, would fail.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn shelf_rows_of_different_families_paint_distinct_glyph_slots() {
    let row = |kind: ComponentType| {
        crate::ui::raster::render(egui::vec2(160.0, 24.0), move |ui, background| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE.fill(background))
                .show(ui, |ui| {
                    shelf_part_row(ui, primitive_shelf_glyph(kind), "PART", false, Some("m"), 0);
                });
        })
    };
    // The identity slot: the 15 px glyph rect at the schematic row's icon
    // column, with a pixel of air on each side.
    let slot = egui::Rect::from_center_size(egui::pos2(33.5, 12.0), egui::vec2(17.0, 17.0));
    let slot_pixels = |canvas: &crate::ui::raster::Canvas| -> Vec<egui::Color32> {
        canvas.pixels_in(slot).collect()
    };

    let resistor = row(ComponentType::Resistor);
    let capacitor = row(ComponentType::Capacitor);
    let xspice = row(ComponentType::XspiceGain);
    for (name, canvas) in [
        ("resistor", &resistor),
        ("capacitor", &capacitor),
        ("XSPICE", &xspice),
    ] {
        assert!(
            slot_pixels(canvas)
                .iter()
                .any(|pixel| *pixel != canvas.background()),
            "the {name} row's glyph slot is empty"
        );
    }
    assert_ne!(
        slot_pixels(&resistor),
        slot_pixels(&capacitor),
        "a resistor row and a capacitor row paint one glyph"
    );
    assert_ne!(
        slot_pixels(&resistor),
        slot_pixels(&xspice),
        "a resistor row and an XSPICE row paint one glyph"
    );
    assert_ne!(
        slot_pixels(&capacitor),
        slot_pixels(&xspice),
        "a capacitor row and an XSPICE row paint one glyph"
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
        place_builtin_xspice(&mut app.state, fixed.stable_id).is_some(),
        "{} has nothing left to choose, so it arms straight away",
        fixed.stable_id
    );
    assert!(
        !app.state.dialogs.builtin_xspice_placement.open,
        "and asks nothing"
    );

    assert!(
        place_builtin_xspice(&mut app.state, open.stable_id).is_none(),
        "{} cannot be armed until its widths are chosen",
        open.stable_id
    );
    assert!(
        app.state.dialogs.builtin_xspice_placement.open,
        "so the click raises the placement dialog instead"
    );
    assert_eq!(
        app.state.dialogs.builtin_xspice_placement.stable_id, open.stable_id,
        "for the model the row named"
    );
}

// -------------------------------------------------- shelf pinning and recents

/// One frame of the whole Component shelf, through the same entry point the
/// dock calls — so the placement watch, the bands, and the catalog all see one
/// frame, as they do in the product.
#[cfg(not(target_arch = "wasm32"))]
fn component_shelf_output(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> egui::FullOutput {
    ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(260.0, 2400.0),
            )),
            events,
            ..egui::RawInput::default()
        },
        |ctx| {
            egui::CentralPanel::default()
                .frame(egui::Frame::NONE)
                .show(ctx, |ui| {
                    ui.set_width(260.0);
                    component_shelf(ui, app);
                });
        },
    )
}

/// What one such frame painted: the joined text and the runs it came from.
///
/// Both, because a gesture has to land on a row, and only the runs carry where
/// the rows are.
#[cfg(not(target_arch = "wasm32"))]
fn component_shelf_frame(
    ctx: &egui::Context,
    app: &mut RSpiceApp,
    events: Vec<egui::Event>,
) -> (String, Vec<(String, egui::Rect, egui::Color32)>) {
    let output = component_shelf_output(ctx, app, events);
    (painted_text(&output), painted_runs(&output))
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

/// The shelf held open across frames, driven to a settled layout.
///
/// Two frames: the first builds the font set, the second lays out against it,
/// and only then are the painted rects worth clicking.
#[cfg(not(target_arch = "wasm32"))]
fn settled_shelf() -> (egui::Context, RSpiceApp) {
    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    (ctx, app)
}

/// The stored history, as the durable domain holds it.
#[cfg(not(target_arch = "wasm32"))]
fn stored_recent(state: &AppState) -> Vec<String> {
    state.ui.preferences.component_shelf().recent
}

/// Arm one primitive and put it on the sheet, the way a click-to-arm placement
/// reaches the design.
#[cfg(not(target_arch = "wasm32"))]
fn place(state: &mut AppState, kind: ComponentType) {
    state.schematic.arm_tool(Tool::Place(kind));
    state
        .schematic
        .add_component(kind, crate::state::Point::new(40, 40));
}

/// Every shelf-entry key survives the round trip through the durable form,
/// including names that carry the characters a naive encoding would split on.
#[test]
fn every_shelf_entry_kind_round_trips_through_its_stored_key() {
    for entry in [
        ShelfEntry::Primitive(ComponentType::Resistor),
        ShelfEntry::Primitive(ComponentType::Ground),
        ShelfEntry::LibraryPart("1N4148".to_owned()),
        ShelfEntry::BuiltinXspice("gain".to_owned()),
        ShelfEntry::GeneratedVerilogA("bsimcmg".to_owned()),
        ShelfEntry::Cell {
            library: "work".to_owned(),
            cell: "opamp".to_owned(),
            view: "schematic".to_owned(),
        },
        // A cell whose names carry the slash a display path joins on and the
        // colon a key tag would.
        ShelfEntry::Cell {
            library: "an/odd lib".to_owned(),
            cell: "cell:with:colons".to_owned(),
            view: "symbol".to_owned(),
        },
    ] {
        let key = entry.storage_key();
        assert_eq!(
            ShelfEntry::from_storage_key(&key).as_ref(),
            Some(&entry),
            "{entry:?} did not survive the key {key:?}"
        );
    }

    // A key this build does not understand is refused rather than guessed at,
    // which is what lets the stored list keep it untouched.
    for unknown in ["", "future-family\u{1f}x", "primitive\u{1f}NoSuchType"] {
        assert_eq!(
            ShelfEntry::from_storage_key(unknown),
            None,
            "{unknown:?} must not resolve"
        );
    }
}

/// A fresh profile takes the shipped pin set, and the first pin or unpin makes
/// the reader's own set authoritative: the shipped list never merges back in,
/// so a default the reader removed stays removed.
#[test]
fn a_fresh_profile_takes_the_shipped_pins_until_the_reader_moves_one() {
    let mut app = RSpiceApp::test_instance();
    let key = |kind| ShelfEntry::Primitive(kind).storage_key();

    assert_eq!(
        pinned_keys(&app.state),
        vec![
            key(ComponentType::Resistor),
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
        ],
        "a profile that has never pinned anything sees the shipped set"
    );
    assert!(
        app.state.ui.preferences.component_shelf().pinned.is_none(),
        "and the shipped set is not yet written as the reader's own"
    );

    // A pin lands at the end: the rail the reader builds keeps its positions.
    toggle_pin(&mut app.state, &ShelfEntry::Primitive(ComponentType::Diode));
    assert_eq!(
        pinned_keys(&app.state),
        vec![
            key(ComponentType::Resistor),
            key(ComponentType::Capacitor),
            key(ComponentType::Ground),
            key(ComponentType::Diode),
        ]
    );

    // Unpinning a shipped default removes it for good.
    toggle_pin(
        &mut app.state,
        &ShelfEntry::Primitive(ComponentType::Resistor),
    );
    assert_eq!(
        pinned_keys(&app.state),
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
        toggle_pin(&mut app.state, &ShelfEntry::Primitive(kind));
    }
    assert_eq!(pinned_keys(&app.state), Vec::<String>::new());
    assert_eq!(
        app.state.ui.preferences.component_shelf().pinned,
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
    let (ctx, mut app) = settled_shelf();

    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(paints_line(&text, "PINNED"), "the fresh band is painted");
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
    toggle_pin(&mut app.state, &ShelfEntry::Primitive(ComponentType::Nmos));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
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
        toggle_pin(&mut app.state, &ShelfEntry::Primitive(kind));
    }
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        !paints_line(&text, "PINNED"),
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
    let (ctx, mut app) = settled_shelf();
    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let row = run_rect(&runs, "Transmission Line")
        .expect("the Passives band paints a Transmission Line row");
    let entry = ShelfEntry::Primitive(ComponentType::TransmissionLine);
    assert!(!is_pinned(&app.state, &entry), "the row starts unpinned");

    let _ = component_shelf_frame(&ctx, &mut app, secondary_click_events(row.center()));
    let (text, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Pin to shelf"),
        "the menu offers the position the row is not in: {text}"
    );

    let pin = run_rect(&runs, "Pin to shelf").expect("the open menu paints its one command");
    let _ = component_shelf_frame(&ctx, &mut app, click_events(pin.center()));
    assert!(
        is_pinned(&app.state, &entry),
        "the command pinned the row it was opened on"
    );

    // The pinned row now leads the shelf, and its menu offers the other
    // position — the same control, stating where the row stands.
    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let pinned_row =
        run_rect(&runs, "Transmission Line").expect("the pinned row is painted at the top");
    let _ = component_shelf_frame(&ctx, &mut app, secondary_click_events(pinned_row.center()));
    let (text, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Unpin") && !paints_line(&text, "Pin to shelf"),
        "a pinned row's menu offers only the way back out: {text}"
    );

    let unpin = run_rect(&runs, "Unpin").expect("the open menu paints its one command");
    let _ = component_shelf_frame(&ctx, &mut app, click_events(unpin.center()));
    assert!(
        !is_pinned(&app.state, &entry),
        "and the command takes the row back off the rail"
    );
}

/// Placement feeds Recent, newest first, with a re-place moving the part to
/// the front instead of listing it twice.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn placement_feeds_recent_front_ordered_and_deduplicated() {
    let (ctx, mut app) = settled_shelf();
    let key = |kind| ShelfEntry::Primitive(kind).storage_key();

    assert!(
        stored_recent(&app.state).is_empty(),
        "a profile that has placed nothing has no history"
    );

    place(&mut app.state, ComponentType::Diode);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(stored_recent(&app.state), vec![key(ComponentType::Diode)]);
    assert!(paints_line(&text, "RECENT"), "the band appears: {text}");

    place(&mut app.state, ComponentType::Nmos);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        stored_recent(&app.state),
        vec![key(ComponentType::Nmos), key(ComponentType::Diode)],
        "the newest placement leads"
    );

    place(&mut app.state, ComponentType::Diode);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        stored_recent(&app.state),
        vec![key(ComponentType::Diode), key(ComponentType::Nmos)],
        "a re-place moves the part to the front rather than listing it twice"
    );
}

/// The band is capped: a session that places more parts than it can show keeps
/// the newest, and the ones that scrolled off are gone from the band.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn the_recent_band_shows_only_its_newest_entries() {
    let (ctx, mut app) = settled_shelf();

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
        place(&mut app.state, kind);
        let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    }
    assert_eq!(stored_recent(&app.state).len(), placed.len());

    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let painted = placed
        .iter()
        .filter(|kind| paints_line(&text, kind.display_name()))
        .count();
    assert_eq!(
        painted, RECENT_SHOWN,
        "the band paints its cap and no more: {text}"
    );
    for dropped in &placed[..placed.len() - RECENT_SHOWN] {
        assert!(
            !paints_line(&text, dropped.display_name()),
            "{} fell off the band and must not be painted: {text}",
            dropped.display_name()
        );
    }
}

/// A pinned part is not listed twice. It keeps its place in the stored
/// history, so unpinning restores it to the band rather than losing it.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_pinned_part_is_filtered_out_of_recent_but_kept_in_the_history() {
    let (ctx, mut app) = settled_shelf();
    let entry = ShelfEntry::Primitive(ComponentType::Diode);

    place(&mut app.state, ComponentType::Diode);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Diode"),
        "the placement is listed: {text}"
    );

    toggle_pin(&mut app.state, &entry);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        !paints_line(&text, "RECENT"),
        "the only recent part is pinned, so the band has nothing left: {text}"
    );
    assert!(
        paints_line(&text, "Diode"),
        "and it is painted once, in the Pinned band: {text}"
    );
    assert_eq!(
        stored_recent(&app.state),
        vec![entry.storage_key()],
        "the history keeps it, so unpinning restores its position"
    );

    toggle_pin(&mut app.state, &entry);
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "RECENT") && paints_line(&text, "Diode"),
        "unpinning returns it to the band: {text}"
    );
}

/// Only what the shelf offered the canvas is credited. A design that grows on
/// its own — a paste, an import, a script — is not this reader's placement
/// history, and the bands must not claim it is.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn a_design_that_grows_without_the_shelf_writes_no_history() {
    let (ctx, mut app) = settled_shelf();

    app.state.schematic.cancel_tool();
    app.state
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::new(40, 40));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());

    assert!(stored_recent(&app.state).is_empty(), "nothing was offered");
    assert!(
        !paints_line(&text, "RECENT"),
        "so there is no band to paint: {text}"
    );
}

/// Opening a different design does not read its existing objects as
/// placements: the watch adopts the new count in silence.
#[cfg(not(target_arch = "wasm32"))]
#[test]
fn switching_designs_credits_nothing_to_the_reader() {
    let (ctx, mut app) = settled_shelf();

    app.state
        .schematic
        .arm_tool(Tool::Place(ComponentType::Diode));
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());

    // A design the reader opened, already holding objects someone else placed.
    app.state.active_schematic_epoch += 1;
    for offset in 0..4 {
        app.state.schematic.add_component(
            ComponentType::Nmos,
            crate::state::Point::new(40 + offset, 40),
        );
    }
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        stored_recent(&app.state).is_empty(),
        "the objects already on an opened design were not placed by this reader"
    );

    // From there the watch tracks the new design normally.
    place(&mut app.state, ComponentType::Diode);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert_eq!(
        stored_recent(&app.state),
        vec![ShelfEntry::Primitive(ComponentType::Diode).storage_key()]
    );
}

/// The pin set and the placement history are per-user, cross-project state,
/// and they survive an application restart.
///
/// The boundary proved here is the real one: the same `eframe::Storage` RON
/// round trip `RSpiceApp::save` performs — `eframe::set_value(storage,
/// eframe::APP_KEY, &self.state)` — and `RSpiceApp::new` reads back. Nothing
/// about that store is per-project: it is the user's own application-data
/// file, and the browser's local storage on the web build.
#[test]
fn the_shelf_set_survives_the_restart_boundary_the_workbench_actually_crosses() {
    #[derive(Default)]
    struct MemoryStorage(std::collections::HashMap<String, String>);

    impl eframe::Storage for MemoryStorage {
        fn get_string(&self, key: &str) -> Option<String> {
            self.0.get(key).cloned()
        }

        fn set_string(&mut self, key: &str, value: String) {
            self.0.insert(key.to_owned(), value);
        }

        fn remove_string(&mut self, key: &str) {
            self.0.remove(key);
        }

        fn flush(&mut self) {}
    }

    let mut app = RSpiceApp::test_instance();
    toggle_pin(&mut app.state, &ShelfEntry::Primitive(ComponentType::Diode));
    toggle_pin(
        &mut app.state,
        &ShelfEntry::Primitive(ComponentType::Resistor),
    );
    record_placement(
        &mut app.state,
        &ShelfEntry::LibraryPart("1N4148".to_owned()),
    );
    record_placement(
        &mut app.state,
        &ShelfEntry::Cell {
            library: "work".to_owned(),
            cell: "opamp".to_owned(),
            view: "schematic".to_owned(),
        },
    );
    let saved = app.state.ui.preferences.component_shelf();

    let mut storage = MemoryStorage::default();
    eframe::set_value(&mut storage, eframe::APP_KEY, &app.state);
    let restored: crate::workbench::app_state::AppState =
        eframe::get_value(&storage, eframe::APP_KEY)
            .expect("RSpice must be able to restore a session it just saved");

    assert_eq!(
        restored.ui.preferences.component_shelf(),
        saved,
        "the shelf's personal set must cross the restart boundary intact"
    );
    assert_eq!(
        restored.ui.preferences.component_shelf().pinned,
        Some(vec![
            ShelfEntry::Primitive(ComponentType::Capacitor).storage_key(),
            ShelfEntry::Primitive(ComponentType::Ground).storage_key(),
            ShelfEntry::Primitive(ComponentType::Diode).storage_key(),
        ]),
        "including the order the reader pinned in"
    );
}

/// A fresh profile writes nothing: the domain is absent from the wire until
/// the reader has an answer of their own, so an untouched shelf costs a saved
/// session no bytes and no forward-compatibility surface.
#[test]
fn an_untouched_shelf_writes_no_preference_domain() {
    use crate::workbench::preferences::ComponentShelfPreferences;

    let mut preferences = crate::workbench::UserPreferences::default();
    let untouched = serde_json::to_value(&preferences).expect("preferences encode");

    preferences.set_component_shelf(ComponentShelfPreferences {
        pinned: Some(Vec::new()),
        recent: Vec::new(),
    });
    assert_ne!(
        serde_json::to_value(&preferences).expect("preferences encode"),
        untouched,
        "a set the reader emptied on purpose is not the same answer as no set"
    );

    preferences.set_component_shelf(ComponentShelfPreferences::default());
    assert_eq!(
        serde_json::to_value(&preferences).expect("preferences encode"),
        untouched,
        "and returning to the shipped default clears the domain again"
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

    let (ctx, mut app) = settled_shelf();
    // NMOS belongs to a band a fresh install folds, so the only NMOS row on
    // screen is the pinned one — the click cannot land on a catalog row by
    // accident.
    toggle_pin(&mut app.state, &ShelfEntry::Primitive(ComponentType::Nmos));
    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    let row = run_rect(&runs, "NMOS").expect("the pinned NMOS row is painted");
    let resting = tone(&runs, "NMOS");
    assert_ne!(
        app.state.schematic.tool,
        Tool::Place(ComponentType::Nmos),
        "nothing is armed before the click"
    );

    let _ = component_shelf_frame(&ctx, &mut app, click_events(row.center()));
    assert_eq!(
        app.state.schematic.tool,
        Tool::Place(ComponentType::Nmos),
        "the click armed the part the row names"
    );

    let (_, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
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
    let (ctx, mut app) = settled_shelf();
    let payload = SchematicShelfDragPayload::primitive(ComponentType::Nmos)
        .expect("a primitive travels to the canvas as a drag payload");

    // The drag in flight: an identity is on offer and nothing has landed.
    egui::DragAndDrop::set_payload(&ctx, payload);
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        stored_recent(&app.state).is_empty(),
        "a drag still in flight has placed nothing"
    );

    // The drop: the canvas takes the payload and commits the object.
    egui::DragAndDrop::clear_payload(&ctx);
    app.state
        .schematic
        .add_component(ComponentType::Nmos, crate::state::Point::new(40, 40));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());

    assert_eq!(
        stored_recent(&app.state),
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
    let (ctx, mut app) = settled_shelf();

    app.state
        .schematic
        .arm_tool(Tool::Place(ComponentType::Diode));
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    app.state.schematic.cancel_tool();
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    let _ = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        stored_recent(&app.state).is_empty(),
        "arming a tool and putting it away places nothing"
    );

    // The design grows on its own, well after the offer was withdrawn.
    app.state
        .schematic
        .add_component(ComponentType::Diode, crate::state::Point::new(40, 40));
    let (text, _) = component_shelf_frame(&ctx, &mut app, Vec::new());

    assert!(
        stored_recent(&app.state).is_empty(),
        "the shelf had nothing on offer when the design grew: {:?}",
        stored_recent(&app.state)
    );
    assert!(
        !paints_line(&text, "RECENT"),
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
        record_placement(&mut app.state, &part(index));
    }

    let recent = app.state.ui.preferences.component_shelf().recent;
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

    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let _ = component_shelf_output(&ctx, &mut app, Vec::new());
    let _ = component_shelf_output(&ctx, &mut app, Vec::new());

    app.state.workbench.focus_placement_search = true;
    let output = component_shelf_output(&ctx, &mut app, Vec::new());
    assert_eq!(
        focus(&output).as_deref(),
        Some("Place component or cell…"),
        "the shelf's filter takes the keyboard it was asked for"
    );

    // Step down to a folded catalog group the way a reader without a pointer
    // reaches it. The rail is finite, so a group that never takes the keyboard
    // is one such a reader could never open.
    let mut output = component_shelf_output(&ctx, &mut app, vec![key_event(egui::Key::ArrowDown)]);
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
        output = component_shelf_output(&ctx, &mut app, vec![key_event(egui::Key::ArrowDown)]);
    }
    assert!(landed, "the Sources group must be reachable by stepping");
    assert_eq!(
        expanded(&output, "Sources"),
        Some(false),
        "which a fresh shelf shows folded"
    );

    let _ = component_shelf_output(&ctx, &mut app, vec![key_event(egui::Key::ArrowRight)]);
    let output = component_shelf_output(&ctx, &mut app, Vec::new());
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

    let _ = component_shelf_output(&ctx, &mut app, vec![key_event(egui::Key::ArrowRight)]);
    let output = component_shelf_output(&ctx, &mut app, Vec::new());
    assert_eq!(
        focus(&output).as_deref(),
        Some("Voltage Source"),
        "a second Right steps onto the first part it disclosed"
    );

    let _ = component_shelf_output(&ctx, &mut app, vec![key_event(egui::Key::ArrowLeft)]);
    let output = component_shelf_output(&ctx, &mut app, Vec::new());
    assert_eq!(
        focus(&output).as_deref(),
        Some("Sources"),
        "and Left climbs out of a part to the group that holds it"
    );

    let _ = component_shelf_output(&ctx, &mut app, vec![key_event(egui::Key::ArrowLeft)]);
    let output = component_shelf_output(&ctx, &mut app, Vec::new());
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

    let mut app = RSpiceApp::test_instance();
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    ctx.enable_accesskit();
    let _ = component_shelf_output(&ctx, &mut app, Vec::new());
    let _ = component_shelf_output(&ctx, &mut app, Vec::new());

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
        let output = component_shelf_output(
            &ctx,
            &mut app,
            vec![key(egui::Key::Tab, egui::Modifiers::NONE)],
        );
        if focused_label(&output).as_deref() == Some("Transmission Line") {
            landed = true;
            break;
        }
    }
    assert!(landed, "a shelf row must be reachable from the keyboard");

    let _ = component_shelf_frame(
        &ctx,
        &mut app,
        vec![key(egui::Key::F10, egui::Modifiers::SHIFT)],
    );
    let (text, runs) = component_shelf_frame(&ctx, &mut app, Vec::new());
    assert!(
        paints_line(&text, "Pin to shelf"),
        "Shift+F10 opens the focused row's pin menu: {text}"
    );

    let pin = run_rect(&runs, "Pin to shelf").expect("the open menu paints its one command");
    let _ = component_shelf_frame(&ctx, &mut app, click_events(pin.center()));
    assert!(
        is_pinned(
            &app.state,
            &ShelfEntry::Primitive(ComponentType::TransmissionLine)
        ),
        "and the menu acts on the row the keyboard was on"
    );
}

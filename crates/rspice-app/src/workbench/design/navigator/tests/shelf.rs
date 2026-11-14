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
use crate::schematic::{
    ComponentCategory, PaletteRow, component_palette, palette_rows, supply_palette,
};
use crate::state::model_hub::{ModelHubPartRow, PartPlacement, PartProvenance, PartState};
use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};
use crate::state::{ComponentType, engine_only_xspice_devices};
use rspice_app_state::veriloga_library::BUILTIN_VERILOGA_MODELS;

/// The shelf's built-in Verilog-A rows read and stand as the Library page
/// lists them: its name, its device class and its order.
#[test]
fn built_in_veriloga_models_read_and_stand_as_the_library_lists_them() {
    let library = BUILTIN_VERILOGA_MODELS;
    let first = &library[0];
    assert_ne!(first.registry_name, first.name);
    assert_eq!(builtin_label(first.registry_name), first.name);
    assert_eq!(
        builtin_meta(4, first.registry_name, first.module),
        format!("4 pin \u{00b7} {}", first.class.label())
    );

    let mut names = library
        .iter()
        .rev()
        .map(|model| model.registry_name)
        .collect::<Vec<_>>();
    in_library_order(&mut names, |name| *name);
    assert_eq!(
        names,
        library
            .iter()
            .map(|model| model.registry_name)
            .collect::<Vec<_>>()
    );
}

/// Every built-in part is filed under exactly one functional group, and a
/// label names one row: two rows a reader cannot tell apart are one too many.
#[test]
fn every_built_in_part_is_filed_once_and_no_two_rows_share_a_label() {
    let rows = ComponentCategory::ALL
        .into_iter()
        .flat_map(palette_rows)
        .collect::<Vec<_>>();
    assert_eq!(rows.len(), catalog_row_count());
    let mut labels = HashSet::new();
    for row in &rows {
        assert!(
            labels.insert(row.label().to_lowercase()),
            "two shelf rows are labelled {:?}, in some case",
            row.label()
        );
    }
    let entries = component_palette()
        .iter()
        .map(|section| section.entries.len())
        .sum::<usize>();
    let filed_primitives = rows
        .iter()
        .filter(|row| matches!(row, PaletteRow::Primitive(_)))
        .count();
    assert_eq!(filed_primitives, entries);

    // A code model is filed unless it duplicates a native part, which the
    // shelf offers instead.
    let filed_models = rows
        .iter()
        .filter(|row| matches!(row, PaletteRow::Xspice(_)))
        .count();
    let grouped = engine_only_xspice_devices()
        .iter()
        .filter(|descriptor| descriptor.category.is_some())
        .count();
    assert_eq!(filed_models, grouped);
    for duplicate in [
        "capacitor",
        "capacitoric",
        "inductor",
        "inductoric",
        "memristor",
        "zener",
        "tline",
        "cpline",
        "hyst",
        "s_h",
    ] {
        let descriptor = engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.model_type == duplicate)
            .expect("the duplicate is still a valid code model");
        assert_eq!(
            descriptor.category, None,
            "{duplicate} duplicates a native part and stays off the shelf"
        );
    }
}

/// A listed code model never answers to a functional part's name: the shelf
/// offers that part once. The event-driven digital models are the exception,
/// as the event-driven twins of the analog-threshold gates and counter.
#[test]
fn no_listed_code_model_answers_to_a_functional_parts_name() {
    let rows = ComponentCategory::ALL
        .into_iter()
        .flat_map(palette_rows)
        .collect::<Vec<_>>();
    let functional = rows
        .iter()
        .filter_map(|row| match row {
            PaletteRow::Functional(descriptor) => Some(descriptor.display_name),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert!(!functional.is_empty());
    for row in &rows {
        let PaletteRow::Xspice(model) = row else {
            continue;
        };
        if model.category == Some(ComponentCategory::DigitalEventDriven) {
            continue;
        }
        for name in &functional {
            assert!(
                !model.display_name.eq_ignore_ascii_case(name)
                    && !model
                        .keywords
                        .iter()
                        .any(|keyword| keyword.eq_ignore_ascii_case(name)),
                "the {} code model answers to the {name} part's name",
                model.display_name
            );
        }
    }
}

#[test]
fn shelf_search_matches_labels_case_insensitively() {
    assert!(matches_query("nmos", &["NMOS", "Semiconductors"]));
    assert!(!matches_query("nmos", &["Resistor", "Passives"]));
}

/// A library of pinned bytes the project retained: one macromodel, one card a
/// device family draws, one card no family does, and one section-scoped
/// subcircuit key a netlist cannot reference by name.
pub(crate) fn retained_model_library(name: &str) -> ModelLibrary {
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

    let rows = shelf_rows(
        &index,
        &libraries,
        &super::super::shelf::search::ShelfQuery::new(""),
    );
    let row = rows
        .iter()
        .find(|row| row.part_id == "RSPICE_PROBE_NPN")
        .expect("the foundation card is on the shelf");
    // A placeable foundation card is the normal case, so the row states what
    // the part is and leaves where it came from to the Models workspace.
    assert_eq!(row.meta, "bjt-npn", "{}", row.meta);
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

    let rows = shelf_rows(
        &index,
        &libraries,
        &super::super::shelf::search::ShelfQuery::new(""),
    );
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

    let rows = shelf_rows(
        &index,
        &[],
        &super::super::shelf::search::ShelfQuery::new(""),
    );
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
    glyphs.extend(ComponentCategory::ALL.map(category_glyph));
    glyphs.extend(
        supply_palette()
            .iter()
            .map(|entry| ShelfGlyph::Supply(entry.glyph())),
    );
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

/// The shelf's meta column states the default the placed instance will
/// carry, spelled through the crate's engineering formatter, and nothing
/// else: the designator prefix it used to lead with is what the glyph column
/// beside it paints, from the same `spice_prefix`.
#[test]
fn shelf_meta_states_the_placed_default() {
    for (kind, meta) in [
        (ComponentType::Resistor, Some("1k")),
        (ComponentType::Capacitor, Some("1u")),
        (ComponentType::Inductor, Some("1m")),
        (ComponentType::VoltageSource, Some("5")),
        (ComponentType::OpAmp, Some("100k")),
        // The one default not authored in the formatter's spelling: the
        // formatter's decade wins, as it does in the property editor.
        (ComponentType::CoupledInductor, Some("990m")),
        // A default no engineering parser reads is stated as authored.
        (ComponentType::BehavioralSource, Some("V=0")),
        // Modelled devices have no meaningful default value.
        (ComponentType::Diode, None),
        (ComponentType::Nmos, None),
        (ComponentType::Memristor, None),
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

// -------------------------------------------------- shelf pinning and recents

/// The Component shelf under test: the application it paints from and the
/// context it paints into, held together so that one frame is one call.
///
/// The pair is owned rather than passed because a helper taking `&mut
/// RSpiceApp` hands a test the whole application, and `tests/module_layering`
/// counts every signature that does.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) struct ShelfHarness {
    pub(crate) ctx: egui::Context,
    pub(crate) app: RSpiceApp,
}

#[cfg(not(target_arch = "wasm32"))]
impl ShelfHarness {
    /// A themed shelf that has painted nothing yet.
    pub(crate) fn opened() -> Self {
        let ctx = egui::Context::default();
        crate::ui::Theme::default().apply(&ctx);
        Self {
            ctx,
            app: RSpiceApp::test_instance(),
        }
    }

    /// The shelf held open across frames, driven to a settled layout.
    ///
    /// Two frames: the first builds the font set, the second lays out against
    /// it, and only then are the painted rects worth clicking.
    pub(crate) fn settled(mut self) -> Self {
        let _ = self.output(Vec::new());
        let _ = self.output(Vec::new());
        self
    }

    /// Publishing an accessibility tree, so a frame reports what the keyboard
    /// is on. Switched on before the shelf settles, because a context starts
    /// reporting from the frame after it is told to.
    pub(crate) fn reporting_accessibility(self) -> Self {
        self.ctx.enable_accesskit();
        self
    }

    /// One frame of the whole Component shelf, through the same entry point
    /// the dock calls — so the placement watch, the bands, and the catalog all
    /// see one frame, as they do in the product.
    pub(crate) fn output(&mut self, events: Vec<egui::Event>) -> egui::FullOutput {
        let Self { ctx, app } = self;
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
                        component_shelf(ui, &mut app.design_cx());
                    });
            },
        )
    }

    /// What one such frame painted: the joined text and the runs it came from.
    ///
    /// Both, because a gesture has to land on a row, and only the runs carry
    /// where the rows are.
    pub(crate) fn frame(
        &mut self,
        events: Vec<egui::Event>,
    ) -> (String, Vec<(String, egui::Rect, egui::Color32)>) {
        let output = self.output(events);
        (painted_text(&output), painted_runs(&output))
    }
}

/// Every shelf-entry key survives the round trip through the durable form,
/// including names that carry the characters a naive encoding would split on.
#[test]
fn every_shelf_entry_kind_round_trips_through_its_stored_key() {
    for entry in [
        ShelfEntry::Primitive(ComponentType::Resistor),
        ShelfEntry::Primitive(ComponentType::Ground),
        ShelfEntry::Variant("Zener Diode".to_owned()),
        ShelfEntry::LibraryPart("1N4148".to_owned()),
        ShelfEntry::BuiltinXspice("gain".to_owned()),
        ShelfEntry::GeneratedVerilogA("bsimcmg".to_owned()),
        ShelfEntry::Supply("VDD".to_owned()),
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

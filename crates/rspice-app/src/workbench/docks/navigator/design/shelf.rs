//! The Component shelf: the design panel's other tab, where a part is picked.
//!
//! A separate surface from the navigator beside it, and separated here for the
//! reason the two tabs exist at all: the navigator answers about the design in
//! front of the reader, and this answers about everything that could be added
//! to it. They share no matcher either: the navigator filters its tree by
//! substring, and the shelf ranks every placeable row by word ([`search`]).
//!
//! Almost nothing here takes the whole application. A shelf row arms a tool,
//! writes a pin, or credits a placement, and all three live in
//! [`AppState`] — so the handlers take that rather than the application that
//! holds it. The five that do not are the ones that route somewhere else
//! entirely: a part the project has not adopted raises the pack confirmation
//! in the Models workspace, and a pin placement runs the `PlacePin` command.
//! Those go through the command vocabulary, which is defined over the whole
//! application.

pub(super) mod search;

use std::collections::{BTreeMap, HashSet};

use egui::{Response, ScrollArea, Ui};
use rspice_design::schematic::device_catalog::CatalogXspiceDeviceDescriptor;

use crate::schematic::view::SchematicShelfDragPayload;
use crate::schematic::{
    ComponentCategory, ComponentPaletteEntry, PaletteRow, SupplyPaletteEntry, component_palette,
    device_class_keywords, palette_rows, supply_palette,
};
use crate::state::model_hub::{
    ModelHubPartRow, PartPlacement, PartProvenance, PartState, plan_library_placement,
    refusal_sentence,
};
use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};
use crate::state::{
    ComponentType, FunctionalPartDescriptor, LibraryCellInstance, LibraryCellPlacementCandidate,
    Tool, builtin_xspice_library_binding, builtin_xspice_vector_ports, engine_only_xspice_devices,
    functional_library_binding, functional_part, generated_veriloga_devices,
    generated_veriloga_library_binding, library_cell_placement_candidates,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;
use crate::workbench::framework::requests::{ConfirmPackAttach, FocusPlacementSearch};
use rspice_app_models::veriloga_library::{
    BUILTIN_VERILOGA_LIBRARY, BUILTIN_VERILOGA_MODELS, builtin_veriloga_model,
};

use super::super::super::super::design_system::{
    WorkbenchIcon, section_header as shelf_section_header,
};
use super::super::rail::{self, RailDisclosure, RailFold};
use super::super::{
    PanelSearchKeys, SCHEMATIC_NAV_LABEL_SIZE, SCHEMATIC_NAV_META_SIZE, SCHEMATIC_NAV_ROW_HEIGHT,
    empty_navigator_row, panel_search_keys, schematic_nav_row_indented_drag_response,
    schematic_nav_row_indented_response, search_focus_requested,
};
use search::{MatchRank, ShelfQuery, ShelfTerms};

/// The family glyph a functional group's row carries.
///
/// The non-Latin marks are painted as vector geometry because the bundled
/// faces do not hold them (see [`ShelfGlyph`]).
pub(super) fn category_glyph(category: ComponentCategory) -> ShelfGlyph {
    match category {
        ComponentCategory::HierarchyConnectivity => ShelfGlyph::Icon(WorkbenchIcon::Pin),
        ComponentCategory::Passives => ShelfGlyph::Text("\u{2014}"),
        ComponentCategory::Sources => ShelfGlyph::Source,
        ComponentCategory::Semiconductors => ShelfGlyph::Text("Q"),
        ComponentCategory::AmplifiersComparators => ShelfGlyph::Amp,
        ComponentCategory::ControlledSourcesSwitches => ShelfGlyph::Text("E"),
        ComponentCategory::LogicTiming => ShelfGlyph::Text("&"),
        ComponentCategory::DigitalEventDriven => ShelfGlyph::Event,
        ComponentCategory::BehavioralMath => ShelfGlyph::Text("B"),
        ComponentCategory::Magnetics => ShelfGlyph::Text("K"),
        ComponentCategory::LinesRf => ShelfGlyph::Text("T"),
        ComponentCategory::ProtectionElectromechanical => ShelfGlyph::Text("F"),
        ComponentCategory::ProbesMeters => ShelfGlyph::Icon(WorkbenchIcon::Probe),
    }
}

/// Passives is the one group open on first run, so a new install's shelf
/// leads with placeable rows instead of a column of folded groups. Every
/// position is the reader's own once moved: only the default behind the
/// persisted flag differs per group.
fn category_open_by_default(category: ComponentCategory) -> bool {
    category == ComponentCategory::Passives
}

/// The identity mark a Component-shelf row paints in its glyph column.
///
/// The rule: a placeable part's glyph is its SPICE card letter — the letter
/// its emitted element card starts with, straight from
/// [`ComponentType::spice_prefix`] — set in the mono face at the schematic
/// symbol tint, exactly the mockup's `PART_CATALOG` glyph column. Identities a
/// card letter cannot state take a mark instead: the op-amp triangle (its `E`
/// card would file it as a plain VCVS), ground and the interface pin
/// (structural objects with no card of their own), the event-driven ⊞ for
/// every A-card XSPICE row, and `VA` for generated Verilog-A models.
///
/// The bundled faces do not hold every candidate in the mockup vocabulary
/// (◯ ▷ ⊳ among them), and a character they lack rasterizes as a tofu box,
/// which is why the family marks are vector geometry rather than text.
/// `every_shelf_glyph_paints_ink_and_no_text_glyph_is_a_tofu_box` walks the
/// table and fails on any glyph the faces lack.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ShelfGlyph {
    /// A mono-face string: a SPICE card letter (`R`, `Q`, `M`), a family
    /// abbreviation (`MR`, `VA`), or the Passives em dash.
    Text(&'static str),
    /// A design-system icon: ground's supply bars, the interface pin, the
    /// Models mark on library rows.
    Icon(WorkbenchIcon),
    /// The source circle — the mockup's ◯.
    Source,
    /// The amplifier triangle — the mockup's ▷ / △.
    Amp,
    /// The event-driven / mixed-signal squared plus — the mockup's ⊞.
    Event,
    /// A supply rail's symbol, drawn from the strokes the canvas paints.
    Supply(rspice_design::schematic::net_label::SupplyGlyph),
}

impl ShelfGlyph {
    /// Paints the glyph centred in `rect`.
    ///
    /// Vector marks share the design-system icon idiom: a 24-unit design
    /// space scaled to the slot, strokes floored at one pixel.
    pub(super) fn paint(self, painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
        let side = rect.width().min(rect.height());
        let scale = side / 24.0;
        let stroke = egui::Stroke::new((1.6 * scale).max(1.0), color);
        match self {
            Self::Text(text) => {
                painter.text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    text,
                    theme::mono(tokens::FS_1, FontWeight::Medium),
                    color,
                );
            }
            Self::Icon(icon) => icon.paint(painter, rect, color),
            Self::Source => {
                painter.circle_stroke(rect.center(), 7.5 * scale, stroke);
            }
            Self::Amp => {
                painter.add(egui::Shape::closed_line(
                    vec![
                        rect.center() + egui::vec2(-6.0 * scale, -7.0 * scale),
                        rect.center() + egui::vec2(7.0 * scale, 0.0),
                        rect.center() + egui::vec2(-6.0 * scale, 7.0 * scale),
                    ],
                    stroke,
                ));
            }
            Self::Event => {
                let half = 7.0 * scale;
                let cross = 3.5 * scale;
                painter.rect_stroke(
                    egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(2.0 * half)),
                    0.0,
                    stroke,
                    egui::StrokeKind::Inside,
                );
                painter.line_segment(
                    [
                        rect.center() - egui::vec2(cross, 0.0),
                        rect.center() + egui::vec2(cross, 0.0),
                    ],
                    stroke,
                );
                painter.line_segment(
                    [
                        rect.center() - egui::vec2(0.0, cross),
                        rect.center() + egui::vec2(0.0, cross),
                    ],
                    stroke,
                );
            }
            Self::Supply(glyph) => {
                let label =
                    crate::state::NetLabel::supply(0, crate::state::Point::new(0, 0), "", glyph);
                let Some(bounds) =
                    rspice_design::schematic::label_layout::supply_glyph_rect(&label)
                else {
                    return;
                };
                // The world strokes fill most of the slot's width, centred.
                let fit = 16.0 * scale / (bounds.max.x - bounds.min.x).max(1.0);
                let middle = egui::pos2(
                    (bounds.min.x + bounds.max.x) * 0.5,
                    (bounds.min.y + bounds.max.y) * 0.5,
                );
                let at = |point: rspice_design::schematic::label_layout::LayoutPoint| {
                    rect.center() + (egui::pos2(point.x, point.y) - middle) * fit
                };
                for [from, to] in rspice_design::schematic::label_layout::supply_glyph_marks(&label)
                {
                    painter.line_segment([at(from), at(to)], stroke);
                }
            }
        }
    }
}

/// The shelf's meta column for one placeable primitive: the value the placed
/// instance opens carrying, e.g. `1k`.
///
/// It comes from [`ComponentType::default_value`], the same single source
/// [`crate::state`]'s placement writes into a fresh component, and is
/// re-presented through the crate's engineering formatter the way the
/// property editor re-presents an untouched draft, so the shelf and the
/// editor spell one decade the same way. A default the formatter cannot read
/// (`V=0`) is stated as authored; a structural row with no meaningful default
/// says nothing.
///
/// The line used to open with the card letter, which is exactly what the
/// glyph column beside it paints from the same [`ComponentType::
/// spice_prefix`].
pub(super) fn primitive_shelf_meta(kind: ComponentType) -> Option<String> {
    let default = kind.default_value();
    if default.is_empty() {
        return None;
    }
    Some(
        crate::quantity::parse_engineering_value(default)
            .map(crate::quantity::format_engineering_value)
            .unwrap_or_else(|_| default.to_owned()),
    )
}

/// The glyph for one placeable primitive.
///
/// Card letters come from [`ComponentType::spice_prefix`], so this column can
/// never drift from the designator the meta column states and the netlist
/// emits. The exceptions are the identities a card letter cannot carry — see
/// [`ShelfGlyph`] for the rule.
pub(super) fn primitive_shelf_glyph(kind: ComponentType) -> ShelfGlyph {
    match kind {
        ComponentType::Ground => ShelfGlyph::Icon(WorkbenchIcon::Supply),
        ComponentType::Port => ShelfGlyph::Icon(WorkbenchIcon::Pin),
        ComponentType::OpAmp => ShelfGlyph::Amp,
        kind if kind.spice_prefix() == "A" => ShelfGlyph::Event,
        kind => ShelfGlyph::Text(kind.spice_prefix()),
    }
}
fn nav_row_indented(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> bool {
    nav_row_indented_response(ui, icon, label, selected, meta, level).clicked()
}

fn nav_row_indented_response(
    ui: &mut Ui,
    icon: WorkbenchIcon,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> Response {
    schematic_nav_row_indented_response(ui, icon, label, selected, meta, level, false, false, false)
}

pub(super) fn component_shelf(ui: &mut Ui, app: &mut RSpiceApp) {
    observe_placements(ui, &mut app.state);
    let keys = shelf_search(ui, &mut app.state);
    rail::open(ui.ctx());
    let query = ShelfQuery::new(&app.state.design.placement_query);
    let library_parts = library_part_rows(app, &query);
    let cells = cell_candidates(&app.state);
    if !query.is_empty() {
        let hits = shelf_hits(&app.state, &query, &library_parts, &cells);
        let state = &mut app.state;
        let clicked = ScrollArea::vertical()
            .id_salt("workbench.design.component_shelf")
            .auto_shrink([false, true])
            .show(ui, |ui| {
                let clicked = shelf_results(ui, state, &hits);
                let _unmoved = rail::traverse(ui, keys.rows);
                clicked
            })
            .inner;
        // Enter in the field places the best answer, as a click on it would.
        let submitted = keys
            .submitted
            .then(|| top_hit(&hits))
            .flatten()
            .map(|hit| hit.arm.clone());
        if let Some(arm) = clicked.or(submitted) {
            apply_shelf_arm(app, arm, ui.ctx());
        }
        return;
    }
    let mut band = None;
    let mut catalog = None;
    let mut stimulus = None;
    let mut generated = None;
    let mut cell = None;
    let mut requested_part = None;
    // Every band and catalog below takes the session rather than the
    // application, so the borrow is narrowed once here instead of at each of
    // the seven calls. Only the arming below needs the application back.
    let state = &mut app.state;
    ScrollArea::vertical()
        .id_salt("workbench.design.component_shelf")
        .auto_shrink([false, true])
        .show(ui, |ui| {
            let pinned = pinned_band(ui, state, &library_parts, &cells);
            band = recent_band(ui, state, &library_parts, &cells).or(pinned);
            catalog = component_catalog(ui, state);
            stimulus = stimulus_library_section(ui, state);
            generated = generated_veriloga_catalog(ui, state);
            requested_part = library_parts_section(ui, state, &library_parts);
            cell = project_library(ui, state, &cells);
            // Every fold position on the shelf is persisted under its own
            // group row, so the traversal moves all of them itself and has
            // nothing to hand back. Bound rather than discarded, so a row
            // added later that folds a navigator-tree node is caught here
            // instead of quietly answering no key.
            let _unmoved = rail::traverse(ui, keys.rows);
            debug_assert!(
                _unmoved.is_none(),
                "the shelf keeps every fold position under its own row"
            );
        });
    if let Some(arm) = band.or(catalog) {
        apply_shelf_arm(app, arm, ui.ctx());
    } else if let Some(name) = stimulus {
        arm_stimulus_definition(&mut app.state, &name, ui.ctx());
    } else if let Some(binding) = generated {
        arm_cell(&mut app.state, binding, ui.ctx());
    } else if let Some(binding) = cell {
        arm_cell(&mut app.state, binding, ui.ctx());
    } else if let Some(row) = requested_part {
        apply_library_part_row(app, row, ui.ctx());
    }
}

/// Complete one click on a library-part row.
fn apply_library_part_row(app: &mut RSpiceApp, row: LibraryPartRow, ctx: &egui::Context) {
    match row.action {
        LibraryPartAction::Arm(placement) => arm_library_part(&mut app.state, *placement, ctx),
        LibraryPartAction::Review {
            pack_id, version, ..
        } => request_library_part(app, row.part_id, pack_id, version),
        // A refused row renders disabled, so its click never arrives.
        LibraryPartAction::Refused(_) => {}
    }
}

/// One indexed part the shelf can offer, with what a click would do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct LibraryPartRow {
    pub(super) part_id: String,
    /// The part's device class, in the unified index's vocabulary.
    pub(super) device: String,
    /// The part's address: its pack, or the project library holding it.
    pub(super) source: String,
    /// The meta column: the part's device class, and where it stands.
    pub(super) meta: String,
    pub(super) action: LibraryPartAction,
}

/// Where one library-part click goes.
///
/// The same fork the Models workspace shelf decides in its `place` module: a
/// definition the project already holds is armed directly, a release it has
/// not adopted is reviewed first, and a part that cannot be drawn is refused
/// with the sentence the disabled row carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum LibraryPartAction {
    /// Arm now: the definition is already in this project's catalog —
    /// compiled in, or retained when the part was added.
    Arm(Box<PartPlacement>),
    /// Raise the pack confirmation in the Models workspace. Only right for a
    /// release that still needs installing or retaining; the hub operation
    /// arms the cursor when the retention lands.
    Review {
        pack_id: String,
        version: String,
        pack_name: String,
    },
    /// No placement exists for this part; the sentence the disabled control
    /// carries.
    Refused(String),
}

/// Every part the unified index can offer this shelf.
///
/// A part the project retained — and a compiled-in foundation part — is
/// listed here and arms directly. The "Project library" section below reads
/// the symbol-cell library, not the model-library manager, so this section is
/// the only door such a definition has onto the canvas. Pack releases the
/// project has not adopted sit beside them and go through the pack
/// confirmation instead.
pub(super) fn library_part_rows(app: &RSpiceApp, query: &ShelfQuery) -> Vec<LibraryPartRow> {
    let libraries = app.state.model.model_library_manager.libraries_sorted();
    let index = app.model_hub.part_index(&libraries);
    shelf_rows(&index, &libraries, query)
}

/// Decides each indexed row's door, dropping the rows that have none.
///
/// Separated from [`library_part_rows`] so a test can feed it an index
/// without opening a hub over a store.
pub(super) fn shelf_rows(
    index: &[ModelHubPartRow],
    libraries: &[&ModelLibrary],
    query: &ShelfQuery,
) -> Vec<LibraryPartRow> {
    /// Rows the shelf lists before asking the user to narrow the search.
    ///
    /// A published catalog is unbounded; a navigator column is not. The cap is
    /// a rendering decision, and the footer says when it bit.
    const SHELF_ROWS: usize = 200;

    // A part the project adopted is the retained row's to place: its
    // installed-pack row is dropped rather than offering to review and add
    // something that is already added — the same rule the Models workspace
    // shelf applies to its own two halves.
    let retained = index
        .iter()
        .filter(|row| matches!(row.provenance, PartProvenance::ProjectRetained { .. }))
        .map(|row| row.part_id.as_str())
        .collect::<HashSet<_>>();
    index
        .iter()
        .filter(|row| match &row.provenance {
            PartProvenance::InstalledPack { .. } => !retained.contains(row.part_id.as_str()),
            // A section-scoped subcircuit key cannot be referenced by its
            // bare name, so it is not a part a reader picks.
            PartProvenance::Foundation | PartProvenance::ProjectRetained { .. } => {
                !row.part_id.contains('\u{1f}')
            }
            PartProvenance::RemoteRelease { .. } => true,
        })
        .filter(|row| {
            library_part_rank(
                query,
                &row.part_id,
                &row.device,
                part_source(&row.provenance),
            )
            .is_some()
        })
        .take(SHELF_ROWS)
        .filter_map(|row| library_part_row(libraries, row))
        .collect()
}

/// The address a part row is searched by besides its number: the pack for a
/// pack row, the holding library for a project one — the same column the
/// Models workspace shelf searches.
fn part_source(provenance: &PartProvenance) -> &str {
    match provenance {
        PartProvenance::InstalledPack { pack_id, .. }
        | PartProvenance::RemoteRelease { pack_id, .. } => pack_id.as_str(),
        PartProvenance::ProjectRetained { library } => library.as_str(),
        PartProvenance::Foundation => "",
    }
}

/// How one library part answers a query. Its number and address are part
/// numbers and its device class supplies the words; it has no label beyond
/// its number.
fn library_part_rank(
    query: &ShelfQuery,
    part_id: &str,
    device: &str,
    source: &str,
) -> Option<MatchRank> {
    query.rank(&ShelfTerms {
        label: "",
        keywords: device_class_keywords(device),
        part_numbers: &[part_id, source],
    })
}

/// One shelf row, with the route its click takes.
fn library_part_row(libraries: &[&ModelLibrary], row: &ModelHubPartRow) -> Option<LibraryPartRow> {
    let (meta, action) = match &row.provenance {
        PartProvenance::Foundation | PartProvenance::ProjectRetained { .. } => {
            let holder = holding_library(libraries, &row.provenance, &row.part_id)?;
            let action = match plan_library_placement(holder, &row.part_id) {
                Ok(placement) => LibraryPartAction::Arm(Box::new(placement)),
                Err(reason) => LibraryPartAction::Refused(refusal_sentence(reason)),
            };
            // A part that is here and placeable is the normal case; where
            // it came from is the Models workspace's business.
            (row.device.clone(), action)
        }
        PartProvenance::InstalledPack { pack_id, version }
        | PartProvenance::RemoteRelease { pack_id, version } => {
            let pack_name = row.pack_name.clone().unwrap_or_else(|| pack_id.clone());
            let meta = match &row.state {
                PartState::Installed => row.device.clone(),
                PartState::Available => format!("{} · available", row.device),
                PartState::UpdateAvailable { latest, .. } => {
                    format!("{} · update {latest}", row.device)
                }
                PartState::Incompatible { missing } => {
                    format!("{} · needs {}", row.device, missing.join(", "))
                }
            };
            let action = match &row.state {
                // The row stays searchable and stays readable; only the
                // action is refused, and the refusal says why in the same
                // words the pack manifest used.
                PartState::Incompatible { missing } => LibraryPartAction::Refused(format!(
                    "This build of RSpice does not offer {}, which {} requires.",
                    missing.join(", "),
                    pack_name
                )),
                _ => LibraryPartAction::Review {
                    pack_id: pack_id.clone(),
                    version: version.clone(),
                    pack_name,
                },
            };
            (meta, action)
        }
    };
    Some(LibraryPartRow {
        part_id: row.part_id.clone(),
        device: row.device.clone(),
        source: part_source(&row.provenance).to_owned(),
        meta,
        action,
    })
}

/// The loaded library whose definition a foundation or retained row names.
///
/// Both come straight out of the same library set the index was built from in
/// this frame, so a miss means the row and the set disagree — the row is
/// dropped rather than offered against bytes that are not there.
fn holding_library<'a>(
    libraries: &[&'a ModelLibrary],
    provenance: &PartProvenance,
    part: &str,
) -> Option<&'a ModelLibrary> {
    libraries.iter().copied().find(|library| match provenance {
        PartProvenance::ProjectRetained { library: name } => library.name == *name,
        PartProvenance::Foundation => {
            matches!(library.source_authority, ModelSourceAuthority::BuiltIn)
                && (library.models.contains_key(part)
                    || library.top_level_models.contains_key(part)
                    || library.subcircuits.contains_key(part))
        }
        PartProvenance::InstalledPack { .. } | PartProvenance::RemoteRelease { .. } => false,
    })
}

/// The shelf section for the parts the unified model index lists.
fn library_parts_section(
    ui: &mut Ui,
    state: &mut AppState,
    rows: &[LibraryPartRow],
) -> Option<LibraryPartRow> {
    if rows.is_empty() {
        return None;
    }
    if !catalog_group_row(
        ui,
        "component-shelf-library-parts",
        ShelfGlyph::Icon(WorkbenchIcon::Models),
        "Library parts",
        rows.len(),
        false,
    ) {
        return None;
    }

    let mut requested = None;
    for row in rows {
        let placeable = !matches!(row.action, LibraryPartAction::Refused(_));
        let offered = ui
            .add_enabled_ui(placeable, |ui| {
                let response = nav_row_indented_response(
                    ui,
                    WorkbenchIcon::Models,
                    &row.part_id,
                    false,
                    Some(&row.meta),
                    2,
                );
                match &row.action {
                    // The row stays searchable and stays readable; only the
                    // action is refused, and the refusal says why.
                    LibraryPartAction::Refused(reason) => {
                        response.on_disabled_hover_text(reason.as_str());
                        None
                    }
                    LibraryPartAction::Arm(_) => {
                        Some(response.on_hover_text(format!("Click to arm {}", row.part_id)))
                    }
                    LibraryPartAction::Review {
                        pack_name, version, ..
                    } => Some(response.on_hover_text(format!(
                        "Review and add {} from {} {}",
                        row.part_id, pack_name, version
                    ))),
                }
            })
            .inner;
        // A refused part has no placement to pin, so it carries no pin menu
        // either: the rail must not fill with doors that lead nowhere.
        let Some(response) = offered else {
            continue;
        };
        if response.clicked() {
            requested = Some(row.clone());
        }
        shelf_pin_context_menu(
            &response,
            state,
            &ShelfEntry::LibraryPart(row.part_id.clone()),
        );
    }
    requested
}

/// Arms the cursor with a definition the project already holds.
///
/// The same completion the Models workspace shelf performs when it arms a
/// held part, minus that workspace's operation receipt: this click happened
/// on the canvas side, so the armed cursor is the whole story.
fn arm_library_part(
    state: &mut crate::workbench::app_state::AppState,
    placement: PartPlacement,
    ctx: &egui::Context,
) {
    let _ = crate::workbench::app::arm_pack_part(&mut state.model, placement);
    finish_shelf_placement(state, ctx);
}

/// Raises the pack confirmation for one shelf part.
///
/// The decision is shown in the Models workspace rather than over the canvas:
/// it commits the project to a licence, a download, and a capability claim,
/// and those are exactly what that workspace is for. The placement is armed
/// on the cursor when the install completes, so the round trip ends where the
/// user started.
fn request_library_part(app: &mut RSpiceApp, part_id: String, pack_id: String, version: String) {
    app.state.workbench.outbox.push(ConfirmPackAttach {
        part_id,
        pack_id,
        version,
    });
}

fn shelf_search(ui: &mut Ui, state: &mut AppState) -> PanelSearchKeys {
    let mut focus = search_focus_requested::<FocusPlacementSearch>(ui, &mut state.workbench.outbox);
    panel_search_keys(
        ui,
        &mut state.design.placement_query,
        "workbench.design.component_shelf.search",
        "Place component or cell…",
        &mut focus,
    )
}

/// The Pinned band a profile that has never pinned anything sees.
///
/// It is a shipped default and nothing more: the first pin or unpin
/// materializes this list into the reader's own set, after which the shipped
/// list never merges back in. Emptying the set leaves the band absent rather
/// than restoring these three.
const DEFAULT_PINNED: [ComponentType; 3] = [
    ComponentType::Resistor,
    ComponentType::Capacitor,
    ComponentType::Ground,
];

/// Recent rows painted at once.
///
/// Six is the working set of one placement session — two or three passives, a
/// source, ground, and the active device under study — and at the 24 px row
/// contract it costs 144 px, which keeps the Pinned and Recent bands together
/// under a quarter of a full-height dock and leaves the catalog's first band
/// on screen without scrolling. A longer history is not more useful here: the
/// catalog underneath is already searchable.
pub(super) const RECENT_SHOWN: usize = 6;

/// Recent entries kept.
///
/// More than are painted, because a pinned entry is filtered out of the band
/// rather than dropped from the history: unpinning must restore a full band,
/// not a short one.
pub(super) const RECENT_STORED: usize = 16;

/// Field separator inside one stored shelf-entry key.
///
/// ASCII unit separator: the same mark the model index uses inside a
/// section-scoped part key, and one no library, cell, view, or model name may
/// carry — so a key round-trips whatever the names hold.
const SHELF_ENTRY_FIELD: char = '\u{1f}';

/// One placeable identity the Component shelf can pin, or list as recently
/// placed.
///
/// The variants are the shelf's row families. Each is stored as an opaque
/// stable key ([`ShelfEntry::storage_key`]) rather than as a resolved row, so
/// a personal pin outlives the session, the project, and — for a part whose
/// library is currently detached — this build's ability to draw it at all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum ShelfEntry {
    Primitive(ComponentType),
    /// A palette row that places a primitive with a symbol skin and model,
    /// by its label.
    Variant(String),
    /// A model-library part, by the id the unified index publishes.
    LibraryPart(String),
    /// A built-in XSPICE code model, by its registry stable id.
    BuiltinXspice(String),
    /// A functional part, by its catalog stable id.
    Functional(String),
    /// A build-time generated Verilog-A model, by its model name.
    GeneratedVerilogA(String),
    /// A project-library cell view.
    Cell {
        library: String,
        cell: String,
        view: String,
    },
    /// A supply rail, by its net name.
    Supply(String),
}

impl ShelfEntry {
    /// The shelf identity of one bound library-cell instance.
    ///
    /// The executable bindings are checked first: an XSPICE or generated
    /// Verilog-A placement also carries a library/cell pair, but that pair
    /// names a synthesized master, not a row a reader can find again.
    fn from_binding(binding: &LibraryCellInstance) -> Self {
        if let Some(xspice) = binding.builtin_xspice.as_ref() {
            return Self::BuiltinXspice(xspice.stable_id.clone());
        }
        if let Some(functional) = binding.builtin_functional.as_ref() {
            return Self::Functional(functional.stable_id.clone());
        }
        if let Some(veriloga) = binding.generated_veriloga.as_ref() {
            return Self::GeneratedVerilogA(veriloga.model_name.clone());
        }
        Self::Cell {
            library: binding.library.clone(),
            cell: binding.cell.clone(),
            view: binding.view.clone(),
        }
    }

    /// The durable key this entry is stored under.
    ///
    /// A primitive is spelled with its own serde name — the same spelling
    /// every saved project already uses for that type — so the pin set and
    /// the document agree on one vocabulary.
    pub(super) fn storage_key(&self) -> String {
        match self {
            Self::Primitive(kind) => {
                let name = match serde_json::to_value(kind) {
                    Ok(serde_json::Value::String(name)) => name,
                    _ => return String::new(),
                };
                format!("primitive{SHELF_ENTRY_FIELD}{name}")
            }
            Self::Variant(label) => format!("variant{SHELF_ENTRY_FIELD}{label}"),
            Self::LibraryPart(part) => format!("part{SHELF_ENTRY_FIELD}{part}"),
            Self::BuiltinXspice(id) => format!("xspice{SHELF_ENTRY_FIELD}{id}"),
            Self::Functional(id) => format!("functional{SHELF_ENTRY_FIELD}{id}"),
            Self::GeneratedVerilogA(model) => format!("veriloga{SHELF_ENTRY_FIELD}{model}"),
            Self::Supply(name) => format!("supply{SHELF_ENTRY_FIELD}{name}"),
            Self::Cell {
                library,
                cell,
                view,
            } => format!(
                "cell{SHELF_ENTRY_FIELD}{library}{SHELF_ENTRY_FIELD}{cell}{SHELF_ENTRY_FIELD}{view}"
            ),
        }
    }

    /// The entry one stored key names, or `None` when this build does not
    /// understand the key. The stored list keeps it either way.
    pub(super) fn from_storage_key(key: &str) -> Option<Self> {
        let mut fields = key.split(SHELF_ENTRY_FIELD);
        let entry = match fields.next()? {
            "primitive" => Self::Primitive(
                serde_json::from_value(serde_json::Value::String(fields.next()?.to_owned()))
                    .ok()?,
            ),
            "variant" => Self::Variant(fields.next()?.to_owned()),
            "part" => Self::LibraryPart(fields.next()?.to_owned()),
            "xspice" => Self::BuiltinXspice(fields.next()?.to_owned()),
            "functional" => Self::Functional(fields.next()?.to_owned()),
            "veriloga" => Self::GeneratedVerilogA(fields.next()?.to_owned()),
            "supply" => Self::Supply(fields.next()?.to_owned()),
            "cell" => Self::Cell {
                library: fields.next()?.to_owned(),
                cell: fields.next()?.to_owned(),
                view: fields.next()?.to_owned(),
            },
            _ => return None,
        };
        fields.next().is_none().then_some(entry)
    }
}

/// The pin set's stored keys, with the shipped default standing in for a
/// profile that has never pinned anything.
pub(super) fn pinned_keys(state: &AppState) -> Vec<String> {
    state
        .model
        .preferences
        .component_shelf()
        .pinned
        .unwrap_or_else(|| {
            DEFAULT_PINNED
                .iter()
                .copied()
                .map(|kind| ShelfEntry::Primitive(kind).storage_key())
                .collect()
        })
}

pub(super) fn is_pinned(state: &AppState, entry: &ShelfEntry) -> bool {
    let key = entry.storage_key();
    pinned_keys(state).contains(&key)
}

/// Pin an unpinned entry, or unpin a pinned one.
///
/// A pin is appended rather than inserted at the front: the band is a rail the
/// reader builds, and one that reordered itself on every pin would defeat the
/// muscle memory that is the whole reason to have it. The first call also
/// materializes the shipped default into the reader's own set, which is what
/// makes that default stop applying.
pub(super) fn toggle_pin(state: &mut AppState, entry: &ShelfEntry) {
    let key = entry.storage_key();
    let mut keys = pinned_keys(state);
    if let Some(at) = keys.iter().position(|held| *held == key) {
        keys.remove(at);
    } else {
        keys.push(key);
    }
    let mut shelf = state.model.preferences.component_shelf();
    shelf.pinned = Some(keys);
    state.model.preferences.set_component_shelf(shelf);
}

/// Record one placement at the front of the history, deduplicated.
pub(super) fn record_placement(state: &mut AppState, entry: &ShelfEntry) {
    let key = entry.storage_key();
    let mut shelf = state.model.preferences.component_shelf();
    shelf.recent.retain(|held| *held != key);
    shelf.recent.insert(0, key);
    shelf.recent.truncate(RECENT_STORED);
    state.model.preferences.set_component_shelf(shelf);
}

/// What the shelf last offered the canvas, and how much of the design was
/// standing when it looked.
///
/// The shelf hands the canvas an identity — an armed tool or a live drag — and
/// the canvas commits the placement somewhere the shelf does not see. Watching
/// the design grow while one identity is on offer is what connects the two.
#[derive(Clone)]
struct ShelfPlacementWatch {
    /// The design these counts belong to. A different one is adopted in
    /// silence: its objects were placed before this shelf ever looked.
    authority: (u64, u64, String),
    components: usize,
    /// What the shelf had on offer when it last looked.
    offered: Option<ShelfEntry>,
    /// Whether that offer has been used up — by the placement it explained, or
    /// by outliving the frame it was live in. See [`observe_placements`].
    spent: bool,
}

/// The identity the schematic currently has armed, as a shelf row names it.
fn armed_shelf_entry(state: &AppState) -> Option<ShelfEntry> {
    if let Some(entry) = armed_supply(state) {
        return Some(ShelfEntry::Supply(entry.name.to_owned()));
    }
    let Tool::Place(kind) = state.model.schematic.session.editor.tool else {
        return None;
    };
    if kind == ComponentType::CellInstance {
        return state
            .model
            .schematic
            .session
            .editor
            .pending_library_cell
            .as_ref()
            .map(|pending| &pending.binding)
            .map(ShelfEntry::from_binding);
    }
    // A native device armed from the model library carries that part's card,
    // and the card name is the part's own id in the unified index — so the
    // history names the part the reader picked, not its device family.
    if let Some(armed) = state
        .model
        .schematic
        .session
        .editor
        .pending_part_model
        .as_ref()
        && armed.tool == Tool::Place(kind)
    {
        if let Some(entry) = variant_entry(kind, armed.variant.as_deref(), &armed.model) {
            return Some(ShelfEntry::Variant(entry.label.to_owned()));
        }
        return Some(ShelfEntry::LibraryPart(armed.model.clone()));
    }
    // A stimulus definition is armed on the same tool as its family's
    // primitive, and it is not one: crediting the primitive would fill the
    // recent band with a source the reader never picked, and a key of its own
    // would name a definition that is project state rather than something this
    // build can offer. The Stimulus library section lists it either way.
    if state
        .model
        .schematic
        .session
        .editor
        .pending_stimulus
        .is_some()
    {
        return None;
    }
    Some(ShelfEntry::Primitive(kind))
}

/// The identity of a shelf row being dragged over the canvas right now.
fn dragged_shelf_entry(ctx: &egui::Context) -> Option<ShelfEntry> {
    let payload = egui::DragAndDrop::payload::<SchematicShelfDragPayload>(ctx)?;
    match payload.as_ref() {
        SchematicShelfDragPayload::Primitive(kind) => Some(ShelfEntry::Primitive(*kind)),
        SchematicShelfDragPayload::LibraryCell(binding) => Some(ShelfEntry::from_binding(binding)),
        // Not a pinnable identity — see [`armed_shelf_entry`].
        SchematicShelfDragPayload::Stimulus(_) => None,
    }
}

/// Credit the shelf identity on offer when the design grows.
///
/// Both placement routes end here: click-to-arm holds the tool across the
/// canvas click, and a drag holds the payload across every frame up to the
/// drop. Nothing the shelf did not offer is credited, so a paste, an import,
/// or a script never writes the reader's placement history.
///
/// An offer that has gone away is honoured for exactly one more frame and for
/// exactly one placement, because the act that ends the offer is the same act
/// that grows the design: the drop consumes the drag payload, the canvas click
/// retires the place tool, and the shelf sees the new object only afterwards.
/// Beyond that one frame the offer is spent — which is what keeps a paste made
/// a minute later from being read as another placement of the last part.
///
/// This runs while the Component shelf is on screen. A placement made with the
/// Navigator tab in front is credited on the reader's next visit, in the order
/// the design grew.
fn observe_placements(ui: &Ui, state: &mut AppState) {
    let id = egui::Id::new("workbench.design.component-shelf.placements");
    let authority = (
        state.model.design_execution_epoch,
        state.model.active_schematic_epoch,
        state.model.workspace.content.active_view.display_path(),
    );
    // A supply rail is a net label, so labels count as placements beside
    // components.
    let document = state.model.schematic.document();
    let components = document.components.len() + document.net_labels.len();
    let live = armed_shelf_entry(state).or_else(|| dragged_shelf_entry(ui.ctx()));
    // A watch belonging to another design is no watch at all: its objects were
    // placed before this shelf ever looked at them.
    let watched = ui
        .data(|data| data.get_temp::<ShelfPlacementWatch>(id))
        .filter(|watched| watched.authority == authority);
    let (offered, mut spent) = match (live, watched.as_ref()) {
        // A live offer stands on its own and is good for as many placements as
        // the reader makes while it lasts.
        (Some(entry), _) => (Some(entry), false),
        (None, Some(watched)) if !watched.spent => (watched.offered.clone(), true),
        (None, _) => (None, true),
    };
    if let Some(watched) = watched.as_ref()
        && components > watched.components
        && let Some(entry) = offered.as_ref()
    {
        record_placement(state, entry);
        spent = true;
    }
    ui.data_mut(|data| {
        data.insert_temp(
            id,
            ShelfPlacementWatch {
                authority,
                components,
                offered,
                spent,
            },
        );
    });
}

/// One pinned or recent entry, resolved against what this build and this
/// project can offer right now.
struct ShelfEntryRow {
    entry: ShelfEntry,
    glyph: ShelfGlyph,
    label: String,
    meta: Option<String>,
    selected: bool,
    arm: ShelfArm,
}

/// What clicking a resolved pinned or recent row does.
#[derive(Clone)]
pub(super) enum ShelfArm {
    Primitive(ComponentType),
    Variant(&'static ComponentPaletteEntry),
    /// Held by stable id rather than by binding: a vector-port code model
    /// opens the placement dialog instead of arming, and that fork belongs
    /// with the catalog row's, not duplicated here.
    BuiltinXspice(String),
    Cell(Box<LibraryCellInstance>),
    Part(Box<LibraryPartRow>),
    /// A project stimulus definition, by name.
    Stimulus(String),
    Supply(&'static SupplyPaletteEntry),
}

/// Resolve one stored entry into the row the band would paint.
///
/// `None` means the entry names nothing this session can place — a detached
/// library, an uninstalled pack, a code model this build does not carry. The
/// stored key is untouched: the band simply does not paint a door that leads
/// nowhere.
fn resolve_shelf_entry(
    state: &AppState,
    library_parts: &[LibraryPartRow],
    cells: &[CellCandidate],
    entry: &ShelfEntry,
) -> Option<ShelfEntryRow> {
    let placing_cell =
        state.model.schematic.session.editor.tool == Tool::Place(ComponentType::CellInstance);
    let pending = state
        .model
        .schematic
        .session
        .editor
        .pending_library_cell
        .as_ref()
        .map(|pending| &pending.binding);
    match entry {
        ShelfEntry::Primitive(kind) => Some(ShelfEntryRow {
            entry: entry.clone(),
            glyph: primitive_shelf_glyph(*kind),
            label: primitive_label(*kind).to_owned(),
            meta: primitive_shelf_meta(*kind),
            selected: primitive_armed(state, *kind),
            arm: ShelfArm::Primitive(*kind),
        }),
        ShelfEntry::Variant(label) => {
            let row = component_palette()
                .iter()
                .flat_map(|section| section.entries.iter())
                .find(|row| row.variant.is_some() && row.label == label)?;
            Some(ShelfEntryRow {
                entry: entry.clone(),
                glyph: primitive_shelf_glyph(row.kind),
                label: row.label.to_owned(),
                meta: variant_meta(row),
                selected: variant_armed(state, row),
                arm: ShelfArm::Variant(row),
            })
        }
        ShelfEntry::LibraryPart(part) => {
            let row = library_parts.iter().find(|row| row.part_id == *part)?;
            // A refused part has no placement to offer, so it is absent here
            // rather than pinned as a permanently disabled row.
            if matches!(row.action, LibraryPartAction::Refused(_)) {
                return None;
            }
            Some(ShelfEntryRow {
                entry: entry.clone(),
                glyph: ShelfGlyph::Icon(WorkbenchIcon::Models),
                label: row.part_id.clone(),
                meta: Some(row.meta.clone()),
                selected: false,
                arm: ShelfArm::Part(Box::new(row.clone())),
            })
        }
        ShelfEntry::BuiltinXspice(id) => {
            let descriptor = engine_only_xspice_devices()
                .iter()
                .find(|descriptor| descriptor.stable_id == *id)?;
            builtin_xspice_library_binding(descriptor).ok()?;
            Some(ShelfEntryRow {
                entry: entry.clone(),
                glyph: ShelfGlyph::Event,
                label: descriptor.display_name.to_owned(),
                meta: xspice_meta(descriptor).map(str::to_owned),
                selected: placing_cell
                    && pending
                        .and_then(|binding| binding.builtin_xspice.as_ref())
                        .is_some_and(|binding| binding.stable_id == *id),
                arm: ShelfArm::BuiltinXspice(id.clone()),
            })
        }
        ShelfEntry::Functional(id) => {
            let descriptor = functional_part(id)?;
            Some(ShelfEntryRow {
                entry: entry.clone(),
                glyph: category_glyph(descriptor.category),
                label: descriptor.display_name.to_owned(),
                meta: None,
                selected: placing_cell && functional_armed(pending, descriptor),
                arm: ShelfArm::Cell(Box::new(functional_library_binding(descriptor))),
            })
        }
        ShelfEntry::GeneratedVerilogA(model) => {
            let descriptor = generated_veriloga_devices()
                .iter()
                .find(|descriptor| descriptor.model_name == *model)?;
            let binding = generated_veriloga_library_binding(descriptor).ok()?;
            Some(ShelfEntryRow {
                entry: entry.clone(),
                glyph: ShelfGlyph::Text("VA"),
                label: builtin_label(descriptor.model_name).to_owned(),
                meta: Some(builtin_meta(
                    descriptor.terminals.len(),
                    descriptor.model_name,
                    descriptor.module_name,
                )),
                selected: placing_cell
                    && pending
                        .and_then(|binding| binding.generated_veriloga.as_ref())
                        .is_some_and(|binding| binding.model_name == *model),
                arm: ShelfArm::Cell(Box::new(binding)),
            })
        }
        ShelfEntry::Cell {
            library,
            cell,
            view,
        } => {
            let candidate = cells.iter().find(|candidate| {
                candidate.library == *library && candidate.cell == *cell && candidate.view == *view
            })?;
            if !candidate.ready {
                return None;
            }
            Some(ShelfEntryRow {
                entry: entry.clone(),
                glyph: ShelfGlyph::Icon(WorkbenchIcon::Models),
                label: candidate.cell.clone(),
                meta: Some(candidate.view.clone()),
                selected: placing_cell
                    && pending.is_some_and(|binding| binding == &candidate.binding),
                arm: ShelfArm::Cell(Box::new(candidate.binding.clone())),
            })
        }
        ShelfEntry::Supply(name) => {
            let supply = supply_palette().iter().find(|supply| supply.name == name)?;
            Some(ShelfEntryRow {
                entry: entry.clone(),
                glyph: supply_shelf_glyph(supply),
                label: supply.name.to_owned(),
                meta: None,
                selected: armed_supply(state) == Some(supply),
                arm: ShelfArm::Supply(supply),
            })
        }
    }
}

/// The Pinned band: the reader's own rail of parts, above the catalog.
///
/// Absent — header and all — when nothing in the set resolves, which is what
/// unpinning the last entry leaves behind. An empty band with a heading would
/// claim the reader still holds a set they just emptied.
fn pinned_band(
    ui: &mut Ui,
    state: &mut AppState,
    library_parts: &[LibraryPartRow],
    cells: &[CellCandidate],
) -> Option<ShelfArm> {
    let rows = pinned_keys(state)
        .iter()
        .filter_map(|key| ShelfEntry::from_storage_key(key))
        .filter_map(|entry| resolve_shelf_entry(state, library_parts, cells, &entry))
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return None;
    }
    let shortcut = state.model.preferences.shortcuts().resolved_label(
        &crate::workbench::commands::vocabulary::CommandRegistry,
        crate::workbench::commands::vocabulary::Command::PlaceInstance.key(),
        crate::workbench::app_state::runtime_command_platform(ui.ctx()),
        ui.ctx().os(),
    );
    shelf_section_header(
        ui,
        "Pinned",
        (!shortcut.is_empty()).then_some(shortcut.as_str()),
    );
    shelf_entry_rows(ui, state, &rows)
}

/// The Recent band: what this reader last put on a sheet, newest first.
///
/// A pinned entry is filtered out rather than listed twice — it is already one
/// band above, and six rows are too few to spend on a part the reader has
/// already given a permanent seat.
fn recent_band(
    ui: &mut Ui,
    state: &mut AppState,
    library_parts: &[LibraryPartRow],
    cells: &[CellCandidate],
) -> Option<ShelfArm> {
    let pinned = pinned_keys(state);
    let rows = state
        .model
        .preferences
        .component_shelf()
        .recent
        .iter()
        .filter(|key| !pinned.contains(key))
        .filter_map(|key| ShelfEntry::from_storage_key(key))
        .filter_map(|entry| resolve_shelf_entry(state, library_parts, cells, &entry))
        .take(RECENT_SHOWN)
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return None;
    }
    shelf_section_header(ui, "Recent", Some(&rows.len().to_string()));
    shelf_entry_rows(ui, state, &rows)
}

/// Paint one band's resolved rows on the shelf's own row contract.
fn shelf_entry_rows(ui: &mut Ui, state: &mut AppState, rows: &[ShelfEntryRow]) -> Option<ShelfArm> {
    let mut armed = None;
    for row in rows {
        let response = shelf_part_row(
            ui,
            row.glyph,
            &row.label,
            row.selected,
            row.meta.as_deref(),
            0,
        );
        if let Some(payload) = shelf_drag_payload(&row.arm) {
            response.dnd_set_drag_payload(payload);
        }
        if response.clicked() {
            armed = Some(row.arm.clone());
        }
        shelf_pin_context_menu(&response, state, &row.entry);
    }
    armed
}

/// The drag payload one armed identity travels to the canvas as.
///
/// A library part has none: its click may have to review and add a pack
/// first, and a drag that could end in a dialog is not a drag.
fn shelf_drag_payload(arm: &ShelfArm) -> Option<SchematicShelfDragPayload> {
    match arm {
        ShelfArm::Primitive(kind) => SchematicShelfDragPayload::primitive(*kind),
        ShelfArm::Cell(binding) => {
            Some(SchematicShelfDragPayload::library_cell((**binding).clone()))
        }
        ShelfArm::BuiltinXspice(id) => engine_only_xspice_devices()
            .iter()
            .find(|descriptor| descriptor.stable_id == *id)
            .and_then(|descriptor| builtin_xspice_library_binding(descriptor).ok())
            .map(SchematicShelfDragPayload::library_cell),
        // A variant arms its card and skin on the place tool, which a drop
        // does not carry.
        // A supply is a net label, which a drop does not place.
        ShelfArm::Variant(_) | ShelfArm::Part(_) | ShelfArm::Stimulus(_) | ShelfArm::Supply(_) => {
            None
        }
    }
}

/// Offer the row's pin state on its own context menu.
///
/// Every placeable row in the shelf carries this, so the rail is built from
/// the catalog where the reader finds the part rather than from a separate
/// editor. Shift+F10 opens it from the keyboard, as the navigator's object
/// menu does.
fn shelf_pin_context_menu(response: &Response, state: &mut AppState, entry: &ShelfEntry) {
    let (popup, _) = super::super::row_context_menu(response);
    let pinned = is_pinned(state, entry);
    popup.show(|ui| {
        if ui
            .button(if pinned { "Unpin" } else { "Pin to shelf" })
            .clicked()
        {
            toggle_pin(state, entry);
            ui.close();
        }
    });
}

/// Complete one click on a pinned or recent row through the same door its
/// catalog row uses.
fn apply_shelf_arm(app: &mut RSpiceApp, arm: ShelfArm, ctx: &egui::Context) {
    match arm {
        ShelfArm::Primitive(kind) => arm_primitive(app, kind, ctx),
        ShelfArm::Variant(entry) => arm_variant(&mut app.state, entry, ctx),
        ShelfArm::BuiltinXspice(id) => {
            if let Some(binding) = place_builtin_xspice(&mut app.state, &id) {
                arm_cell(&mut app.state, binding, ctx);
            }
        }
        ShelfArm::Cell(binding) => arm_cell(&mut app.state, *binding, ctx),
        ShelfArm::Part(row) => apply_library_part_row(app, *row, ctx),
        ShelfArm::Stimulus(name) => arm_stimulus_definition(&mut app.state, &name, ctx),
        ShelfArm::Supply(entry) => arm_supply(&mut app.state, entry, ctx),
    }
}

/// The built-in catalog: every functional group that has rows, each holding
/// native primitives and code models side by side.
pub(super) fn component_catalog(ui: &mut Ui, state: &mut AppState) -> Option<ShelfArm> {
    let mut armed = None;
    shelf_section_header(ui, "Built-in", Some(&catalog_row_count().to_string()));
    for category in ComponentCategory::ALL {
        let rows = palette_rows(category);
        if rows.is_empty() {
            continue;
        }
        if catalog_group_row(
            ui,
            ("component-shelf", category.title()),
            category_glyph(category),
            category.title(),
            rows.len(),
            category_open_by_default(category),
        ) {
            for row in rows {
                let clicked = match row {
                    PaletteRow::Primitive(entry) => primitive_row(ui, state, entry, 2),
                    PaletteRow::Functional(descriptor) => functional_row(ui, state, descriptor, 2),
                    PaletteRow::Xspice(descriptor) => xspice_row(ui, state, descriptor, 2),
                    PaletteRow::Supply(entry) => supply_row(ui, state, entry, 2),
                };
                armed = clicked.or(armed);
            }
        }
    }
    armed
}

/// One placeable shelf row: the mockup's `.shelf-part` — an identity glyph
/// at the symbol tint, the sans label, the mono meta column — on the same
/// 24 px schematic tree-row contract every navigator row keeps.
///
/// [`schematic_nav_row_indented_drag_response`] paints this geometry for
/// [`WorkbenchIcon`] rows; the shelf's identity column is a [`ShelfGlyph`],
/// so the shelf owns this variant rather than widening every navigator row
/// call in the crate with a parameter only this panel would pass.
pub(super) fn shelf_part_row(
    ui: &mut Ui,
    glyph: ShelfGlyph,
    label: &str,
    selected: bool,
    meta: Option<&str>,
    level: usize,
) -> Response {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(
        egui::vec2(ui.available_width(), SCHEMATIC_NAV_ROW_HEIGHT),
        egui::Sense::click_and_drag(),
    );
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label,
        )
    });
    if selected || response.hovered() {
        ui.painter().rect_filled(
            rect,
            0.0,
            if selected {
                t.color.accent_dim
            } else {
                t.color.bg_hover
            },
        );
    }
    if selected {
        ui.painter().rect_filled(
            egui::Rect::from_min_max(
                rect.left_top(),
                egui::pos2(rect.left() + 2.0, rect.bottom()),
            ),
            0.0,
            t.color.accent,
        );
    }
    let indent = 14.0 * level as f32;
    glyph.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 33.5 + indent, rect.center().y),
            egui::vec2(15.0, 15.0),
        ),
        t.color.symbol,
    );
    let meta_width = meta.map_or(0.0, |meta| {
        ui.painter()
            .layout_no_wrap(
                meta.to_owned(),
                theme::mono(SCHEMATIC_NAV_META_SIZE, FontWeight::Regular),
                t.color.text_faint,
            )
            .size()
            .x
    });
    let label_left = rect.left() + 47.0 + indent;
    let label_right = if meta.is_some() {
        rect.right() - 14.0 - meta_width
    } else {
        rect.right() - 8.0
    };
    ui.painter()
        .with_clip_rect(egui::Rect::from_x_y_ranges(
            label_left..=label_right.max(label_left),
            rect.y_range(),
        ))
        .text(
            egui::pos2(label_left, rect.center().y),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(SCHEMATIC_NAV_LABEL_SIZE, FontWeight::Regular),
            if selected {
                t.color.text
            } else {
                t.color.text_dim
            },
        );
    if let Some(meta) = meta {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            meta,
            theme::mono(SCHEMATIC_NAV_META_SIZE, FontWeight::Regular),
            t.color.text_faint,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    rail::row(ui, &response, level + 1, None);
    response
}

fn primitive_row(
    ui: &mut Ui,
    state: &mut AppState,
    entry: &'static ComponentPaletteEntry,
    level: usize,
) -> Option<ShelfArm> {
    let (selected, meta, pin, arm) = if entry.variant.is_some() {
        (
            variant_armed(state, entry),
            variant_meta(entry),
            ShelfEntry::Variant(entry.label.to_owned()),
            ShelfArm::Variant(entry),
        )
    } else {
        (
            primitive_armed(state, entry.kind),
            primitive_shelf_meta(entry.kind),
            ShelfEntry::Primitive(entry.kind),
            ShelfArm::Primitive(entry.kind),
        )
    };
    let response = shelf_part_row(
        ui,
        primitive_shelf_glyph(entry.kind),
        entry.label,
        selected,
        meta.as_deref(),
        level,
    );
    if entry.variant.is_none()
        && let Some(payload) = SchematicShelfDragPayload::primitive(entry.kind)
    {
        response.dnd_set_drag_payload(payload);
    }
    shelf_pin_context_menu(&response, state, &pin);
    response.clicked().then_some(arm)
}

/// The value a variant row places: its own card, or the type's default.
fn variant_model(entry: &ComponentPaletteEntry) -> &'static str {
    entry.default_model.unwrap_or(entry.kind.default_value())
}

/// A variant row's meta column: the placed default, unless the row names its
/// own card, which its label already says.
fn variant_meta(entry: &ComponentPaletteEntry) -> Option<String> {
    entry
        .default_model
        .is_none()
        .then(|| primitive_shelf_meta(entry.kind))
        .flatten()
}

/// The palette variant row an armed card and skin came from, if any.
fn variant_entry(
    kind: ComponentType,
    variant: Option<&str>,
    model: &str,
) -> Option<&'static ComponentPaletteEntry> {
    component_palette()
        .iter()
        .flat_map(|section| section.entries.iter())
        .find(|entry| {
            entry.kind == kind
                && entry.variant.is_some()
                && entry.variant == variant
                && variant_model(entry) == model
        })
}

/// Whether a palette variant row places this library part's card on its
/// device.
fn placed_by_variant_row(row: &LibraryPartRow) -> bool {
    let LibraryPartAction::Arm(placement) = &row.action else {
        return false;
    };
    let PartPlacement::NativeDevice {
        component_type,
        model,
        ..
    } = placement.as_ref()
    else {
        return false;
    };
    component_palette()
        .iter()
        .flat_map(|section| section.entries.iter())
        .any(|entry| {
            entry.kind == *component_type
                && entry.variant.is_some()
                && entry.default_model == Some(model.as_str())
        })
}

/// Whether the place tool holds this primitive bare, with no card or skin.
/// The supply rail the Label tool holds from the shelf, if any.
fn armed_supply(state: &AppState) -> Option<&'static SupplyPaletteEntry> {
    let editor = &state.model.schematic.session.editor;
    if editor.tool != Tool::Label {
        return None;
    }
    let sequence = editor.pending_net_label_sequence.as_ref()?;
    let name = sequence.repeated_name()?;
    supply_palette()
        .iter()
        .find(|supply| supply.name == name && supply.kind() == sequence.kind)
}

fn supply_shelf_glyph(entry: &SupplyPaletteEntry) -> ShelfGlyph {
    ShelfGlyph::Supply(entry.glyph())
}

/// Arm the Label tool to place one supply rail on every click.
pub(super) fn arm_supply(state: &mut AppState, entry: &SupplyPaletteEntry, ctx: &egui::Context) {
    state.model.schematic.session.editor.pending_library_cell = None;
    state.model.schematic.arm_tool(Tool::Label);
    state
        .model
        .schematic
        .session
        .editor
        .pending_net_label_sequence = Some(
        rspice_schematic_editor::session::net_label_sequence::NetLabelSequence::repeating(
            entry.name,
            entry.kind(),
        ),
    );
    finish_shelf_placement(state, ctx);
}

fn supply_row(
    ui: &mut Ui,
    state: &mut AppState,
    entry: &'static SupplyPaletteEntry,
    level: usize,
) -> Option<ShelfArm> {
    let response = shelf_part_row(
        ui,
        supply_shelf_glyph(entry),
        entry.name,
        armed_supply(state) == Some(entry),
        None,
        level,
    );
    shelf_pin_context_menu(&response, state, &ShelfEntry::Supply(entry.name.to_owned()));
    response.clicked().then_some(ShelfArm::Supply(entry))
}

fn primitive_armed(state: &AppState, kind: ComponentType) -> bool {
    let editor = &state.model.schematic.session.editor;
    editor.tool == Tool::Place(kind)
        && editor
            .pending_part_model
            .as_ref()
            .is_none_or(|armed| armed.tool != Tool::Place(kind))
}

/// Whether the place tool holds exactly this variant row's card and skin.
fn variant_armed(state: &AppState, entry: &ComponentPaletteEntry) -> bool {
    let editor = &state.model.schematic.session.editor;
    editor.tool == Tool::Place(entry.kind)
        && editor.pending_part_model.as_ref().is_some_and(|armed| {
            armed.tool == Tool::Place(entry.kind)
                && armed.variant.as_deref() == entry.variant
                && armed.model == variant_model(entry)
        })
}

/// Arm the place tool with a variant row's card and skin.
pub(super) fn arm_variant(
    state: &mut AppState,
    entry: &ComponentPaletteEntry,
    ctx: &egui::Context,
) {
    let _ = crate::workbench::app::arm_pack_part(
        &mut state.model,
        PartPlacement::NativeDevice {
            component_type: entry.kind,
            variant: entry.variant.map(str::to_owned),
            model: variant_model(entry).to_owned(),
        },
    );
    finish_shelf_placement(state, ctx);
}

/// The Stimulus library section: every definition this project authored, as a
/// placeable part.
///
/// It sits directly under the built-in groups because that is what a
/// definition is — a source of one of the families above it, with its fields
/// already filled in — and above the pack and cell catalogs, which publish devices
/// rather than excitations. A project that has authored none paints nothing at
/// all rather than an empty header: the shelf's other sections do the same, and
/// a heading over nothing is a door that leads nowhere.
///
/// Returns the definition a click armed.
fn stimulus_library_section(ui: &mut Ui, state: &mut AppState) -> Option<String> {
    let rows = stimulus_library_rows(state);
    if rows.is_empty() {
        return None;
    }
    if !catalog_group_row(
        ui,
        "component-shelf-stimulus-library",
        ShelfGlyph::Source,
        "Stimulus library",
        rows.len(),
        false,
    ) {
        return None;
    }

    let mut armed = None;
    for row in &rows {
        let response = shelf_part_row(
            ui,
            ShelfGlyph::Source,
            &row.name,
            state
                .model
                .schematic
                .session
                .editor
                .pending_stimulus
                .as_ref()
                .is_some_and(|held| held.definition().eq_ignore_ascii_case(&row.name)),
            Some(&row.meta),
            2,
        )
        .on_hover_text(&row.hover);
        response.dnd_set_drag_payload(SchematicShelfDragPayload::stimulus(row.placement.clone()));
        if response.clicked() {
            armed = Some(row.name.clone());
        }
    }
    armed
}

/// One Stimulus library row, resolved through the definition's own realization.
struct StimulusShelfRow {
    name: String,
    /// `V · SIN · 1 kHz`, from the instance this definition would place.
    meta: String,
    hover: String,
    /// The definition's family and purpose, which a query reads as words.
    keywords: Vec<String>,
    placement: crate::state::PendingStimulusPlacement,
}

/// Every definition, in the library's own list order.
fn stimulus_library_rows(state: &AppState) -> Vec<StimulusShelfRow> {
    state
        .model
        .workspace
        .content
        .stimulus_library
        .definitions()
        .iter()
        .map(|definition| {
            let instance = definition.transient_component();
            let meta = crate::simulation::placed_sources::source_identity_line(&instance)
                .unwrap_or_else(|| definition.family().label().to_owned());
            let revision = definition.revision();
            let mut hover = format!("{} \u{00b7} r{revision} \u{00b7} {meta}", definition.name());
            if !definition.purpose.trim().is_empty() {
                hover.push('\n');
                hover.push_str(definition.purpose.trim());
            }
            hover.push_str("\nClick to place it, or drag it onto the schematic");
            StimulusShelfRow {
                name: definition.name().to_owned(),
                meta,
                hover,
                keywords: vec![
                    definition.family().label().to_owned(),
                    definition.purpose.clone(),
                ],
                placement: crate::state::PendingStimulusPlacement::of(definition),
            }
        })
        .collect()
}

/// Arm one definition from the shelf; a refusal answers on the status bar.
fn arm_stimulus_definition(state: &mut AppState, name: &str, ctx: &egui::Context) {
    match crate::workbench::app::arm_placement_from_definition(
        &mut state.model,
        &mut state.workbench,
        name,
    ) {
        Ok(()) => finish_shelf_placement(state, ctx),
        Err(refusal) => state.prompt(refusal),
    }
}

/// A code model's meta column: its netlist model name, unless that name only
/// repeats the label (`potentiometer`, `oneshot`).
fn xspice_meta(descriptor: &CatalogXspiceDeviceDescriptor) -> Option<&'static str> {
    let folded = |text: &str| {
        text.chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>()
    };
    (folded(descriptor.model_type) != folded(descriptor.display_name))
        .then_some(descriptor.model_type)
}

/// Whether the place tool holds `descriptor`'s binding.
fn functional_armed(
    pending: Option<&LibraryCellInstance>,
    descriptor: &FunctionalPartDescriptor,
) -> bool {
    pending
        .and_then(|binding| binding.builtin_functional.as_ref())
        .is_some_and(|binding| binding.stable_id == descriptor.stable_id)
}

fn functional_row(
    ui: &mut Ui,
    state: &mut AppState,
    descriptor: &'static FunctionalPartDescriptor,
    level: usize,
) -> Option<ShelfArm> {
    let editor = &state.model.schematic.session.editor;
    let selected = editor.tool == Tool::Place(ComponentType::CellInstance)
        && functional_armed(
            editor
                .pending_library_cell
                .as_ref()
                .map(|pending| &pending.binding),
            descriptor,
        );
    let response = shelf_part_row(
        ui,
        category_glyph(descriptor.category),
        descriptor.display_name,
        selected,
        None,
        level,
    );
    let binding = functional_library_binding(descriptor);
    response.dnd_set_drag_payload(SchematicShelfDragPayload::library_cell(binding.clone()));
    shelf_pin_context_menu(
        &response,
        state,
        &ShelfEntry::Functional(descriptor.stable_id.to_owned()),
    );
    response
        .clicked()
        .then(|| ShelfArm::Cell(Box::new(binding)))
}

fn xspice_row(
    ui: &mut Ui,
    state: &mut AppState,
    descriptor: &CatalogXspiceDeviceDescriptor,
    level: usize,
) -> Option<ShelfArm> {
    let selected = state
        .model
        .schematic
        .session
        .editor
        .pending_library_cell
        .as_ref()
        .map(|pending| &pending.binding)
        .and_then(|binding| binding.builtin_xspice.as_ref())
        .is_some_and(|binding| binding.stable_id == descriptor.stable_id)
        && state.model.schematic.session.editor.tool == Tool::Place(ComponentType::CellInstance);
    let response = shelf_part_row(
        ui,
        ShelfGlyph::Event,
        descriptor.display_name,
        selected,
        xspice_meta(descriptor),
        level,
    );
    let stable_id = descriptor.stable_id;
    match builtin_xspice_library_binding(descriptor) {
        Ok(binding) => {
            response.dnd_set_drag_payload(SchematicShelfDragPayload::library_cell(binding));
            shelf_pin_context_menu(
                &response,
                state,
                &ShelfEntry::BuiltinXspice(stable_id.to_owned()),
            );
            response
                .clicked()
                .then(|| ShelfArm::BuiltinXspice(stable_id.to_owned()))
        }
        Err(error) => {
            log::error!("Cannot expose {stable_id} in the XSPICE catalog: {error}");
            None
        }
    }
}

/// Decide what one built-in XSPICE click does.
///
/// A code model whose vector ports are not fixed cannot be armed until their
/// widths are chosen, so the click raises the placement dialog and arms
/// nothing. Shared by the catalog row and the pinned/recent rows so both forks
/// stay one decision.
pub(super) fn place_builtin_xspice(
    state: &mut AppState,
    stable_id: &str,
) -> Option<LibraryCellInstance> {
    let descriptor = engine_only_xspice_devices()
        .iter()
        .find(|descriptor| descriptor.stable_id == stable_id)?;
    let binding = builtin_xspice_library_binding(descriptor).ok()?;
    match builtin_xspice_vector_ports(descriptor) {
        Ok(vector_ports)
            if vector_ports
                .iter()
                .any(|port| port.maximum.is_none_or(|maximum| maximum != port.minimum)) =>
        {
            state.workbench.close_drawer();
            state.design.builtin_xspice_placement.open(
                descriptor.stable_id,
                descriptor.display_name,
                vector_ports,
                crate::workbench::app::schematic_editor_request_source(state),
            );
            None
        }
        Ok(_) => Some(binding),
        Err(error) => {
            log::error!("Cannot configure {stable_id} in the XSPICE catalog: {error}");
            None
        }
    }
}

/// What the built-in library calls a model.
pub(super) fn builtin_label(registry_name: &'static str) -> &'static str {
    builtin_veriloga_model(registry_name).map_or(registry_name, |model| model.name)
}

/// A built-in model's pins and device class, beside its name.
pub(super) fn builtin_meta(pins: usize, registry_name: &str, module_name: &str) -> String {
    format!(
        "{pins} pin \u{00b7} {}",
        builtin_veriloga_model(registry_name).map_or(module_name, |model| model.class.label())
    )
}

/// `devices` in the order the built-in library lists them, grouped by class.
pub(super) fn in_library_order<T>(devices: &mut [T], registry_name: impl Fn(&T) -> &str) {
    devices.sort_by_key(|device| {
        BUILTIN_VERILOGA_MODELS
            .iter()
            .position(|model| model.registry_name == registry_name(device))
            .unwrap_or(usize::MAX)
    });
}

fn generated_veriloga_catalog(ui: &mut Ui, state: &mut AppState) -> Option<LibraryCellInstance> {
    let descriptors = generated_veriloga_devices();
    if descriptors.is_empty() {
        return None;
    }
    if !catalog_group_row(
        ui,
        "component-shelf-generated-veriloga",
        ShelfGlyph::Text("VA"),
        BUILTIN_VERILOGA_LIBRARY,
        descriptors.len(),
        false,
    ) {
        return None;
    }

    let mut armed = None;
    let mut descriptors = descriptors.iter().collect::<Vec<_>>();
    in_library_order(&mut descriptors, |descriptor| descriptor.model_name);
    for descriptor in descriptors {
        let selected = state
            .model
            .schematic
            .session
            .editor
            .pending_library_cell
            .as_ref()
            .map(|pending| &pending.binding)
            .and_then(|binding| binding.generated_veriloga.as_ref())
            .is_some_and(|binding| binding.model_name == descriptor.model_name)
            && state.model.schematic.session.editor.tool
                == Tool::Place(ComponentType::CellInstance);
        let response = shelf_part_row(
            ui,
            ShelfGlyph::Text("VA"),
            builtin_label(descriptor.model_name),
            selected,
            Some(&builtin_meta(
                descriptor.terminals.len(),
                descriptor.model_name,
                descriptor.module_name,
            )),
            2,
        );
        match generated_veriloga_library_binding(descriptor) {
            Ok(binding) => {
                response
                    .dnd_set_drag_payload(SchematicShelfDragPayload::library_cell(binding.clone()));
                if response.clicked() {
                    armed = Some(binding);
                }
                shelf_pin_context_menu(
                    &response,
                    state,
                    &ShelfEntry::GeneratedVerilogA(descriptor.model_name.to_owned()),
                );
            }
            Err(error) => log::error!(
                "Cannot expose generated Verilog-A model '{}': {error}",
                descriptor.model_name
            ),
        }
    }
    armed
}

fn project_library(
    ui: &mut Ui,
    state: &mut AppState,
    cells: &[CellCandidate],
) -> Option<LibraryCellInstance> {
    let mut grouped = BTreeMap::<String, Vec<CellCandidate>>::new();
    for candidate in cells.iter().cloned() {
        grouped
            .entry(candidate.library.clone())
            .or_default()
            .push(candidate);
    }
    shelf_section_header(ui, "Project library", None);
    let mut armed = None;
    for (library, grouped_cells) in grouped {
        if catalog_group_row(
            ui,
            ("component-shelf-library", library.as_str()),
            ShelfGlyph::Icon(WorkbenchIcon::Models),
            &library,
            grouped_cells.len(),
            false,
        ) {
            armed = cell_rows(ui, state, &grouped_cells, 2).or_else(|| armed.take());
        }
    }
    armed
}

fn cell_rows(
    ui: &mut Ui,
    state: &mut AppState,
    cells: &[CellCandidate],
    level: usize,
) -> Option<LibraryCellInstance> {
    let mut armed = None;
    for candidate in cells {
        let meta = if candidate.ready {
            candidate.view.as_str()
        } else {
            candidate.unavailable_reason.as_str()
        };
        let offered = ui
            .add_enabled_ui(candidate.ready, |ui| {
                if candidate.ready {
                    let payload =
                        SchematicShelfDragPayload::library_cell(candidate.binding.clone());
                    let response = schematic_nav_row_indented_drag_response(
                        ui,
                        WorkbenchIcon::Models,
                        &candidate.cell,
                        false,
                        Some(meta),
                        level,
                        false,
                        false,
                        false,
                    );
                    response.dnd_set_drag_payload(payload);
                    Some(response.on_hover_text(format!(
                        "Click to arm {}/{} or drag it onto the sheet",
                        candidate.library, candidate.cell
                    )))
                } else {
                    nav_row_indented(
                        ui,
                        WorkbenchIcon::Models,
                        &candidate.cell,
                        false,
                        Some(meta),
                        level,
                    );
                    None
                }
            })
            .inner;
        // A cell whose view cannot be drawn has no placement to pin.
        let Some(response) = offered else {
            continue;
        };
        if response.clicked() {
            armed = Some(candidate.binding.clone());
        }
        shelf_pin_context_menu(
            &response,
            state,
            &ShelfEntry::Cell {
                library: candidate.library.clone(),
                cell: candidate.cell.clone(),
                view: candidate.view.clone(),
            },
        );
    }
    armed
}

/// Mockup-native expandable tree row used by the component shelf.
///
/// `egui::CollapsingHeader` carries stock indentation, typography, and
/// animation that do not match the workbench's 31 px tree-row contract.  The
/// shelf keeps only the persisted disclosure state and paints the same row
/// geometry as the rest of the navigator.
fn catalog_group_row(
    ui: &mut Ui,
    key: impl std::hash::Hash + std::fmt::Debug,
    glyph: ShelfGlyph,
    label: &str,
    count: usize,
    open_default: bool,
) -> bool {
    let t = Tokens::get(ui.ctx());
    let id = ui.make_persistent_id(key);
    let mut open = ui.data_mut(|data| data.get_persisted::<bool>(id).unwrap_or(open_default));
    let (rect, response) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 24.0), egui::Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::labeled(egui::WidgetType::Button, ui.is_enabled(), label)
    });
    if response.hovered() {
        ui.painter().rect_filled(rect, 0.0, t.color.bg_hover);
    }

    let caret_center = egui::pos2(rect.left() + 26.5, rect.center().y);
    let caret_stroke = egui::Stroke::new(1.25, t.color.text_faint);
    let caret_points = if open {
        [
            egui::pos2(caret_center.x - 3.0, caret_center.y - 1.5),
            egui::pos2(caret_center.x, caret_center.y + 1.5),
            egui::pos2(caret_center.x + 3.0, caret_center.y - 1.5),
        ]
    } else {
        [
            egui::pos2(caret_center.x - 1.5, caret_center.y - 3.0),
            egui::pos2(caret_center.x + 1.5, caret_center.y),
            egui::pos2(caret_center.x - 1.5, caret_center.y + 3.0),
        ]
    };
    ui.painter()
        .add(egui::Shape::line(caret_points.to_vec(), caret_stroke));
    glyph.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(rect.left() + 46.5, rect.center().y),
            egui::vec2(15.0, 15.0),
        ),
        t.color.text_faint,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 60.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().text(
        egui::pos2(rect.right() - 8.0, rect.center().y),
        egui::Align2::RIGHT_CENTER,
        count.to_string(),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_faint,
    );
    theme::paint_focus_ring(ui, &response, rect);

    if response.clicked() {
        open = !open;
        ui.data_mut(|data| data.insert_persisted(id, open));
    }
    ui.ctx().accesskit_node_builder(response.id, |node| {
        node.set_expanded(open);
    });
    // The shelf's own root row: the band above it paints no control and takes
    // no focus, so this is what Left climbs out to from a part row.
    rail::row(
        ui,
        &response,
        0,
        Some(RailDisclosure {
            unfolded: open,
            fold: RailFold::Persisted(id),
        }),
    );
    open
}

type CellCandidate = LibraryCellPlacementCandidate;

pub(super) fn cell_candidates(state: &AppState) -> Vec<CellCandidate> {
    library_cell_placement_candidates(&state.model.library_manager, &state.model.workspace)
}

pub(super) fn arm_primitive(app: &mut RSpiceApp, kind: ComponentType, ctx: &egui::Context) {
    if kind == ComponentType::Port {
        crate::workbench::commands::vocabulary::Command::PlacePin.execute(app);
        return;
    }
    let state = &mut app.state;
    state.model.schematic.session.editor.pending_library_cell = None;
    state.model.schematic.arm_tool(Tool::Place(kind));
    finish_shelf_placement(state, ctx);
}

/// Capture the current placement context and return focus to the canvas.
fn arm_cell(
    state: &mut crate::workbench::app_state::AppState,
    binding: LibraryCellInstance,
    ctx: &egui::Context,
) {
    let _ = crate::workbench::app::arm_library_cell_placement(state, binding);
    finish_shelf_placement(state, ctx);
}

/// Hand the canvas back after a shelf click.
///
/// No toast: the armed cursor is the canvas's own answer, and one toast per
/// shelf click is one per placement.
fn finish_shelf_placement(state: &mut AppState, ctx: &egui::Context) {
    // Selecting a part releases the compact shelf's modal input ownership.
    state.workbench.close_drawer();
    rspice_schematic_editor::view::canvas::request_focus(ctx);
}

/// The label a primitive's own shelf row carries.
pub(super) fn primitive_label(kind: ComponentType) -> &'static str {
    component_palette()
        .iter()
        .flat_map(|section| section.entries)
        .find(|entry| entry.kind == kind && entry.variant.is_none())
        .map_or(kind.display_name(), |entry| entry.label)
}

/// Rows the built-in catalog holds across every group.
pub(super) fn catalog_row_count() -> usize {
    ComponentCategory::ALL
        .into_iter()
        .map(|category| palette_rows(category).len())
        .sum()
}

/// Answers a query lists before asking the reader to narrow it.
const SHELF_HITS: usize = 200;

/// One answer to a shelf query: the row it paints and where its click goes.
pub(super) struct ShelfHit {
    rank: MatchRank,
    glyph: ShelfGlyph,
    pub(super) label: String,
    meta: Option<String>,
    pub(super) selected: bool,
    arm: ShelfArm,
    /// The pinnable identity; a stimulus definition is project state and has
    /// none.
    pin: Option<ShelfEntry>,
    drag: Option<SchematicShelfDragPayload>,
    /// Why the row cannot be placed, when it cannot.
    pub(super) refusal: Option<String>,
}

impl ShelfHit {
    fn of(rank: MatchRank, row: ShelfEntryRow) -> Self {
        Self {
            rank,
            glyph: row.glyph,
            label: row.label,
            meta: row.meta,
            selected: row.selected,
            drag: shelf_drag_payload(&row.arm),
            arm: row.arm,
            pin: Some(row.entry),
            refusal: None,
        }
    }
}

/// The answer Enter places: the best-ranked one that can be placed.
pub(super) fn top_hit(hits: &[ShelfHit]) -> Option<&ShelfHit> {
    hits.iter().find(|hit| hit.refusal.is_none())
}

/// Every placeable identity the query answers, best first.
///
/// Rows of one rank keep catalog order: primitives, code models, generated
/// Verilog-A, stimulus definitions, library parts, project cells.
pub(super) fn shelf_hits(
    state: &AppState,
    query: &ShelfQuery,
    library_parts: &[LibraryPartRow],
    cells: &[CellCandidate],
) -> Vec<ShelfHit> {
    let mut hits = Vec::new();
    let offer = |hits: &mut Vec<ShelfHit>, rank: Option<MatchRank>, entry: ShelfEntry| {
        if let Some(rank) = rank
            && let Some(row) = resolve_shelf_entry(state, library_parts, cells, &entry)
        {
            hits.push(ShelfHit::of(rank, row));
        }
    };
    for row in ComponentCategory::ALL.into_iter().flat_map(palette_rows) {
        match row {
            PaletteRow::Primitive(entry) if entry.variant.is_some() => {
                let rank = query.rank(&ShelfTerms {
                    label: entry.label,
                    keywords: entry.keywords,
                    part_numbers: entry.default_model.as_slice(),
                });
                offer(&mut hits, rank, ShelfEntry::Variant(entry.label.to_owned()));
            }
            PaletteRow::Primitive(entry) => {
                let mut keywords = entry.keywords.to_vec();
                keywords.push(entry.kind.display_name());
                let rank = query.rank(&ShelfTerms {
                    label: entry.label,
                    keywords: &keywords,
                    part_numbers: &[],
                });
                offer(&mut hits, rank, ShelfEntry::Primitive(entry.kind));
            }
            PaletteRow::Supply(entry) => {
                let rank = query.rank(&ShelfTerms {
                    label: entry.name,
                    keywords: entry.keywords,
                    part_numbers: &[],
                });
                offer(&mut hits, rank, ShelfEntry::Supply(entry.name.to_owned()));
            }
            PaletteRow::Functional(descriptor) => {
                let rank = query.rank(&ShelfTerms {
                    label: descriptor.display_name,
                    keywords: descriptor.keywords,
                    part_numbers: &[],
                });
                offer(
                    &mut hits,
                    rank,
                    ShelfEntry::Functional(descriptor.stable_id.to_owned()),
                );
            }
            PaletteRow::Xspice(descriptor) => {
                let mut part_numbers = vec![descriptor.model_type];
                part_numbers.extend_from_slice(descriptor.aliases);
                let rank = query.rank(&ShelfTerms {
                    label: descriptor.display_name,
                    keywords: descriptor.keywords,
                    part_numbers: &part_numbers,
                });
                offer(
                    &mut hits,
                    rank,
                    ShelfEntry::BuiltinXspice(descriptor.stable_id.to_owned()),
                );
            }
        }
    }
    for descriptor in generated_veriloga_devices() {
        let rank = query.rank(&ShelfTerms {
            label: builtin_label(descriptor.model_name),
            keywords: &[descriptor.model_name],
            part_numbers: &[descriptor.module_name, descriptor.source_digest],
        });
        offer(
            &mut hits,
            rank,
            ShelfEntry::GeneratedVerilogA(descriptor.model_name.to_owned()),
        );
    }
    let pending_stimulus = state
        .model
        .schematic
        .session
        .editor
        .pending_stimulus
        .as_ref()
        .map(|held| held.definition().to_owned());
    for row in stimulus_library_rows(state) {
        let keywords = row.keywords.iter().map(String::as_str).collect::<Vec<_>>();
        let Some(rank) = query.rank(&ShelfTerms {
            label: &row.name,
            keywords: &keywords,
            part_numbers: &[],
        }) else {
            continue;
        };
        hits.push(ShelfHit {
            rank,
            glyph: ShelfGlyph::Source,
            selected: pending_stimulus
                .as_deref()
                .is_some_and(|held| held.eq_ignore_ascii_case(&row.name)),
            drag: Some(SchematicShelfDragPayload::stimulus(row.placement)),
            arm: ShelfArm::Stimulus(row.name.clone()),
            label: row.name,
            meta: Some(row.meta),
            pin: None,
            refusal: None,
        });
    }
    for row in library_parts {
        // A card a variant row places is answered by that row, which ranks on
        // the card's name as well, so the search lists the part once.
        if placed_by_variant_row(row) {
            continue;
        }
        let rank = library_part_rank(query, &row.part_id, &row.device, &row.source);
        match &row.action {
            // Still listed, so a search for it says why it cannot be placed
            // instead of claiming it does not exist.
            LibraryPartAction::Refused(reason) => {
                if let Some(rank) = rank {
                    hits.push(ShelfHit {
                        rank,
                        glyph: ShelfGlyph::Icon(WorkbenchIcon::Models),
                        label: row.part_id.clone(),
                        meta: Some(row.meta.clone()),
                        selected: false,
                        arm: ShelfArm::Part(Box::new(row.clone())),
                        pin: None,
                        drag: None,
                        refusal: Some(reason.clone()),
                    });
                }
            }
            _ => offer(
                &mut hits,
                rank,
                ShelfEntry::LibraryPart(row.part_id.clone()),
            ),
        }
    }
    for candidate in cells {
        let rank = query.rank(&ShelfTerms {
            label: &candidate.cell,
            keywords: &[],
            part_numbers: &[&candidate.library, &candidate.view],
        });
        if candidate.ready {
            let entry = ShelfEntry::Cell {
                library: candidate.library.clone(),
                cell: candidate.cell.clone(),
                view: candidate.view.clone(),
            };
            offer(&mut hits, rank, entry);
        } else if let Some(rank) = rank {
            hits.push(ShelfHit {
                rank,
                glyph: ShelfGlyph::Icon(WorkbenchIcon::Models),
                label: candidate.cell.clone(),
                meta: Some(candidate.unavailable_reason.clone()),
                selected: false,
                arm: ShelfArm::Cell(Box::new(candidate.binding.clone())),
                pin: None,
                drag: None,
                refusal: Some(candidate.unavailable_reason.clone()),
            });
        }
    }
    hits.sort_by_key(|hit| hit.rank);
    hits.truncate(SHELF_HITS);
    hits
}

/// The answers to a standing query, best first, on the shelf's row contract.
///
/// The row Enter would place carries the hover fill, so the field's answer to
/// Enter is on screen before the key is pressed.
fn shelf_results(ui: &mut Ui, state: &mut AppState, hits: &[ShelfHit]) -> Option<ShelfArm> {
    if hits.is_empty() {
        empty_navigator_row(ui, "No component or cell matches this filter");
        return None;
    }
    shelf_section_header(ui, "Matches", Some(&hits.len().to_string()));
    let top = hits.iter().position(|hit| hit.refusal.is_none());
    let hover = Tokens::get(ui.ctx()).color.bg_hover;
    let mut armed = None;
    for (index, hit) in hits.iter().enumerate() {
        let placeable = hit.refusal.is_none();
        let response = ui
            .add_enabled_ui(placeable, |ui| {
                let fill = ui.painter().add(egui::Shape::Noop);
                let response = shelf_part_row(
                    ui,
                    hit.glyph,
                    &hit.label,
                    hit.selected,
                    hit.meta.as_deref(),
                    0,
                );
                if top == Some(index) && !hit.selected {
                    ui.painter()
                        .set(fill, egui::Shape::rect_filled(response.rect, 0.0, hover));
                }
                response
            })
            .inner;
        if let Some(reason) = &hit.refusal {
            response.on_disabled_hover_text(reason.as_str());
            continue;
        }
        if let Some(payload) = hit.drag.clone() {
            response.dnd_set_drag_payload(payload);
        }
        if response.clicked() {
            armed = Some(hit.arm.clone());
        }
        if let Some(entry) = &hit.pin {
            shelf_pin_context_menu(&response, state, entry);
        }
    }
    armed
}

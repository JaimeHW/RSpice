//! The Component shelf: the design panel's other tab, where a part is picked.
//!
//! A separate surface from the navigator beside it, and separated here for the
//! reason the two tabs exist at all: the navigator answers about the design in
//! front of the reader, and this answers about everything that could be added
//! to it. They share only how a query is folded ([`super::normalized`]) and
//! what it matches ([`super::matches_query`]).
//!
//! Almost nothing here takes the whole application. A shelf row arms a tool,
//! writes a pin, or credits a placement, and all three live in
//! [`AppState`] — so the handlers take that rather than the application that
//! holds it. The five that do not are the ones that route somewhere else
//! entirely: a part the project has not adopted raises the pack confirmation
//! in the Models workspace, and a pin placement runs the `PlacePin` command.
//! Those go through the command vocabulary, which is defined over the whole
//! application.

use std::collections::{BTreeMap, HashSet};

use egui::{Response, ScrollArea, Ui};

use crate::schematic::view::SchematicShelfDragPayload;
use crate::schematic::{ComponentPaletteEntry, component_palette};
use crate::state::model_hub::{
    ModelHubPartRow, PartPlacement, PartProvenance, PartState, plan_library_placement,
    refusal_sentence,
};
use crate::state::model_library::{ModelLibrary, ModelSourceAuthority};
use crate::state::{
    ComponentType, LibraryCellInstance, LibraryCellPlacementCandidate, Tool,
    builtin_xspice_library_binding, builtin_xspice_vector_ports, engine_only_xspice_devices,
    generated_veriloga_devices, generated_veriloga_library_binding,
    library_cell_placement_candidates,
};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::RSpiceApp;
use crate::workbench::app_state::AppState;

use super::super::super::super::design_system::{
    WorkbenchIcon, schematic_section_header as shelf_section_header,
};
use super::super::rail::{self, RailDisclosure, RailFold};
use super::super::{
    SCHEMATIC_NAV_LABEL_SIZE, SCHEMATIC_NAV_META_SIZE, SCHEMATIC_NAV_ROW_HEIGHT,
    empty_navigator_row, panel_search, schematic_nav_row_indented_drag_response,
    schematic_nav_row_indented_response,
};
use super::{matches_query, normalized};

/// The shelf's primitive groups: the band's name, the family glyph its group
/// row carries, whether a fresh install shows the band open, and the palette
/// sections it gathers. The family glyphs follow the mockup's `PART_CATALOG`
/// group column — Passives —, Sources ◯, Analog ▷, Mixed signal ⊞ — with the
/// non-Latin marks painted as vector geometry because the bundled faces do
/// not hold them (see [`ShelfGlyph`]).
///
/// Passives is the one band open on first run — the mockup's
/// `shelfOpenGroups: { Passives: true }` — so a new install's shelf leads
/// with placeable rows instead of four folded bands over an empty-looking
/// panel. Every position is still the reader's own once moved: only the
/// default behind the persisted flag differs per group.
pub(super) const PRIMITIVE_GROUPS: [(&str, ShelfGlyph, bool, &[&str]); 4] = [
    (
        "Passives",
        ShelfGlyph::Text("\u{2014}"),
        true,
        &["Passives"],
    ),
    ("Sources", ShelfGlyph::Source, false, &["Sources"]),
    (
        "Analog",
        ShelfGlyph::Amp,
        false,
        &["Hierarchy", "Semiconductors", "Controlled sources"],
    ),
    (
        "Mixed signal / XSPICE",
        ShelfGlyph::Event,
        false,
        &["Behavioral (XSPICE)"],
    ),
];

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
/// The bundled IBM Plex faces are Latin subsets: every non-Latin candidate in
/// the mockup vocabulary — ◯ ▷ △ ⊞ ⊳ ⏚, and even Σ and Ω — rasterizes as a
/// tofu box, which is why the family marks are vector geometry rather than
/// text. `every_shelf_glyph_paints_ink_and_no_text_glyph_is_a_tofu_box`
/// walks the table and fails on any glyph the faces lack.
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
        }
    }
}

/// The shelf's meta column for one placeable primitive: `prefix · default`,
/// e.g. `R · 1k` — what the designator will start with and the value the
/// placed instance opens carrying.
///
/// Both halves come from the type's own metadata — [`ComponentType::
/// spice_prefix`] and [`ComponentType::default_value`], the same single
/// source [`crate::state`]'s placement writes into a fresh component — and
/// the value is re-presented through the crate's engineering formatter the
/// way the property editor re-presents an untouched draft, so the shelf and
/// the editor spell one decade the same way. A default the formatter cannot
/// read (`V=0`) is stated as authored; a part with no meaningful default
/// keeps the prefix alone, and a structural row with neither says nothing.
pub(super) fn primitive_shelf_meta(kind: ComponentType) -> Option<String> {
    let prefix = kind.spice_prefix();
    let default = kind.default_value();
    let value = if default.is_empty() {
        String::new()
    } else {
        crate::quantity::parse_engineering_value(default)
            .map(crate::quantity::format_engineering_value)
            .unwrap_or_else(|_| default.to_owned())
    };
    match (prefix.is_empty(), value.is_empty()) {
        (true, true) => None,
        (true, false) => Some(value),
        (false, true) => Some(prefix.to_owned()),
        (false, false) => Some(format!("{prefix} \u{00b7} {value}")),
    }
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
    let enter_rows = shelf_search(ui, &mut app.state);
    rail::open(ui.ctx());
    let query = normalized(&app.state.workbench.placement_query);
    let library_parts = library_part_rows(app, &query);
    let cells = cell_candidates(&app.state);
    let visible_matches = component_shelf_match_count(&app.state, &query) + library_parts.len();
    let mut band = None;
    let mut primitive = None;
    let mut builtin = None;
    let mut generated = None;
    let mut cell = None;
    let mut requested_part = None;
    // Every band and catalog below takes the session rather than the
    // application, so the borrow is narrowed once here instead of at each of
    // the seven calls. Only the arming below needs the application back.
    let state = &mut app.state;
    ScrollArea::vertical()
        .id_salt("workbench.design.component_shelf")
        .show(ui, |ui| {
            let pinned = pinned_band(ui, state, &library_parts, &cells);
            band = recent_band(ui, state, &library_parts, &cells).or(pinned);
            primitive = primitive_catalog(ui, state);
            builtin = builtin_xspice_catalog(ui, state);
            generated = generated_veriloga_catalog(ui, state);
            requested_part = library_parts_section(ui, state, &library_parts);
            cell = project_library(ui, state, &cells);
            if !query.is_empty() && visible_matches == 0 {
                empty_navigator_row(ui, "No component or cell matches this filter");
            }
            // Every fold position on the shelf is persisted under its own
            // group row, so the traversal moves all of them itself and has
            // nothing to hand back. Bound rather than discarded, so a row
            // added later that folds a navigator-tree node is caught here
            // instead of quietly answering no key.
            let _unmoved = rail::traverse(ui, enter_rows);
            debug_assert!(
                _unmoved.is_none(),
                "the shelf keeps every fold position under its own row"
            );
        });
    if let Some(arm) = band {
        apply_shelf_arm(app, arm, ui.ctx());
    } else if let Some(kind) = primitive {
        arm_primitive(app, kind, ui.ctx());
    } else if let Some(binding) = builtin {
        arm_cell(&mut app.state, binding, ui.ctx());
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
pub(super) fn library_part_rows(app: &RSpiceApp, query: &str) -> Vec<LibraryPartRow> {
    let libraries = app.state.model_library_manager.libraries_sorted();
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
    query: &str,
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
            // The third search field is the row's address: the pack for a
            // pack row, the holding library for a project one — the same
            // column the Models workspace shelf searches.
            let source = match &row.provenance {
                PartProvenance::InstalledPack { pack_id, .. }
                | PartProvenance::RemoteRelease { pack_id, .. } => pack_id.as_str(),
                PartProvenance::ProjectRetained { library } => library.as_str(),
                PartProvenance::Foundation => "",
            };
            matches_query(query, &[&row.part_id, &row.device, source])
        })
        .take(SHELF_ROWS)
        .filter_map(|row| library_part_row(libraries, row))
        .collect()
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
            let standing = if matches!(row.provenance, PartProvenance::Foundation) {
                "built in"
            } else {
                "in project"
            };
            (format!("{} · {standing}", row.device), action)
        }
        PartProvenance::InstalledPack { pack_id, version }
        | PartProvenance::RemoteRelease { pack_id, version } => {
            let pack_name = row.pack_name.clone().unwrap_or_else(|| pack_id.clone());
            let meta = match &row.state {
                PartState::Installed => format!("{} · installed", row.device),
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
    let query = normalized(&state.workbench.placement_query);
    let visible = if query.is_empty() {
        catalog_group_row(
            ui,
            "component-shelf-library-parts",
            ShelfGlyph::Icon(WorkbenchIcon::Models),
            "Library parts",
            rows.len(),
            false,
        )
    } else {
        shelf_section_header(ui, "Library parts", Some(&rows.len().to_string()));
        true
    };
    if !visible {
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
                    if query.is_empty() { 2 } else { 0 },
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
/// on the canvas side, so the toast and the canvas focus are the whole story.
fn arm_library_part(
    state: &mut crate::workbench::app_state::AppState,
    placement: PartPlacement,
    ctx: &egui::Context,
) {
    let armed = state.schematic.arm_pack_part(placement);
    crate::schematic::view::request_schematic_canvas_focus(ctx);
    state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{armed} will snap to the schematic grid."),
    );
}

/// Raises the pack confirmation for one shelf part.
///
/// The decision is shown in the Models workspace rather than over the canvas:
/// it commits the project to a licence, a download, and a capability claim,
/// and those are exactly what that workspace is for. The placement is armed
/// on the cursor when the install completes, so the round trip ends where the
/// user started.
fn request_library_part(app: &mut RSpiceApp, part_id: String, pack_id: String, version: String) {
    use crate::workbench::state::{ModelsPage, ModelsWorkbenchDialog, PackReleaseConfirmation};

    let Some(release) = PackReleaseConfirmation::for_release(
        &app.model_hub,
        &pack_id,
        &version,
        Some(part_id.clone()),
    ) else {
        app.state
            .push_user_message(crate::diagnostics::ConsoleMessage::warning(format!(
                "The model hub no longer describes {pack_id} {version}, so '{part_id}' cannot \
                 be added."
            )));
        return;
    };
    app.state.workbench.models_view.dialog = Some(ModelsWorkbenchDialog::ConfirmPack {
        pack_id,
        attach: true,
        release: Some(Box::new(release)),
    });
    crate::workbench::commands::vocabulary::Command::ModelsPage(ModelsPage::Models).execute(app);
}

pub(super) fn component_shelf_match_count(state: &AppState, query: &str) -> usize {
    let primitive_matches = PRIMITIVE_GROUPS
        .iter()
        .map(|(_, _, _, section_names)| {
            primitive_entries(section_names)
                .into_iter()
                .filter(|entry| matches_query(query, &[entry.label, entry.kind.display_name()]))
                .count()
        })
        .sum::<usize>();
    let library_matches = cell_candidates(state)
        .into_iter()
        .filter(|candidate| {
            matches_query(
                query,
                &[&candidate.library, &candidate.cell, &candidate.view],
            )
        })
        .count();
    let builtin_matches = engine_only_xspice_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                query,
                &[
                    descriptor.display_name,
                    descriptor.model_type,
                    descriptor.stable_id,
                ],
            )
        })
        .count();
    let generated_matches = generated_veriloga_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                query,
                &[
                    descriptor.model_name,
                    descriptor.module_name,
                    descriptor.source_digest,
                ],
            )
        })
        .count();
    primitive_matches + builtin_matches + generated_matches + library_matches
}

fn shelf_search(ui: &mut Ui, state: &mut AppState) -> bool {
    panel_search(
        ui,
        &mut state.workbench.placement_query,
        "workbench.design.component_shelf.search",
        "Place component or cell…",
        &mut state.workbench.focus_placement_search,
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
    /// A model-library part, by the id the unified index publishes.
    LibraryPart(String),
    /// A built-in XSPICE code model, by its registry stable id.
    BuiltinXspice(String),
    /// A build-time generated Verilog-A model, by its model name.
    GeneratedVerilogA(String),
    /// A project-library cell view.
    Cell {
        library: String,
        cell: String,
        view: String,
    },
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
            Self::LibraryPart(part) => format!("part{SHELF_ENTRY_FIELD}{part}"),
            Self::BuiltinXspice(id) => format!("xspice{SHELF_ENTRY_FIELD}{id}"),
            Self::GeneratedVerilogA(model) => format!("veriloga{SHELF_ENTRY_FIELD}{model}"),
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
            "part" => Self::LibraryPart(fields.next()?.to_owned()),
            "xspice" => Self::BuiltinXspice(fields.next()?.to_owned()),
            "veriloga" => Self::GeneratedVerilogA(fields.next()?.to_owned()),
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
        .ui
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
    let mut shelf = state.ui.preferences.component_shelf();
    shelf.pinned = Some(keys);
    state.ui.preferences.set_component_shelf(shelf);
}

/// Record one placement at the front of the history, deduplicated.
pub(super) fn record_placement(state: &mut AppState, entry: &ShelfEntry) {
    let key = entry.storage_key();
    let mut shelf = state.ui.preferences.component_shelf();
    shelf.recent.retain(|held| *held != key);
    shelf.recent.insert(0, key);
    shelf.recent.truncate(RECENT_STORED);
    state.ui.preferences.set_component_shelf(shelf);
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
    let Tool::Place(kind) = state.schematic.tool else {
        return None;
    };
    if kind == ComponentType::CellInstance {
        return state
            .schematic
            .pending_library_cell
            .as_ref()
            .map(ShelfEntry::from_binding);
    }
    // A native device armed from the model library carries that part's card,
    // and the card name is the part's own id in the unified index — so the
    // history names the part the reader picked, not its device family.
    if let Some(armed) = state.schematic.pending_part_model.as_ref()
        && armed.tool == Tool::Place(kind)
    {
        return Some(ShelfEntry::LibraryPart(armed.model.clone()));
    }
    Some(ShelfEntry::Primitive(kind))
}

/// The identity of a shelf row being dragged over the canvas right now.
fn dragged_shelf_entry(ctx: &egui::Context) -> Option<ShelfEntry> {
    let payload = egui::DragAndDrop::payload::<SchematicShelfDragPayload>(ctx)?;
    Some(match payload.as_ref() {
        SchematicShelfDragPayload::Primitive(kind) => ShelfEntry::Primitive(*kind),
        SchematicShelfDragPayload::LibraryCell(binding) => ShelfEntry::from_binding(binding),
    })
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
        state.design_execution_epoch,
        state.active_schematic_epoch,
        state.workspace.active_view.display_path(),
    );
    let components = state.schematic.components.len();
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
enum ShelfArm {
    Primitive(ComponentType),
    /// Held by stable id rather than by binding: a vector-port code model
    /// opens the placement dialog instead of arming, and that fork belongs
    /// with the catalog row's, not duplicated here.
    BuiltinXspice(String),
    Cell(Box<LibraryCellInstance>),
    Part(Box<LibraryPartRow>),
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
    let placing_cell = state.schematic.tool == Tool::Place(ComponentType::CellInstance);
    let pending = state.schematic.pending_library_cell.as_ref();
    match entry {
        ShelfEntry::Primitive(kind) => Some(ShelfEntryRow {
            entry: entry.clone(),
            glyph: primitive_shelf_glyph(*kind),
            label: kind.display_name().to_owned(),
            meta: primitive_shelf_meta(*kind),
            selected: state.schematic.tool == Tool::Place(*kind),
            arm: ShelfArm::Primitive(*kind),
        }),
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
                meta: Some(descriptor.model_type.to_owned()),
                selected: placing_cell
                    && pending
                        .and_then(|binding| binding.builtin_xspice.as_ref())
                        .is_some_and(|binding| binding.stable_id == *id),
                arm: ShelfArm::BuiltinXspice(id.clone()),
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
                label: descriptor.model_name.to_owned(),
                meta: Some(format!(
                    "{} pin \u{00b7} {}",
                    descriptor.terminals.len(),
                    descriptor.module_name
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
    // Search asks the catalog a question; the personal rail is not an answer
    // to it.
    if !normalized(&state.workbench.placement_query).is_empty() {
        return None;
    }
    let rows = pinned_keys(state)
        .iter()
        .filter_map(|key| ShelfEntry::from_storage_key(key))
        .filter_map(|entry| resolve_shelf_entry(state, library_parts, cells, &entry))
        .collect::<Vec<_>>();
    if rows.is_empty() {
        return None;
    }
    let shortcut = state.ui.preferences.shortcuts().resolved_label(
        crate::workbench::commands::vocabulary::Command::PlaceInstance,
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
    if !normalized(&state.workbench.placement_query).is_empty() {
        return None;
    }
    let pinned = pinned_keys(state);
    let rows = state
        .ui
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
        ShelfArm::Part(_) => None,
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
        ShelfArm::BuiltinXspice(id) => {
            if let Some(binding) = place_builtin_xspice(&mut app.state, &id) {
                arm_cell(&mut app.state, binding, ctx);
            }
        }
        ShelfArm::Cell(binding) => arm_cell(&mut app.state, *binding, ctx),
        ShelfArm::Part(row) => apply_library_part_row(app, *row, ctx),
    }
}

pub(super) fn primitive_catalog(ui: &mut Ui, state: &mut AppState) -> Option<ComponentType> {
    let query = normalized(&state.workbench.placement_query);
    let mut armed = None;
    let visible_count = PRIMITIVE_GROUPS
        .iter()
        .map(|(_, _, _, section_names)| {
            primitive_entries(section_names)
                .into_iter()
                .filter(|entry| matches_query(&query, &[entry.label, entry.kind.display_name()]))
                .count()
        })
        .sum::<usize>();
    if visible_count == 0 && !query.is_empty() {
        return None;
    }
    shelf_section_header(ui, "Primitives", Some(&visible_count.to_string()));
    for (group, glyph, open_default, section_names) in PRIMITIVE_GROUPS {
        let entries = primitive_entries(section_names)
            .into_iter()
            .filter(|entry| matches_query(&query, &[entry.label, entry.kind.display_name()]))
            .collect::<Vec<_>>();
        if entries.is_empty() {
            continue;
        }
        if query.is_empty() {
            if catalog_group_row(
                ui,
                ("component-shelf", group),
                glyph,
                group,
                entries.len(),
                open_default,
            ) {
                armed = primitive_rows(ui, state, &entries, 2).or(armed);
            }
        } else {
            shelf_section_header(ui, group, Some(&entries.len().to_string()));
            armed = primitive_rows(ui, state, &entries, 0).or(armed);
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

fn primitive_rows(
    ui: &mut Ui,
    state: &mut AppState,
    entries: &[ComponentPaletteEntry],
    level: usize,
) -> Option<ComponentType> {
    let mut armed = None;
    for entry in entries {
        let response = shelf_part_row(
            ui,
            primitive_shelf_glyph(entry.kind),
            entry.label,
            state.schematic.tool == Tool::Place(entry.kind),
            primitive_shelf_meta(entry.kind).as_deref(),
            level,
        );
        if let Some(payload) = SchematicShelfDragPayload::primitive(entry.kind) {
            response.dnd_set_drag_payload(payload);
        }
        if response.clicked() {
            armed = Some(entry.kind);
        }
        shelf_pin_context_menu(&response, state, &ShelfEntry::Primitive(entry.kind));
    }
    armed
}

fn builtin_xspice_catalog(ui: &mut Ui, state: &mut AppState) -> Option<LibraryCellInstance> {
    let query = normalized(&state.workbench.placement_query);
    let descriptors = engine_only_xspice_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                &query,
                &[
                    descriptor.display_name,
                    descriptor.model_type,
                    descriptor.stable_id,
                ],
            )
        })
        .collect::<Vec<_>>();
    if descriptors.is_empty() {
        return None;
    }

    let visible = if query.is_empty() {
        catalog_group_row(
            ui,
            "component-shelf-builtin-xspice",
            ShelfGlyph::Event,
            "Built-in XSPICE",
            descriptors.len(),
            false,
        )
    } else {
        shelf_section_header(ui, "Built-in XSPICE", Some(&descriptors.len().to_string()));
        true
    };
    if !visible {
        return None;
    }

    let mut armed = None;
    for descriptor in descriptors {
        let selected = state
            .schematic
            .pending_library_cell
            .as_ref()
            .and_then(|binding| binding.builtin_xspice.as_ref())
            .is_some_and(|binding| binding.stable_id == descriptor.stable_id)
            && state.schematic.tool == Tool::Place(ComponentType::CellInstance);
        let response = shelf_part_row(
            ui,
            ShelfGlyph::Event,
            descriptor.display_name,
            selected,
            Some(descriptor.model_type),
            if query.is_empty() { 2 } else { 0 },
        );
        let stable_id = descriptor.stable_id;
        match builtin_xspice_library_binding(descriptor) {
            Ok(binding) => {
                response
                    .dnd_set_drag_payload(SchematicShelfDragPayload::library_cell(binding.clone()));
                let clicked = response.clicked();
                shelf_pin_context_menu(
                    &response,
                    state,
                    &ShelfEntry::BuiltinXspice(stable_id.to_owned()),
                );
                if clicked {
                    armed = place_builtin_xspice(state, stable_id).or(armed);
                }
            }
            Err(error) => {
                log::error!("Cannot expose {stable_id} in the XSPICE catalog: {error}");
            }
        }
    }
    armed
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
            state.dialogs.builtin_xspice_placement.open(
                descriptor.stable_id,
                descriptor.display_name,
                vector_ports,
                state.design_execution_epoch,
                state.active_schematic_epoch,
                state.workspace.active_view.display_path(),
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

fn generated_veriloga_catalog(ui: &mut Ui, state: &mut AppState) -> Option<LibraryCellInstance> {
    let query = normalized(&state.workbench.placement_query);
    let descriptors = generated_veriloga_devices()
        .iter()
        .filter(|descriptor| {
            matches_query(
                &query,
                &[
                    descriptor.model_name,
                    descriptor.module_name,
                    descriptor.source_digest,
                ],
            )
        })
        .collect::<Vec<_>>();
    if descriptors.is_empty() {
        return None;
    }
    let visible = if query.is_empty() {
        catalog_group_row(
            ui,
            "component-shelf-generated-veriloga",
            ShelfGlyph::Text("VA"),
            "Generated Verilog-A",
            descriptors.len(),
            false,
        )
    } else {
        shelf_section_header(
            ui,
            "Generated Verilog-A",
            Some(&descriptors.len().to_string()),
        );
        true
    };
    if !visible {
        return None;
    }

    let mut armed = None;
    for descriptor in descriptors {
        let selected = state
            .schematic
            .pending_library_cell
            .as_ref()
            .and_then(|binding| binding.generated_veriloga.as_ref())
            .is_some_and(|binding| binding.model_name == descriptor.model_name)
            && state.schematic.tool == Tool::Place(ComponentType::CellInstance);
        let response = shelf_part_row(
            ui,
            ShelfGlyph::Text("VA"),
            descriptor.model_name,
            selected,
            Some(&format!(
                "{} pin · {}",
                descriptor.terminals.len(),
                descriptor.module_name
            )),
            if query.is_empty() { 2 } else { 0 },
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
    let query = normalized(&state.workbench.placement_query);
    let mut grouped = BTreeMap::<String, Vec<CellCandidate>>::new();
    for candidate in cells.iter().cloned() {
        if matches_query(
            &query,
            &[&candidate.library, &candidate.cell, &candidate.view],
        ) {
            grouped
                .entry(candidate.library.clone())
                .or_default()
                .push(candidate);
        }
    }
    if grouped.is_empty() && !query.is_empty() {
        return None;
    }
    shelf_section_header(ui, "Project library", None);
    let mut armed = None;
    for (library, grouped_cells) in grouped {
        if query.is_empty() {
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
        } else {
            shelf_section_header(ui, &library, Some(&grouped_cells.len().to_string()));
            armed = cell_rows(ui, state, &grouped_cells, 0).or(armed);
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

fn cell_candidates(state: &AppState) -> Vec<CellCandidate> {
    library_cell_placement_candidates(&state.library_manager, &state.workspace)
}

pub(super) fn arm_primitive(app: &mut RSpiceApp, kind: ComponentType, ctx: &egui::Context) {
    if kind == ComponentType::Port {
        crate::workbench::commands::vocabulary::Command::PlacePin.execute(app);
        return;
    }
    let state = &mut app.state;
    state.schematic.pending_library_cell = None;
    state.schematic.arm_tool(Tool::Place(kind));
    state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{} will snap to the schematic grid.", kind.display_name()),
    );
    crate::schematic::view::request_schematic_canvas_focus(ctx);
}

/// Arms one library cell for placement.
///
/// It takes the session state rather than the application because that is all
/// arming touches: a pending binding, a tool, a toast, and the canvas focus.
fn arm_cell(
    state: &mut crate::workbench::app_state::AppState,
    binding: LibraryCellInstance,
    ctx: &egui::Context,
) {
    let label = format!("{}/{}", binding.library, binding.cell);
    state.schematic.pending_library_cell = Some(binding);
    state
        .schematic
        .arm_tool(Tool::Place(ComponentType::CellInstance));
    state.ui.toasts.success(
        ctx,
        "Component placement armed",
        format!("{label} will snap to the schematic grid."),
    );
    crate::schematic::view::request_schematic_canvas_focus(ctx);
}

pub(super) fn primitive_entries(section_names: &[&str]) -> Vec<ComponentPaletteEntry> {
    component_palette()
        .iter()
        .filter(|section| section_names.contains(&section.title))
        .flat_map(|section| section.entries.iter().copied())
        .collect()
}

#[cfg(test)]
pub(super) fn primitive_entry_count() -> usize {
    component_palette()
        .iter()
        .map(|section| section.entries.len())
        .sum()
}

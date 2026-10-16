//! Symbol inspector presentation and source-bound edit drafts.

use super::interaction::{SymbolEditCapabilities, SymbolRequestSource};
use super::session::{SymbolEditorSession, SymbolSelection};
use egui::Ui;
use rspice_design::symbol::edit::SymbolCommitIntent;
use rspice_design::symbol::{
    MAX_SYMBOL_PIN_NAME_BYTES, MAX_SYMBOL_TEXT_BYTES, SymbolAttributeKind, SymbolDocument,
    SymbolEditorMetadata, SymbolPin, SymbolPinElectricalKind, SymbolShape, SymbolTextAlign,
    SymbolTextSize,
};
use rspice_design_model::{
    Point, cell_view::CellViewRef, port::PortSpec, symbol_pin::SymbolPinSide,
};
use rspice_ui_kit::panels::{
    PANEL_INSET, Problem, WorkbenchIcon, labeled_icon_button, panel_bleed, panel_inset,
    property_row, property_row_combo, property_row_input, property_row_problem, section_block_gap,
    section_header,
};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};

/// The field-edit lifetime. Selection here identifies the draft, not a second selection owner.
#[derive(Debug, Clone, Default)]
pub struct SymbolInspectorSession {
    source: Option<SymbolRequestSource>,
    selection: SymbolSelection,
    scope: Option<egui::Id>,
    generation: u64,
    undo_recorded: bool,
}

impl SymbolInspectorSession {
    fn bind(&mut self, source: &SymbolRequestSource, selection: &SymbolSelection) {
        if self.source.as_ref() != Some(source) || &self.selection != selection {
            self.scope = Some(egui::Id::new((
                source.project,
                &source.document,
                source.design_epoch,
                source.document_epoch,
                source.window,
            )));
            self.generation = self.generation.wrapping_add(1);
            self.undo_recorded = false;
            self.source = Some(source.clone());
            self.selection = selection.clone();
        }
    }

    /// A successful host commit advances this field's source without splitting its undo group.
    pub fn accept_source(&mut self, source: SymbolRequestSource) {
        self.source = Some(source);
    }

    /// User navigation ends a field's focus lifetime and its undo group.
    pub fn end_field_edit(&mut self) {
        self.source = None;
        self.undo_recorded = false;
    }

    pub(super) fn resume_source(&mut self, source: &SymbolRequestSource) {
        if let Some(previous) = &self.source {
            let mut resumed = previous.clone();
            resumed.document_epoch = source.document_epoch;
            if &resumed == source {
                self.source = Some(resumed);
            }
        }
    }
}

/// The host validates source and selection before recording history or publishing the draft.
#[derive(Debug)]
pub struct SymbolInspectorRequest {
    pub source: SymbolRequestSource,
    pub expected_selection: SymbolSelection,
    pub selection: SymbolSelection,
    pub session: SymbolInspectorSession,
    pub document: SymbolDocument,
    pub metadata: SymbolEditorMetadata,
    pub intent: SymbolCommitIntent,
    pub undo_before: Vec<SymbolDocument>,
    pub changed: bool,
    pub open_parameter_form: bool,
}

pub fn show(
    ui: &mut Ui,
    source: SymbolRequestSource,
    mut document: SymbolDocument,
    mut metadata: SymbolEditorMetadata,
    ports: &[PortSpec],
    editor: &SymbolEditorSession,
    capabilities: SymbolEditCapabilities,
) -> SymbolInspectorRequest {
    let expected_selection = editor.effective_selection();
    let mut edit = InspectorEdit {
        selection: expected_selection.clone(),
        session: editor.inspector.clone(),
        undo_before: Vec::new(),
        open_parameter_form: false,
    };
    edit.session.bind(&source, &expected_selection);
    let mut changed = false;
    let mut intent = SymbolCommitIntent::default();
    let field_id = (
        "symbol-inspector",
        edit.session.scope,
        edit.session.generation,
    );
    ui.push_id(field_id, |ui| {
        hero(ui, &source.document, &document, ports);
        ui.add_enabled_ui(capabilities.edit, |ui| {
            let selection = edit.selection.clone();
            let total = selection.len();
            // Nothing selected draws nothing: the hero and the definition
            // section are what the panel has to say about the symbol itself.
            if total > 1 {
                multi_selection_section(
                    ui,
                    selection.pins.len(),
                    selection.shapes.len(),
                    selection.attributes.len(),
                );
            } else if let Some(name) = selection.pins.iter().next() {
                changed |= pin_section(ui, &mut edit, &mut document, &mut intent, ports, name);
            } else if let Some(index) = selection.shapes.iter().next() {
                changed |= shape_section(ui, &mut edit, &mut document, *index);
            } else if let Some(kind) = selection.attributes.iter().next() {
                changed |= attribute_section(ui, &mut edit, &mut document, &mut metadata, *kind);
            }
        });
        // The pin list is the navigator's, including the ports no pin was
        // placed for: one list, in one place, rather than the same numbered
        // pins with the same marks on two panels at once.
        definition_section(ui, &mut edit);
    });
    edit.session.selection = edit.selection.clone();
    SymbolInspectorRequest {
        source,
        expected_selection,
        selection: edit.selection,
        session: edit.session,
        document,
        metadata,
        intent,
        undo_before: edit.undo_before,
        changed,
        open_parameter_form: edit.open_parameter_form,
    }
}

struct InspectorEdit {
    selection: SymbolSelection,
    session: SymbolInspectorSession,
    undo_before: Vec<SymbolDocument>,
    open_parameter_form: bool,
}

impl InspectorEdit {
    fn record_undo(&mut self, document: &SymbolDocument) {
        self.undo_before.push(document.clone());
    }
    fn select_pin(&mut self, name: impl Into<String>) {
        self.selection = SymbolSelection::single_pin(name);
    }
    fn clear_selection(&mut self) {
        self.selection = SymbolSelection::default();
    }
}

// =============================================================================
// Hero
// =============================================================================

fn hero(ui: &mut Ui, reference: &CellViewRef, document: &SymbolDocument, ports: &[PortSpec]) {
    let t = Tokens::get(ui.ctx());
    let inset = panel_inset(ui);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 64.0), egui::Sense::hover());
    // The band reaches the panel's edges; its text keeps the panel inset on
    // the right.
    let band = panel_bleed(ui, rect);
    let preview = egui::Rect::from_min_max(
        band.min,
        egui::pos2((band.left() + 82.0).min(band.right()), band.bottom()),
    );
    ui.painter().rect_filled(preview, 0.0, t.color.canvas_bg);
    ui.painter().vline(
        preview.right(),
        preview.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        band.x_range(),
        band.bottom(),
        egui::Stroke::new(1.0, t.color.border),
    );
    super::draw_document_preview(
        ui.painter(),
        preview.shrink(14.0),
        document,
        ports,
        &reference.cell,
        t.color.symbol,
    );

    let text_left = preview.right() + PANEL_INSET;
    let painter = ui.painter().with_clip_rect(egui::Rect::from_x_y_ranges(
        text_left..=(rect.right() - inset).max(text_left),
        rect.y_range(),
    ));
    let at = |y: f32| egui::pos2(text_left, rect.top() + y);
    painter.text(
        at(12.0),
        egui::Align2::LEFT_CENTER,
        format!(
            "{} / {} · {}",
            reference.library,
            reference.cell,
            reference.view.to_ascii_uppercase()
        ),
        theme::mono(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    painter.text(
        at(31.0),
        egui::Align2::LEFT_CENTER,
        &reference.cell,
        theme::sans(tokens::FS_2, FontWeight::SemiBold),
        t.color.text,
    );
    painter.text(
        at(49.0),
        egui::Align2::LEFT_CENTER,
        format!(
            "{} pins · {} body shapes",
            document.pins.len(),
            document.body.len()
        ),
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
}

// =============================================================================
// Selection sections
// =============================================================================

/// Record one undo snapshot for the open inspector field, however many
/// keystrokes it receives.
fn record_once(edit: &mut InspectorEdit, before: &SymbolDocument) {
    if edit.session.undo_recorded {
        return;
    }
    edit.record_undo(before);
    edit.session.undo_recorded = true;
}

/// An editable integer coordinate. Illegal text is simply not applied; the
/// row keeps showing the value the document holds.
fn coordinate_row(
    ui: &mut Ui,
    edit: &mut InspectorEdit,
    before: &SymbolDocument,
    label: &str,
    value: &mut i32,
) -> bool {
    let id = ui.id().with(("symbol-coordinate", label));
    let mut buffer = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| value.to_string());
    let parsed = buffer.trim().parse::<i32>();
    let response = property_row_input(ui, label, &mut buffer, parsed.is_err());
    let mut changed = false;
    if response.changed() {
        ui.data_mut(|data| data.insert_temp(id, buffer.clone()));
        if let Ok(parsed) = buffer.trim().parse::<i32>()
            && parsed != *value
        {
            record_once(edit, before);
            *value = parsed;
            changed = true;
        }
    }
    if response.lost_focus() {
        // The row shows the document's value again the moment the field is
        // not being typed into, and the next session starts a new undo step.
        ui.data_mut(|data| data.remove_temp::<String>(id));
        edit.session.undo_recorded = false;
    }
    changed
}

fn text_row(
    ui: &mut Ui,
    edit: &mut InspectorEdit,
    before: &SymbolDocument,
    id_salt: impl std::hash::Hash + std::fmt::Debug,
    label: &str,
    value: &str,
    is_valid: impl Fn(&str) -> bool,
) -> Option<String> {
    let id = ui.id().with(("symbol-text", id_salt));
    let mut buffer = ui
        .data_mut(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| value.to_owned());
    let valid = is_valid(buffer.trim());
    let response = property_row_input(ui, label, &mut buffer, !valid);
    let mut accepted = None;
    if response.changed() {
        ui.data_mut(|data| data.insert_temp(id, buffer.clone()));
        let trimmed = buffer.trim();
        if valid && trimmed != value {
            record_once(edit, before);
            accepted = Some(trimmed.to_owned());
        }
    }
    if response.lost_focus() {
        ui.data_mut(|data| data.remove_temp::<String>(id));
        edit.session.undo_recorded = false;
    }
    accepted
}

/// A pin's name IS its width, so the row that states the width points at the
/// field that changes it instead of offering a second one that could disagree.
const PIN_WIDTH_HINT: &str =
    "The name declares the width. Rename the pin to change the range it carries.";

/// The width row's value for one pin, or `None` when the pin declares no
/// range.
///
/// One bit is what every name that is not a range carries, so a row stating it
/// would be a row on every pin. The width is read from [`SymbolPin::vector`],
/// which reads the name — the form cannot show a width the netlister
/// disagrees with. The range itself is not repeated here: it is the Name field
/// directly above.
fn pin_width_row(pin: &SymbolPin) -> Option<String> {
    Some(format!("{} bits", pin.vector()?.width()))
}

/// What the integers on this panel's coordinate rows are counted in. The
/// terminal pitch is ten of them, which is what the canvas grid shows.
const COORDINATE_UNIT: &str = "grid units";

/// The pin's place in netlist order, and the control that changes it.
///
/// One row on the property grid rather than a pair of buttons below it: the
/// order *is* a property of the pin, the two buttons that used to carry it sat
/// off the grid in a line of their own, and moving a terminal from last to
/// first took one press per place it passed.
///
/// Returns the index the pin is to take.
fn netlist_order_row(ui: &mut Ui, order: usize, total: usize) -> Option<usize> {
    let mut selected = (order + 1).to_string();
    let options = (1..=total)
        .map(|position| (position.to_string(), format!("{position} of {total}")))
        .collect::<Vec<_>>();
    let changed = property_row_combo(
        ui,
        "Netlist order",
        ("symbol-pin-order", order),
        &mut selected,
        &options,
        total > 1,
    );
    let picked = selected.parse::<usize>().ok()?.checked_sub(1)?;
    (changed && picked != order && picked < total).then_some(picked)
}

fn pin_section(
    ui: &mut Ui,
    edit: &mut InspectorEdit,
    document: &mut SymbolDocument,
    intent: &mut SymbolCommitIntent,
    ports: &[PortSpec],
    name: &str,
) -> bool {
    let Some(order) = document
        .pins
        .iter()
        .position(|pin| pin.name.eq_ignore_ascii_case(name))
    else {
        return false;
    };
    let before = document.clone();
    let total = document.pins.len();
    let current_name = document.pins[order].name.clone();

    // The meta carries the unit the coordinate row below has no room to
    // state, in place of a port count the Netlist order row already gives.
    section_header(ui, "Selected pin", Some(COORDINATE_UNIT));

    let mut changed = false;
    if let Some(new_name) =
        text_row(
            ui,
            edit,
            &before,
            ("pin-name", order),
            "Name",
            &current_name,
            |candidate| {
                !candidate.is_empty()
                    && candidate.len() <= MAX_SYMBOL_PIN_NAME_BYTES
                    && !candidate.chars().any(char::is_control)
                    && document.pins.iter().enumerate().all(|(index, pin)| {
                        index == order || !pin.name.eq_ignore_ascii_case(candidate)
                    })
            },
        )
    {
        // Declared, not inferred: the commit carries the old name so every
        // placed instance's terminal follows the pin instead of detaching
        // from a name that no longer exists.
        intent
            .renames
            .insert(current_name.clone(), new_name.clone());
        document.pins[order].name = new_name.clone();
        edit.select_pin(new_name);
        changed = true;
    }

    if let Some(width) = pin_width_row(&document.pins[order]) {
        property_row(ui, "Width", &width).on_hover_text(PIN_WIDTH_HINT);
    }

    let mut electrical = SymbolPinElectricalKind::from_pin(&document.pins[order])
        .label()
        .to_owned();
    let electrical_options = SymbolPinElectricalKind::ALL
        .into_iter()
        .map(|kind| (kind.label().to_owned(), kind.label().to_owned()))
        .collect::<Vec<_>>();
    if property_row_combo(
        ui,
        "Electrical type",
        ("symbol-pin-electrical", order),
        &mut electrical,
        &electrical_options,
        true,
    ) && let Some(kind) = SymbolPinElectricalKind::ALL
        .into_iter()
        .find(|kind| kind.label() == electrical)
    {
        record_once(edit, &before);
        let (electrical_type, direction) = kind.contract();
        document.pins[order].set_electrical_contract(electrical_type, direction);
        changed = true;
    }

    let mut side = document.pins[order].side();
    let mut side_value = side.label().to_owned();
    let side_options = SymbolPinSide::ALL
        .into_iter()
        .map(|side| (side.label().to_owned(), side.label().to_owned()))
        .collect::<Vec<_>>();
    let side_changed = property_row_combo(
        ui,
        "Side",
        ("symbol-pin-side", order),
        &mut side_value,
        &side_options,
        true,
    );
    if side_changed
        && let Some(new_side) = SymbolPinSide::ALL
            .into_iter()
            .find(|candidate| candidate.label() == side_value)
    {
        record_once(edit, &before);
        side = new_side;
        changed = true;
    }

    let mut offset = document.pins[order].offset();
    let offset_changed = coordinate_row(ui, edit, &before, "Offset", &mut offset);
    if side_changed || offset_changed {
        let bounds = document.body_bounds();
        document.pins[order].set_side_and_offset(side, offset, bounds);
        changed = true;
    }

    if let Some(moved) = netlist_order_row(ui, order, total) {
        edit.record_undo(&before);
        let pin = document.pins.remove(order);
        document.pins.insert(moved, pin);
        edit.session.undo_recorded = false;
        changed = true;
    }

    let active_name = document.pins[order].name.clone();
    // Both rows state a mismatch and nothing else. A pin the interface
    // declares, placed on the terminal grid, is the state every pin is in
    // while nothing is wrong, and a row saying so is a row on every pin.
    if !ports
        .iter()
        .any(|port| port.name.eq_ignore_ascii_case(&active_name))
        && !ports.is_empty()
    {
        property_row_problem(ui, "Interface", "not in schematic", Some(Problem::Warning));
    }
    let placed = document.pins[order].position.is_some();
    let on_grid = document.pins[order].terminal_on_grid();
    if !placed || !on_grid {
        property_row_problem(
            ui,
            "Terminal",
            if placed { "off grid" } else { "unplaced" },
            Some(Problem::Warning),
        );
    }
    if !placed
        && labeled_icon_button(
            ui,
            WorkbenchIcon::Target,
            "Place on selected side",
            false,
            ui.available_width(),
        )
        .clicked()
    {
        edit.record_undo(&before);
        let bounds = document.body_bounds();
        document.pins[order].set_side_and_offset(side, offset, bounds);
        edit.session.undo_recorded = false;
        changed = true;
    }
    section_block_gap(ui);
    if ui
        .add_sized(
            [ui.available_width(), Tokens::get(ui.ctx()).metrics.ctl_h],
            egui::Button::new("Remove pin"),
        )
        .clicked()
    {
        edit.record_undo(&before);
        document.pins.remove(order);
        edit.clear_selection();
        edit.session.undo_recorded = false;
        changed = true;
    }
    changed
}

/// The four orientations an arrow head takes, in quarter turns.
const ARROW_ROTATIONS: [i32; 4] = [0, 1, 2, 3];

/// A quarter-turn count in the degrees a designer reads it in.
fn arrow_rotation_label(quarters: i32) -> String {
    format!("{}\u{b0}", quarters.rem_euclid(4) * 90)
}

fn shape_kind(shape: &SymbolShape) -> &'static str {
    match shape {
        SymbolShape::Polyline { closed: true, .. } => "polygon",
        SymbolShape::Polyline { .. } => "polyline",
        SymbolShape::Circle { .. } => "circle",
        SymbolShape::Arc { .. } => "arc",
        SymbolShape::Arrow { .. } => "arrow",
        SymbolShape::Dot { .. } => "dot",
        SymbolShape::Text { .. } => "text",
    }
}

/// The selected body shape. Body geometry is cosmetic: it never changes the
/// port contract, which the section says outright.
fn shape_section(
    ui: &mut Ui,
    edit: &mut InspectorEdit,
    document: &mut SymbolDocument,
    index: usize,
) -> bool {
    let Some(shape) = document.body.get(index).cloned() else {
        return false;
    };
    let before = document.clone();
    section_header(
        ui,
        "Selected shape",
        Some(&format!("{} · {COORDINATE_UNIT}", shape_kind(&shape))),
    );

    let mut changed = false;
    let mut edited = shape;
    match &mut edited {
        SymbolShape::Polyline { points, closed: _ } => {
            if let Some(first) = points.first_mut() {
                let mut anchor = *first;
                // "Position", not "Start": editing either moves the whole
                // shape, which is what the delta below applies.
                let moved = coordinate_row(ui, edit, &before, "Position X", &mut anchor.x)
                    | coordinate_row(ui, edit, &before, "Position Y", &mut anchor.y);
                if moved {
                    let delta = Point::new(anchor.x - first.x, anchor.y - first.y);
                    for point in points.iter_mut() {
                        *point = *point + delta;
                    }
                    changed = true;
                }
            }
        }
        SymbolShape::Circle { center, radius } | SymbolShape::Dot { center, radius } => {
            changed |= coordinate_row(ui, edit, &before, "Center X", &mut center.x);
            changed |= coordinate_row(ui, edit, &before, "Center Y", &mut center.y);
            changed |= coordinate_row(ui, edit, &before, "Radius", radius);
        }
        SymbolShape::Arc {
            center,
            radius,
            start_degrees,
            sweep_degrees,
        } => {
            changed |= coordinate_row(ui, edit, &before, "Center X", &mut center.x);
            changed |= coordinate_row(ui, edit, &before, "Center Y", &mut center.y);
            changed |= coordinate_row(ui, edit, &before, "Radius", radius);
            changed |= coordinate_row(ui, edit, &before, "Start °", start_degrees);
            changed |= coordinate_row(ui, edit, &before, "Sweep °", sweep_degrees);
        }
        SymbolShape::Arrow {
            tip,
            rotation_quarters,
        } => {
            changed |= coordinate_row(ui, edit, &before, "Tip X", &mut tip.x);
            changed |= coordinate_row(ui, edit, &before, "Tip Y", &mut tip.y);
            // Degrees, in the four the shape has: a quarter turn is a unit
            // nothing else on a schematic is measured in.
            let mut rotation = arrow_rotation_label(*rotation_quarters);
            if property_row_combo(
                ui,
                "Rotation",
                ("symbol-arrow-rotation", index),
                &mut rotation,
                &ARROW_ROTATIONS
                    .iter()
                    .map(|degrees| {
                        (
                            arrow_rotation_label(*degrees),
                            arrow_rotation_label(*degrees),
                        )
                    })
                    .collect::<Vec<_>>(),
                true,
            ) && let Some(quarters) = ARROW_ROTATIONS
                .iter()
                .find(|degrees| arrow_rotation_label(**degrees) == rotation)
            {
                record_once(edit, &before);
                *rotation_quarters = *quarters;
                changed = true;
            }
        }
        SymbolShape::Text {
            anchor,
            text,
            size,
            align,
        } => {
            if let Some(value) = text_row(
                ui,
                edit,
                &before,
                ("symbol-body-text", index),
                "Text",
                text,
                |value| {
                    value.len() <= MAX_SYMBOL_TEXT_BYTES && !value.chars().any(char::is_control)
                },
            ) {
                *text = value;
                changed = true;
            }
            let mut size_value = size.label().to_owned();
            let size_options = SymbolTextSize::ALL
                .into_iter()
                .map(|option| (option.label().to_owned(), option.label().to_owned()))
                .collect::<Vec<_>>();
            if property_row_combo(
                ui,
                "Size",
                ("symbol-text-size", index),
                &mut size_value,
                &size_options,
                true,
            ) && let Some(picked) = SymbolTextSize::ALL
                .into_iter()
                .find(|option| option.label() == size_value)
            {
                record_once(edit, &before);
                *size = picked;
                changed = true;
            }
            let mut align_value = align.label().to_owned();
            let align_options = SymbolTextAlign::ALL
                .into_iter()
                .map(|option| (option.label().to_owned(), option.label().to_owned()))
                .collect::<Vec<_>>();
            if property_row_combo(
                ui,
                "Align",
                ("symbol-text-align", index),
                &mut align_value,
                &align_options,
                true,
            ) && let Some(picked) = SymbolTextAlign::ALL
                .into_iter()
                .find(|option| option.label() == align_value)
            {
                record_once(edit, &before);
                *align = picked;
                changed = true;
            }
            changed |= coordinate_row(ui, edit, &before, "Anchor X", &mut anchor.x);
            changed |= coordinate_row(ui, edit, &before, "Anchor Y", &mut anchor.y);
        }
    }
    if changed {
        document.body[index] = edited;
    }
    changed
}

fn attribute_section(
    ui: &mut Ui,
    edit: &mut InspectorEdit,
    document: &mut SymbolDocument,
    metadata: &mut SymbolEditorMetadata,
    kind: SymbolAttributeKind,
) -> bool {
    let Some(index) = metadata
        .attributes
        .iter()
        .position(|attribute| attribute.kind == kind)
    else {
        return false;
    };
    let before = document.clone();
    let mut changed = false;
    section_header(
        ui,
        "Selected attribute",
        Some(&format!("{} · {COORDINATE_UNIT}", kind.key())),
    );

    let current_value = metadata.attributes[index].default_value.clone();
    if let Some(value) = text_row(
        ui,
        edit,
        &before,
        ("attribute-value", kind),
        "Default value",
        &current_value,
        |value| value.len() <= MAX_SYMBOL_TEXT_BYTES && !value.chars().any(char::is_control),
    ) {
        metadata.attributes[index].default_value = value;
        changed = true;
    }

    // One control with one label. The row used to be a hand-built pair off
    // the grid whose tick box re-labelled itself "Shown" / "Hidden", saying
    // twice what the tick already said.
    let mut shown = metadata.attributes[index].shown;
    if rspice_ui_kit::widgets::switch_row(ui, "Show on instances", &mut shown) {
        record_once(edit, &before);
        metadata.attributes[index].shown = shown;
        changed = true;
    }

    let mut position = metadata.attributes[index].position;
    let moved = coordinate_row(ui, edit, &before, "Position X", &mut position.x)
        | coordinate_row(ui, edit, &before, "Position Y", &mut position.y);
    if moved {
        metadata.attributes[index].position = position;
        match kind {
            SymbolAttributeKind::Reference => document.name_anchor = position,
            SymbolAttributeKind::Value => document.value_anchor = position,
            SymbolAttributeKind::Model => {}
        }
        changed = true;
    }
    changed
}

/// More than one object: the count, and nothing else. Which kinds they are is
/// in the navigator's lists, and the instruction that used to sit here told a
/// reader to do what the next click does anyway.
fn multi_selection_section(ui: &mut Ui, pins: usize, shapes: usize, attributes: usize) {
    let total = pins + shapes + attributes;
    section_header(ui, "Selection", Some(&format!("{total} selected")));
}

fn definition_section(ui: &mut Ui, edit: &mut InspectorEdit) {
    section_header(ui, "Symbol definition", None);
    if labeled_icon_button(
        ui,
        WorkbenchIcon::Sliders,
        "Edit parameter form...",
        false,
        ui.available_width(),
    )
    .clicked()
    {
        edit.open_parameter_form = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::symbol::SymbolPin;
    use rspice_design_model::port::PortDirection;

    /// The row is `declared_width` in words: the name decides the width, and
    /// the form has no second answer. A pin that declares no range shows no
    /// row rather than one stating the obvious, and the row never repeats the
    /// range itself, which is the Name field directly above it.
    #[test]
    fn the_pin_width_row_states_what_the_name_declares() {
        for (name, expected) in [("DATA[3:0]", "4 bits"), ("ADDR<0:7>", "8 bits")] {
            let pin = SymbolPin::new(name, PortDirection::InOut, None);
            assert_eq!(pin_width_row(&pin).as_deref(), Some(expected));
            assert_eq!(pin.width(), rspice_design_model::bus::declared_width(name));
        }

        for scalar in ["EN", "DATA[3]", "bias_1"] {
            let pin = SymbolPin::new(scalar, PortDirection::In, None);
            assert_eq!(pin_width_row(&pin), None, "{scalar}");
            assert_eq!(pin.width(), 1, "{scalar}");
        }
    }

    /// An arrow's orientation reads in degrees, which is how every other
    /// rotation in the editor reads. "Quarter turns" was the stored unit
    /// shown raw, and the four values it takes are the whole range.
    #[test]
    fn an_arrows_rotation_reads_in_degrees() {
        assert_eq!(
            ARROW_ROTATIONS.map(arrow_rotation_label).to_vec(),
            ["0\u{b0}", "90\u{b0}", "180\u{b0}", "270\u{b0}"]
        );
        // A document written with a turn count outside the range still lands
        // on one of the four the combo offers.
        assert_eq!(arrow_rotation_label(5), "90\u{b0}");
        assert_eq!(arrow_rotation_label(-1), "270\u{b0}");
    }
}

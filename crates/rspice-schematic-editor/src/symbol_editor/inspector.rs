//! Symbol inspector presentation and source-bound edit drafts.

use super::interaction::{SymbolEditCapabilities, SymbolRequestSource};
use super::session::{SymbolEditorSession, SymbolSelection};
use egui::Ui;
use rspice_design::symbol::edit::SymbolCommitIntent;
use rspice_design::symbol::{
    MAX_SYMBOL_PIN_NAME_BYTES, MAX_SYMBOL_TEXT_BYTES, PinSummary, SymbolAttributeKind,
    SymbolDocument, SymbolEditorMetadata, SymbolPin, SymbolPinElectricalKind, SymbolShape,
    SymbolTextAlign, SymbolTextSize,
};
use rspice_design_model::{
    Point, cell_view::CellViewRef, port::PortSpec, symbol_pin::SymbolPinSide,
};
use rspice_ui_kit::accessibility::plural_suffix;
use rspice_ui_kit::panels::inspector::{muted_inspector_copy, section_header};
use rspice_ui_kit::panels::{
    StatusMark, WorkbenchIcon, labeled_icon_button, property_row, property_row_combo,
    property_row_input, property_row_status,
};
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::TreeRow;

/// The field-edit lifetime. Selection here identifies the draft, not a second selection owner.
#[derive(Debug, Clone, Default)]
pub struct SymbolInspectorSession {
    source: Option<SymbolRequestSource>,
    selection: SymbolSelection,
    generation: u64,
    undo_recorded: bool,
}

impl SymbolInspectorSession {
    fn bind(&mut self, source: &SymbolRequestSource, selection: &SymbolSelection) {
        if self.source.as_ref() != Some(source) || &self.selection != selection {
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
    ui.push_id(("symbol-inspector", edit.session.generation), |ui| {
        hero(ui, &source.document, &document, ports);
        ui.add_enabled_ui(capabilities.edit, |ui| {
            let selection = edit.selection.clone();
            let total = selection.len();
            if total == 0 {
                empty_selection_section(ui);
            } else if total > 1 {
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
        contract_section(ui, &mut edit, &document, ports);
        definition_section(ui, &mut edit, &metadata);
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
    let summary = document.pin_summary(ports);
    let (rect, _) =
        ui.allocate_exact_size(egui::vec2(ui.available_width(), 82.0), egui::Sense::hover());
    let preview = egui::Rect::from_min_max(
        rect.min,
        egui::pos2((rect.left() + 82.0).min(rect.right()), rect.bottom()),
    );
    ui.painter().rect_filled(preview, 0.0, t.color.canvas_bg);
    ui.painter().vline(
        preview.right(),
        preview.y_range(),
        egui::Stroke::new(1.0, t.color.border),
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
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

    let text_left = preview.right() + 10.0;
    let painter = ui.painter().with_clip_rect(egui::Rect::from_x_y_ranges(
        text_left..=(rect.right() - 10.0),
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
    let (status, tone) = summary_status(&t, summary);
    painter.text(
        at(68.0),
        egui::Align2::LEFT_CENTER,
        status,
        theme::mono(tokens::FS_0, FontWeight::Regular),
        tone,
    );
}

fn summary_status(t: &Tokens, summary: PinSummary) -> (String, egui::Color32) {
    match summary {
        PinSummary::Match => ("pin contract matches the interface".to_owned(), t.color.ok),
        PinSummary::Unplaced(count) => (
            format!("{count} declared pin{} unplaced", plural_suffix(count)),
            t.color.err,
        ),
        PinSummary::Orphaned(count) => (
            format!(
                "{count} pin{} not declared by the interface",
                plural_suffix(count)
            ),
            t.color.err,
        ),
        PinSummary::NoSchematic => (
            "no schematic interface declares this cell".to_owned(),
            t.color.warn,
        ),
    }
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
/// One conductor is what every name that is not a range carries, so a row
/// stating it would be a row on every pin. The width and the declaration are
/// both read from [`SymbolPin::vector`], which reads the name —
/// the form cannot show a width the netlister disagrees with.
fn pin_width_row(pin: &SymbolPin) -> Option<String> {
    let declaration = pin.vector()?;
    Some(format!(
        "{} conductors \u{2014} {declaration}",
        declaration.width()
    ))
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
        empty_selection_section(ui);
        return false;
    };
    let before = document.clone();
    let total = document.pins.len();
    let current_name = document.pins[order].name.clone();

    section_header(
        ui,
        "Selected pin",
        Some(&format!("port {} / {total}", order + 1)),
    );

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

    property_row(ui, "Netlist order", &format!("{} of {total}", order + 1));
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        if ui
            .add_enabled(order > 0, egui::Button::new("Move earlier"))
            .clicked()
        {
            edit.record_undo(&before);
            document.pins.swap(order, order - 1);
            edit.session.undo_recorded = false;
            changed = true;
        }
        if ui
            .add_enabled(order + 1 < total, egui::Button::new("Move later"))
            .clicked()
        {
            edit.record_undo(&before);
            document.pins.swap(order, order + 1);
            edit.session.undo_recorded = false;
            changed = true;
        }
    });

    let active_name = document.pins[order].name.clone();
    let declared = ports
        .iter()
        .any(|port| port.name.eq_ignore_ascii_case(&active_name));
    property_row_status(
        ui,
        "Declared by interface",
        if declared { "yes" } else { "orphaned" },
        if declared {
            Tokens::get(ui.ctx()).color.ok
        } else {
            Tokens::get(ui.ctx()).color.err
        },
        if declared {
            StatusMark::Success
        } else {
            StatusMark::Warning
        },
    );
    let placed = document.pins[order].position.is_some();
    let on_grid = document.pins[order].terminal_on_grid();
    property_row_status(
        ui,
        "Terminal",
        if !placed {
            "unplaced"
        } else if on_grid {
            "on grid"
        } else {
            "off grid"
        },
        if placed && on_grid {
            Tokens::get(ui.ctx()).color.ok
        } else {
            Tokens::get(ui.ctx()).color.err
        },
        if placed && on_grid {
            StatusMark::Success
        } else {
            StatusMark::Warning
        },
    );
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
    ui.add_space(4.0);
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
        empty_selection_section(ui);
        return false;
    };
    let before = document.clone();
    section_header(ui, "Selected shape", Some(shape_kind(&shape)));

    let mut changed = false;
    let mut edited = shape;
    match &mut edited {
        SymbolShape::Polyline { points, closed } => {
            property_row(ui, "Type", if *closed { "closed" } else { "open" });
            property_row(ui, "Points", &points.len().to_string());
            if let Some(first) = points.first_mut() {
                let mut anchor = *first;
                let moved = coordinate_row(ui, edit, &before, "Start X", &mut anchor.x)
                    | coordinate_row(ui, edit, &before, "Start Y", &mut anchor.y);
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
            changed |= coordinate_row(ui, edit, &before, "Quarter turns", rotation_quarters);
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
    muted_inspector_copy(
        ui,
        "Body geometry is cosmetic; it never changes the port contract.",
    );
    changed
}

fn visibility_row(ui: &mut Ui, label: &str, shown: &mut bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let before = *shown;
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.add_sized(
            [112.0, t.metrics.ctl_h],
            egui::Label::new(
                egui::RichText::new(label)
                    .size(tokens::FS_0)
                    .color(t.color.text_dim),
            ),
        );
        ui.checkbox(shown, if *shown { "Shown" } else { "Hidden" });
    });
    *shown != before
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
        empty_selection_section(ui);
        return false;
    };
    let before = document.clone();
    let mut changed = false;
    section_header(ui, "Selected attribute", Some(kind.key()));

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

    let mut shown = metadata.attributes[index].shown;
    if visibility_row(ui, "Visibility", &mut shown) {
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
    muted_inspector_copy(
        ui,
        "Attributes are rendered on every placed instance when visibility is enabled.",
    );
    changed
}

fn empty_selection_section(ui: &mut Ui) {
    section_header(ui, "Selection", Some("none"));
    muted_inspector_copy(
        ui,
        "Select a pin or body shape on the canvas, or in the Symbol editor panel, to edit it here.",
    );
}

fn multi_selection_section(ui: &mut Ui, pins: usize, shapes: usize, attributes: usize) {
    section_header(
        ui,
        "Selection",
        Some(&(pins + shapes + attributes).to_string()),
    );
    property_row(ui, "Pins", &pins.to_string());
    property_row(ui, "Body shapes", &shapes.to_string());
    property_row(ui, "Attributes", &attributes.to_string());
    muted_inspector_copy(
        ui,
        "Reduce the selection to one object to edit its geometry. Canvas transforms apply to the whole selection.",
    );
}

fn definition_section(ui: &mut Ui, edit: &mut InspectorEdit, metadata: &SymbolEditorMetadata) {
    section_header(ui, "Symbol definition", Some("CDF"));
    property_row(ui, "Published revision", &metadata.revision.to_string());
    property_row(
        ui,
        "Revision note",
        if metadata.revision_note.is_empty() {
            "not published"
        } else {
            &metadata.revision_note
        },
    );
    if labeled_icon_button(
        ui,
        WorkbenchIcon::Sliders,
        "Open CDF / form designer",
        false,
        ui.available_width(),
    )
    .clicked()
    {
        edit.open_parameter_form = true;
    }
}

// =============================================================================
// Pin contract
// =============================================================================

/// The ordered comparison the symbol is answerable to: for each declared
/// port, the pin this symbol places for it.
fn contract_section(
    ui: &mut Ui,
    edit: &mut InspectorEdit,
    document: &SymbolDocument,
    ports: &[PortSpec],
) {
    let summary = document.pin_summary(ports);
    section_header(
        ui,
        "Pin contract · netlist order",
        Some(match summary {
            PinSummary::Match => "matches",
            PinSummary::NoSchematic => "no interface",
            _ => "mismatch",
        }),
    );
    if ports.is_empty() {
        muted_inspector_copy(
            ui,
            "No schematic in this project declares an interface for this cell, so there is no contract to satisfy.",
        );
        return;
    }

    let t = Tokens::get(ui.ctx());
    let mut select: Option<String> = None;
    for (index, port) in ports.iter().enumerate() {
        let pin = document
            .pins
            .iter()
            .find(|pin| pin.name.eq_ignore_ascii_case(&port.name));
        let placed = pin.is_some_and(|pin| pin.position.is_some());
        let meta = match pin {
            Some(pin) => match pin.position {
                Some(point) => format!("{}, {}", point.x, point.y),
                None => "unplaced".to_owned(),
            },
            None => "no pin".to_owned(),
        };
        let label = format!("{}. {}", index + 1, port.name);
        let row = TreeRow::new(&label)
            .mono()
            .indent(1)
            .meta(&meta)
            .chip_dot(if placed { t.color.ok } else { t.color.err })
            .selected(edit.selection.pins.contains(&port.name))
            .show(ui);
        if row.response.clicked() && pin.is_some() {
            select = Some(port.name.clone());
        }
    }
    if let Some(name) = select {
        edit.select_pin(name);
    }

    // Pins the symbol places that no port declares are the other half of a
    // mismatch, and are invisible in a port-ordered list.
    let orphaned: Vec<&SymbolPin> = document
        .pins
        .iter()
        .filter(|pin| {
            !ports
                .iter()
                .any(|port| port.name.eq_ignore_ascii_case(&pin.name))
        })
        .collect();
    if !orphaned.is_empty() {
        section_header(ui, "Undeclared pins", Some(&orphaned.len().to_string()));
        for pin in orphaned {
            property_row_status(
                ui,
                &pin.name,
                "not declared by the interface",
                t.color.err,
                StatusMark::Warning,
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rspice_design::symbol::SymbolPin;
    use rspice_design_model::port::PortDirection;

    /// The row is `declared_width` in words: the name decides the width, and
    /// the form has no second answer. A pin that declares no range shows no
    /// row rather than one stating the obvious.
    #[test]
    fn the_pin_width_row_states_what_the_name_declares() {
        for (name, expected) in [
            ("DATA[3:0]", "4 conductors \u{2014} DATA[3:0]"),
            ("ADDR<0:7>", "8 conductors \u{2014} ADDR<0:7>"),
        ] {
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
}

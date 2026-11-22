//! The editable rows of a documentation shape, a note and a net label.
//!
//! A shape's stroke, dash and fill, and a note's size and alignment, take
//! the symbol editor's words, so what is drawn on a sheet and what is drawn
//! in a symbol are styled alike. Each change is one undo step.

use super::*;
use rspice_design::schematic::design_note::{
    DESIGN_NOTE_SIZE_CHOICES, DesignNote, SymbolTextAlign, design_note_size_label,
};
use rspice_design::schematic::documentation_shape::{
    DocumentationShape, SymbolDash, SymbolFill, SymbolStroke,
};
use rspice_design::schematic::net_label::{NetLabel, NetLabelKind, NetLabelSide, SupplyGlyph};
use rspice_design_model::design_management::CrossSheetPortDirection;

/// One choice row: `all` named by `name`, with `current` chosen. Returns the
/// choice picked this frame when it differs.
fn choice_row<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    id: (&str, u64),
    all: &[T],
    name: fn(T) -> &'static str,
    current: T,
    enabled: bool,
) -> Option<T> {
    let options = all
        .iter()
        .map(|item| (name(*item).to_owned(), name(*item).to_owned()))
        .collect::<Vec<_>>();
    let mut selected = name(current).to_owned();
    if !property_row_combo(ui, label, id, &mut selected, &options, enabled) {
        return None;
    }
    all.iter()
        .copied()
        .find(|item| name(*item) == selected)
        .filter(|item| *item != current)
}

/// Stroke, Dash and, on a closed shape, Fill.
pub(super) fn shape_style_rows(
    ui: &mut Ui,
    cx: &mut Cx<'_, DesignFeature>,
    shape: &DocumentationShape,
) {
    let writable = !cx.model.schematic_edit_read_only();
    let mut style = shape.style;
    if let Some(stroke) = choice_row(
        ui,
        "Stroke",
        ("shape-stroke", shape.id),
        &SymbolStroke::ALL,
        SymbolStroke::label,
        style.stroke,
        writable,
    ) {
        style.stroke = stroke;
    }
    if let Some(dash) = choice_row(
        ui,
        "Dash",
        ("shape-dash", shape.id),
        &SymbolDash::ALL,
        SymbolDash::label,
        style.dash,
        writable,
    ) {
        style.dash = dash;
    }
    if shape.kind().is_closed()
        && let Some(fill) = choice_row(
            ui,
            "Fill",
            ("shape-fill", shape.id),
            &SymbolFill::ALL,
            SymbolFill::label,
            style.fill,
            writable,
        )
    {
        style.fill = fill;
    }
    if style == shape.style {
        return;
    }
    match cx.model.schematic.edit_documentation_shape_properties(
        shape.clone(),
        shape.geometry.clone(),
        style,
    ) {
        Ok(true) => sync_active_schematic(cx.model, cx.wb),
        Ok(false) => {}
        Err(error) => cx.wb.prompt(format!("Shape not changed: {error}")),
    }
}

/// Size and Align.
pub(super) fn note_lettering_rows(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>, note: &DesignNote) {
    let writable = !cx.model.schematic_edit_read_only();
    let current = note.text_size_percent;
    let mut sizes = DESIGN_NOTE_SIZE_CHOICES
        .iter()
        .map(|percent| (percent.to_string(), design_note_size_label(*percent)))
        .collect::<Vec<_>>();
    if !DESIGN_NOTE_SIZE_CHOICES.contains(&current) {
        // A size a drawing brought in keeps its place in the list.
        sizes.push((current.to_string(), design_note_size_label(current)));
    }
    let mut selected = current.to_string();
    let mut size_percent = current;
    if property_row_combo(
        ui,
        "Size",
        ("note-size", note.id),
        &mut selected,
        &sizes,
        writable,
    ) && let Ok(picked) = selected.parse::<u16>()
    {
        size_percent = picked;
    }
    let align = choice_row(
        ui,
        "Align",
        ("note-align", note.id),
        &SymbolTextAlign::ALL,
        SymbolTextAlign::label,
        note.alignment(),
        writable,
    )
    .unwrap_or(note.alignment());
    if size_percent == current && align == note.alignment() {
        return;
    }
    match cx.model.schematic.edit_design_note_properties(
        note.clone(),
        note.kind,
        note.text.clone(),
        note.review.as_ref().map(|review| review.state),
        (size_percent, align),
    ) {
        Ok(true) => sync_active_schematic(cx.model, cx.wb),
        Ok(false) => {}
        Err(error) => cx.wb.prompt(format!("Note not changed: {error}")),
    }
}

/// What a label is, as the Label row names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LabelRole {
    Net,
    OffSheet,
    Supply,
}

impl LabelRole {
    const ALL: [Self; 3] = [Self::Net, Self::OffSheet, Self::Supply];

    const fn of(kind: NetLabelKind) -> Self {
        match kind {
            NetLabelKind::Local => Self::Net,
            NetLabelKind::OffSheet { .. } => Self::OffSheet,
            NetLabelKind::Supply { .. } => Self::Supply,
        }
    }

    const fn label(self) -> &'static str {
        match self {
            Self::Net => "Net label",
            Self::OffSheet => "Off-sheet connector",
            Self::Supply => "Supply",
        }
    }
}

/// The sides a placed label's name may stand on, as the Side row names
/// them. The upward sides are a drawing's flags on vertical wires.
fn side_label(side: NetLabelSide) -> &'static str {
    match side {
        NetLabelSide::Right => "right",
        NetLabelSide::Left => "left",
        NetLabelSide::Above => "above",
        NetLabelSide::Below => "below",
        NetLabelSide::RisingAbove => "upward, above",
        NetLabelSide::RisingBelow => "upward, below",
        NetLabelSide::RisingBeside => "upward, beside",
    }
}

const SIDES: [NetLabelSide; 7] = [
    NetLabelSide::Right,
    NetLabelSide::Left,
    NetLabelSide::Above,
    NetLabelSide::Below,
    NetLabelSide::RisingAbove,
    NetLabelSide::RisingBelow,
    NetLabelSide::RisingBeside,
];

/// A selected net label's kind and the side its name stands on: a local
/// label can become an off-sheet connector or a supply and back, as KiCad's
/// "Change to" turns a label into another kind, keeping its name and place.
pub(super) fn net_label_rows(ui: &mut Ui, cx: &mut Cx<'_, DesignFeature>) {
    let Some(label) = cx
        .model
        .schematic
        .session
        .editor
        .selection
        .single_net_label()
        .and_then(|id| {
            cx.model
                .schematic
                .document()
                .net_labels
                .iter()
                .find(|label| label.id == id)
        })
        .cloned()
    else {
        return;
    };
    let writable = !cx.model.schematic_edit_read_only();
    section_header(ui, "Label", None);
    let (mut kind, mut side) = (label.kind, label.side);
    if let Some(role) = choice_row(
        ui,
        "Kind",
        ("label-kind", label.id),
        &LabelRole::ALL,
        LabelRole::label,
        LabelRole::of(kind),
        writable,
    ) {
        kind = match role {
            LabelRole::Net => NetLabelKind::Local,
            LabelRole::OffSheet => NetLabelKind::OffSheet {
                direction: CrossSheetPortDirection::InOut,
            },
            LabelRole::Supply => NetLabelKind::Supply {
                glyph: SupplyGlyph::for_name(&label.name),
            },
        };
    }
    match kind {
        NetLabelKind::OffSheet { direction } => {
            if let Some(direction) = choice_row(
                ui,
                "Direction",
                ("label-direction", label.id),
                &NetLabelKind::DIRECTIONS,
                NetLabelKind::direction_label,
                direction,
                writable,
            ) {
                kind = NetLabelKind::OffSheet { direction };
            }
        }
        NetLabelKind::Supply { glyph } => {
            if let Some(glyph) = choice_row(
                ui,
                "Symbol",
                ("label-glyph", label.id),
                &SupplyGlyph::ALL,
                SupplyGlyph::label,
                glyph,
                writable,
            ) {
                kind = NetLabelKind::Supply { glyph };
            }
        }
        NetLabelKind::Local => {}
    }
    // A supply's name follows its symbol, so it has no side to choose.
    if !matches!(kind, NetLabelKind::Supply { .. })
        && let Some(next) = choice_row(
            ui,
            "Side",
            ("label-side", label.id),
            &SIDES,
            side_label,
            side,
            writable,
        )
    {
        side = next;
    }
    if kind == label.kind && side == label.side {
        return;
    }
    change_label(cx, label, kind, side);
}

fn change_label(
    cx: &mut Cx<'_, DesignFeature>,
    label: NetLabel,
    kind: NetLabelKind,
    side: NetLabelSide,
) {
    match cx.model.schematic.change_net_label_kind(label, kind, side) {
        Ok(true) => sync_active_schematic(cx.model, cx.wb),
        Ok(false) => {}
        Err(error) => cx.wb.prompt(format!("Label not changed: {error}")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::Point;
    use crate::workbench::design::test_host::DesignTestHost;

    /// A placed net label becomes a supply, whose name is global, and back;
    /// its name moves to the left of its point. Each change is one undo step
    /// and keeps the label's name and place.
    #[test]
    fn a_placed_label_changes_kind_and_side_in_one_step_each() {
        let mut state = DesignTestHost::default();
        let label = NetLabel::new(5, Point::new(40, 20), "VDD");
        state
            .model
            .schematic
            .document_mut_for_test()
            .net_labels
            .push(label.clone());
        state.model.schematic.recalculate_runtime_state();
        state.model.schematic.init_undo_history();
        let topology = state.model.schematic.topology_version();

        let supply = NetLabelKind::Supply {
            glyph: SupplyGlyph::Bar,
        };
        assert_eq!(
            state
                .model
                .schematic
                .change_net_label_kind(label.clone(), supply, label.side),
            Ok(true)
        );
        let changed = state.model.schematic.document().net_labels[0].clone();
        assert_eq!((changed.kind, changed.pos), (supply, label.pos));
        assert_eq!(changed.global_declaration().as_deref(), Some("VDD!"));
        assert_ne!(state.model.schematic.topology_version(), topology);
        assert_eq!(
            state.model.schematic.undo_description(),
            Some("change net label kind")
        );
        assert!(state.model.schematic.undo());
        assert_eq!(state.model.schematic.document().net_labels[0], label);

        assert_eq!(
            state.model.schematic.change_net_label_kind(
                label.clone(),
                NetLabelKind::Local,
                NetLabelSide::Left
            ),
            Ok(true)
        );
        assert_eq!(
            state.model.schematic.document().net_labels[0].side,
            NetLabelSide::Left
        );
        assert_eq!(
            state.model.schematic.change_net_label_kind(
                label,
                NetLabelKind::Local,
                NetLabelSide::Right
            ),
            Err("The selected net label changed before commit.".to_owned())
        );
    }
}

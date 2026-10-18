//! The Sheet section: the paper, where the sheet's format comes from, the
//! frame its standard draws, and the unit its sizes are stated in, with what
//! does not fit on the sheet they make said under the size.

use egui::Ui;

use super::super::action_stack;
use super::{PanelEdit, SheetIdentity, SheetText, choice_row, text_row};
use crate::schematic::view::drawing_sheet::{
    ActiveDrawingSheet, DrawingSheetOverflowSummary, drawing_sheet_overflow_summary,
    show_first_drawing_sheet_overflow,
};
use crate::schematic::view::frame_symbol_context;
use crate::state::{
    AuthoredDrawingSheetSize, DrawingSheetBorderTemplate, DrawingSheetDisplayUnit,
    DrawingSheetFrameConvention, DrawingSheetInheritance, DrawingSheetPreset,
    DrawingSheetPresetScope, DrawingSheetStandard, SchematicPageOrientation, SchematicSheetFormat,
    drawing_sheet_automatic_zone_counts,
};
use crate::ui::icons::Icon;
use crate::ui::widgets::Button;
use crate::workbench::AppState;
use crate::workbench::app::drawing_sheet_edit::{
    DrawingSheetEdit, current_size_choice_label, mixed_size_advisory,
    standard_orientation_advisory, standard_size_choice_label,
};
use crate::workbench::app::open_drawing_sheet_overflow_review;
use crate::workbench::design_system::{Problem, problem_line};
use rspice_ui_kit::panels::section_header;

/// What the Size menu offers: a standard size, a saved custom size, or a
/// one-off size the sheet states its own edges for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SizeChoice {
    Standard(DrawingSheetStandard),
    /// The saved custom size at this index of the menu's list.
    Saved(usize),
    OneOff,
}

/// Paper size and orientation, where the format comes from, the frame and
/// the unit, and what does not fit on the sheet they make.
pub(super) fn sheet_section(
    ui: &mut Ui,
    state: &mut AppState,
    sheet: &ActiveDrawingSheet,
    identity: &SheetIdentity,
    editable: bool,
    edit: &mut Option<PanelEdit>,
) {
    section_header(ui, "Sheet", None);
    let format = &sheet.format;
    size_rows(ui, state, format, identity, editable, edit);
    overflow_rows(ui, state, sheet);
    if let Some(advisory) = mixed_size_advisory(state, format) {
        problem_line(ui, Problem::Warning, advisory);
    }

    let orientations = [
        (SchematicPageOrientation::Landscape, "Landscape"),
        (SchematicPageOrientation::Portrait, "Portrait"),
    ]
    .map(|(choice, name)| (choice, name.to_owned()));
    if let Some(orientation) = choice_row(
        ui,
        editable,
        "Orientation",
        "sheet-inspector-orientation",
        format.orientation,
        &orientations,
    ) {
        *edit = Some(Ok(DrawingSheetEdit::Orientation(orientation)));
    }
    if let Some(advisory) = standard_orientation_advisory(&format.authored_size, format.orientation)
    {
        problem_line(ui, Problem::Warning, advisory);
    }

    let mut sources = vec![
        (
            DrawingSheetInheritance::ProjectDefault,
            "Project default".to_owned(),
        ),
        (DrawingSheetInheritance::Explicit, "This sheet".to_owned()),
    ];
    if format.inheritance == DrawingSheetInheritance::UserDefault {
        sources.push((
            DrawingSheetInheritance::UserDefault,
            "Personal default".to_owned(),
        ));
    }
    if let Some(source) = choice_row(
        ui,
        editable,
        "Format from",
        "sheet-inspector-format-source",
        format.inheritance,
        &sources,
    ) {
        *edit = Some(Ok(DrawingSheetEdit::FollowProjectDefault(
            source == DrawingSheetInheritance::ProjectDefault,
        )));
    }

    frame_rows(ui, format, editable, edit);

    let units = [
        (DrawingSheetDisplayUnit::Millimetres, "Millimetres"),
        (DrawingSheetDisplayUnit::Centimetres, "Centimetres"),
        (DrawingSheetDisplayUnit::Inches, "Inches"),
    ]
    .map(|(unit, name)| (unit, name.to_owned()));
    if let Some(unit) = choice_row(
        ui,
        editable,
        "Units",
        "sheet-inspector-units",
        format.display_unit,
        &units,
    ) {
        *edit = Some(Ok(DrawingSheetEdit::Units(unit)));
    }
}

/// The Size menu, and for a one-off size its edges and name and the two
/// places it can be saved to.
fn size_rows(
    ui: &mut Ui,
    state: &AppState,
    format: &SchematicSheetFormat,
    identity: &SheetIdentity,
    editable: bool,
    edit: &mut Option<PanelEdit>,
) {
    let saved = saved_sizes(ui, state);
    let one_off = match &format.authored_size {
        AuthoredDrawingSheetSize::Standard { .. } => false,
        AuthoredDrawingSheetSize::Custom { snapshot } => !saved.iter().any(|preset| {
            snapshot
                .preset_id
                .as_deref()
                .is_some_and(|id| preset.id.eq_ignore_ascii_case(id))
        }),
    };
    let current = match &format.authored_size {
        AuthoredDrawingSheetSize::Standard { standard } => SizeChoice::Standard(*standard),
        AuthoredDrawingSheetSize::Custom { .. } if one_off => SizeChoice::OneOff,
        AuthoredDrawingSheetSize::Custom { snapshot } => saved
            .iter()
            .position(|preset| {
                snapshot
                    .preset_id
                    .as_deref()
                    .is_some_and(|id| preset.id.eq_ignore_ascii_case(id))
            })
            .map_or(SizeChoice::OneOff, SizeChoice::Saved),
    };
    let mut choices = DrawingSheetStandard::ALL
        .into_iter()
        .map(|standard| {
            (
                SizeChoice::Standard(standard),
                standard_size_choice_label(standard, format),
            )
        })
        .collect::<Vec<_>>();
    choices.extend(saved.iter().enumerate().map(|(index, preset)| {
        (
            SizeChoice::Saved(index),
            if current == SizeChoice::Saved(index) {
                current_size_choice_label(format)
            } else {
                format!(
                    "{} \u{00b7} {}",
                    preset.name,
                    match preset.scope {
                        DrawingSheetPresetScope::User => "personal",
                        _ => "project",
                    }
                )
            },
        )
    }));
    choices.push((
        SizeChoice::OneOff,
        if one_off {
            current_size_choice_label(format)
        } else {
            "Custom size\u{2026}".to_owned()
        },
    ));
    match choice_row(
        ui,
        editable,
        "Size",
        "sheet-inspector-size",
        current,
        &choices,
    ) {
        Some(SizeChoice::Standard(standard)) => {
            *edit = Some(Ok(DrawingSheetEdit::Size(standard)));
        }
        Some(SizeChoice::Saved(index)) => {
            *edit = Some(Ok(DrawingSheetEdit::Preset(saved[index].clone())));
        }
        Some(SizeChoice::OneOff) => {
            let (width_um, height_um) = format.oriented_dimensions_um();
            *edit = Some(Ok(DrawingSheetEdit::CustomSize {
                width_um,
                height_um,
            }));
        }
        None => {}
    }
    if !(one_off && editable) {
        return;
    }
    let unit = match format.display_unit {
        DrawingSheetDisplayUnit::Millimetres => "mm",
        DrawingSheetDisplayUnit::Centimetres => "cm",
        DrawingSheetDisplayUnit::Inches => "in",
    };
    for text in [SheetText::Width, SheetText::Height, SheetText::SizeName] {
        let label = match text {
            SheetText::SizeName => text.label().to_owned(),
            _ => format!("{} ({unit})", text.label()),
        };
        if let Some(typed) = text_row(ui, identity, text, &label, &text.stored(state, format), "") {
            *edit = Some(text.edit(format, typed));
        }
    }
    action_stack(ui, |ui| {
        if Button::new("Save as project size").show(ui).clicked() {
            *edit = Some(Ok(DrawingSheetEdit::SaveCustomSize(
                DrawingSheetPresetScope::Project,
            )));
        }
        if Button::new("Save as personal size").show(ui).clicked() {
            *edit = Some(Ok(DrawingSheetEdit::SaveCustomSize(
                DrawingSheetPresetScope::User,
            )));
        }
    });
}

/// The saved custom sizes the Size menu offers: the project's, and the
/// person's own while a menu is open to show them. Reading the personal ones
/// parses preferences, which a closed menu has no use for every frame.
fn saved_sizes(ui: &Ui, state: &AppState) -> Vec<DrawingSheetPreset> {
    let mut saved = state
        .workspace
        .content
        .design_management
        .drawing_sheet_settings()
        .presets
        .clone();
    if egui::Popup::is_any_open(ui.ctx()) {
        saved.extend(
            state
                .ui
                .preferences
                .drawing_sheet_personal_preferences()
                .presets,
        );
    }
    saved.retain(|preset| {
        !matches!(
            &preset.format.authored_size,
            AuthoredDrawingSheetSize::Custom { snapshot } if snapshot.source_preset_unavailable
        )
    });
    saved
}

/// How much stands outside the drawing area of the sheet as it is, said
/// under the size that decides it, with the way to the first object and to
/// the review of them all.
fn overflow_rows(ui: &mut Ui, state: &mut AppState, sheet: &ActiveDrawingSheet) {
    // The canvas's own cached context: the inspector resolves nothing.
    let symbol_context = frame_symbol_context(state);
    let overflow = drawing_sheet_overflow_summary(state, &symbol_context, sheet);
    if overflow.is_clear() {
        return;
    }
    problem_line(
        ui,
        Problem::Warning,
        &drawing_sheet_overflow_text(&overflow),
    );
    action_stack(ui, |ui| {
        if Button::new("Show first")
            .icon(Icon::ZoomFit)
            .show(ui)
            .clicked()
        {
            show_first_drawing_sheet_overflow(state);
        }
        if Button::new("Review all\u{2026}").show(ui).clicked() {
            open_drawing_sheet_overflow_review(state);
        }
    });
}

/// The frame of the sheet's standard, and on it the standard's zones, its
/// centring marks or microfilm arrows, and ASME Y14.1's binding margin.
fn frame_rows(
    ui: &mut Ui,
    format: &SchematicSheetFormat,
    editable: bool,
    edit: &mut Option<PanelEdit>,
) {
    let convention = format.frame_convention();
    let frames = [
        (
            DrawingSheetBorderTemplate::Standard,
            match convention {
                DrawingSheetFrameConvention::Iso5457 => "ISO 5457 frame",
                DrawingSheetFrameConvention::AsmeY141 => "ASME Y14.1 border",
            },
        ),
        (DrawingSheetBorderTemplate::None, "None"),
    ]
    .map(|(border, name)| (border, name.to_owned()));
    if let Some(border) = choice_row(
        ui,
        editable,
        "Frame",
        "sheet-inspector-frame",
        format.border,
        &frames,
    ) {
        *edit = Some(Ok(DrawingSheetEdit::Frame(border)));
    }
    if format.border == DrawingSheetBorderTemplate::None {
        return;
    }

    let (columns, rows) =
        drawing_sheet_automatic_zone_counts(&format.authored_size, format.orientation);
    let zones = [
        (true, format!("{columns} \u{00d7} {rows}")),
        (false, "None".to_owned()),
    ];
    let zoned = format.zones.mode != crate::state::DrawingSheetZoneMode::None;
    if let Some(on) = choice_row(
        ui,
        editable,
        "Zones",
        "sheet-inspector-zones",
        zoned,
        &zones,
    ) {
        *edit = Some(Ok(DrawingSheetEdit::Zones(on)));
    }

    let marks = [(true, "Drawn"), (false, "None")].map(|(on, name)| (on, name.to_owned()));
    let marks_label = match convention {
        DrawingSheetFrameConvention::Iso5457 => "Centring marks",
        DrawingSheetFrameConvention::AsmeY141 => "Microfilm arrows",
    };
    if let Some(on) = choice_row(
        ui,
        editable,
        marks_label,
        "sheet-inspector-marks",
        format.marks.centring,
        &marks,
    ) {
        *edit = Some(Ok(DrawingSheetEdit::CentringMarks(on)));
    }

    if format.can_bind() {
        let margins =
            [(false, "None"), (true, "0.75 in left")].map(|(on, name)| (on, name.to_owned()));
        if let Some(on) = choice_row(
            ui,
            editable,
            "Binding margin",
            "sheet-inspector-binding",
            format.binding_margin,
            &margins,
        ) {
            *edit = Some(Ok(DrawingSheetEdit::BindingMargin(on)));
        }
    }
}

/// How many objects stand outside the drawing area, and where.
fn drawing_sheet_overflow_text(summary: &DrawingSheetOverflowSummary) -> String {
    let mut details = Vec::with_capacity(2);
    if summary.outside_border > 0 {
        let mut outside = format!("{} outside the border", summary.outside_border);
        if summary.off_paper > 0 {
            outside.push_str(&format!(" · {} beyond the paper edge", summary.off_paper));
        }
        details.push(outside);
    }
    if summary.title_block_collisions > 0 {
        details.push(format!(
            "{} overlapping the title block",
            summary.title_block_collisions
        ));
    }
    let headline = format!(
        "{} outside the drawing area",
        super::super::count_label(summary.finding_count(), "object")
    );
    if details.is_empty() {
        headline
    } else {
        format!("{headline}: {}", details.join(" · "))
    }
}

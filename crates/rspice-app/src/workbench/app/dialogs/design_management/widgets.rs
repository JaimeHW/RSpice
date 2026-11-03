//! Widgets shared by the design management surfaces.

use super::*;

pub(super) fn split_surface(
    ui: &mut Ui,
    left_fraction: f32,
    content: impl FnOnce(&mut Ui, &mut Ui),
) {
    let t = Tokens::get(ui.ctx());
    let available = ui.available_width().max(1.0);
    let stacked = available < SPLIT_BREAKPOINT;
    let origin = ui.next_widget_position();
    // A pane's rows stand [`PANE_INSET`] in from its edge, so a pane stands
    // the rest of the dialog's inset in from the body's: every row then
    // starts where the title and the footer do. The panes were bordered
    // boxes a further 12 points in, and their rows started 10 right of the
    // title.
    let padding = vec2(
        crate::ui::widgets::DIALOG_BODY_INSET - PANE_INSET,
        tokens::SP_5,
    );
    let extent = 10_000.0;

    if stacked {
        ui.columns(2, |columns| {
            let (left, right) = columns.split_at_mut(1);
            content(&mut left[0], &mut right[0]);
        });
        return;
    }

    let (left_width, right_width) =
        if (left_fraction - MAIN_SPLIT_LEFT_FRACTION).abs() < f32::EPSILON {
            main_split_widths(available)
        } else {
            subflow_split_widths(available)
        };
    let divider_x = origin.x + left_width;
    let left_rect = egui::Rect::from_min_size(
        origin + padding,
        vec2((left_width - padding.x * 2.0).max(1.0), extent),
    );
    let right_rect = egui::Rect::from_min_size(
        egui::pos2(divider_x + padding.x, origin.y + padding.y),
        vec2((right_width - padding.x * 2.0).max(1.0), extent),
    );
    let mut left_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(left_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    let mut right_ui = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(right_rect)
            .layout(Layout::top_down(Align::Min)),
    );
    content(&mut left_ui, &mut right_ui);
    let height = left_ui
        .min_rect()
        .height()
        .max(right_ui.min_rect().height())
        + padding.y * 2.0;
    let surface = egui::Rect::from_min_size(origin, vec2(available, height.max(80.0)));
    ui.painter().vline(
        divider_x,
        surface.y_range(),
        Stroke::new(1.0, t.color.border),
    );
    ui.advance_cursor_after_rect(surface);
}

/// A pane's actions: one row under what they act on, on the pane's inset.
pub(super) fn toolbar(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    Frame::NONE
        .inner_margin(Margin {
            left: PANE_INSET as i8,
            right: PANE_INSET as i8,
            top: 8,
            bottom: 0,
        })
        .show(ui, |ui| {
            ui.horizontal_wrapped(content);
        });
}

/// Where a pane's rows start: the inset its section titles and property rows
/// already keep, so labels, selects, notes, buttons and table text share it.
pub(super) const PANE_INSET: f32 = crate::workbench::design_system::PANEL_INSET;

/// `count` of `noun`, which takes an "s" for any count but one.
pub(super) fn counted(count: usize, noun: &str) -> String {
    format!("{count} {noun}{}", if count == 1 { "" } else { "s" })
}

pub(super) fn paint_table_header(ui: &mut Ui, weights: &[f32], labels: &[&str]) {
    let values = labels
        .iter()
        .map(|label| (*label).to_owned())
        .collect::<Vec<_>>();
    paint_table_cells(ui, weights, &values, true, None);
}

pub(super) fn paint_table_row(
    ui: &mut Ui,
    weights: &[f32],
    values: &[String],
    tone: Option<egui::Color32>,
) {
    paint_table_cells(ui, weights, values, false, tone);
}

pub(super) fn paint_table_cells(
    ui: &mut Ui,
    weights: &[f32],
    values: &[String],
    header: bool,
    tone: Option<egui::Color32>,
) {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { 29.0 };
    let (rect, response) =
        ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::hover());
    ui.painter().rect_filled(
        rect,
        0.0,
        if header {
            t.color.bg_panel_2
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        },
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let total = weights.iter().sum::<f32>().max(f32::EPSILON);
    let mut x = rect.left();
    for (index, value) in values.iter().enumerate() {
        let weight = weights.get(index).copied().unwrap_or(1.0);
        let width = if index + 1 == values.len() {
            rect.right() - x
        } else {
            rect.width() * weight / total
        };
        let cell = egui::Rect::from_min_max(
            egui::pos2(x, rect.top()),
            egui::pos2((x + width).min(rect.right()), rect.bottom()),
        );
        if index > 0 {
            ui.painter().vline(
                cell.left(),
                cell.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
        ui.painter()
            .with_clip_rect(cell.shrink2(vec2(PANE_INSET, 2.0)))
            .text(
                cell.left_center() + vec2(PANE_INSET, 0.0),
                egui::Align2::LEFT_CENTER,
                value,
                if header {
                    theme::sans(tokens::FS_0, FontWeight::SemiBold)
                } else {
                    theme::mono(tokens::FS_0, FontWeight::Regular)
                },
                tone.unwrap_or(if header {
                    t.color.text_dim
                } else {
                    t.color.text
                }),
            );
        x += width;
    }
}

/// A table's one row when it has none: a sentence, so the body face, at the
/// size a value reads. It was a cell in the rows' small mono.
pub(super) fn empty_table_row(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), 29.0), Sense::hover());
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), text));
    ui.painter().rect_filled(rect, 0.0, t.color.bg_inset);
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    ui.painter()
        .with_clip_rect(rect.shrink2(vec2(PANE_INSET, 2.0)))
        .text(
            rect.left_center() + vec2(PANE_INSET, 0.0),
            egui::Align2::LEFT_CENTER,
            text,
            theme::field_sans(),
            t.color.text_dim,
        );
}

/// A count or a state on a pane's label/value grid: words, so the body face.
pub(super) fn fact_row(ui: &mut Ui, label: &str, value: &str) {
    crate::workbench::design_system::property_row_wrapped(ui, label, value);
}

pub(super) fn muted_note(ui: &mut Ui, text: &str) {
    Frame::NONE
        .inner_margin(Margin::symmetric(PANE_INSET as i8, 6))
        .show(ui, |ui| {
            ui.label(
                egui::RichText::new(text)
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(Tokens::get(ui.ctx()).color.text_dim),
            );
        });
}

pub(super) fn combo_field<T: Copy + PartialEq>(
    ui: &mut Ui,
    id_namespace: &'static str,
    label: &str,
    options: &[(T, &str)],
    selected: &mut T,
    enabled: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    let selected_text = options
        .iter()
        .find(|(value, _)| value == selected)
        .map_or("Select", |(_, text)| *text);
    let labels = options
        .iter()
        .map(|(_, text)| (*text).to_owned())
        .collect::<Vec<_>>();
    ui.add_enabled_ui(enabled, |ui| {
        let width = ui.available_width();
        let output = crate::ui::widgets::select_with_response(
            ui,
            &format!("{id_namespace}.{label}"),
            label,
            selected_text,
            &labels,
            width,
        );
        if let Some(index) = output.picked {
            *selected = options[index].0;
        }
        output.response
    })
    .inner
}

/// One of a pane's settings: a select on the label/value grid its property
/// rows stand on, so a pane's facts and choices share one label column.
pub(super) fn setting_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, &str)],
    selected: &mut T,
    enabled: bool,
) -> bool {
    let shown = options
        .iter()
        .find(|(value, _)| value == selected)
        .map_or("", |(_, text)| *text);
    let labels = options
        .iter()
        .map(|(_, text)| (*text).to_owned())
        .collect::<Vec<_>>();
    match setting_select(
        ui,
        "design-management-setting",
        label,
        shown,
        &labels,
        enabled,
    ) {
        Some(index) if options[index].0 != *selected => {
            *selected = options[index].0;
            true
        }
        _ => false,
    }
}

/// A setting's row: its label in the property rows' label column and the
/// kit's select in their value column, so a pane's facts and choices share
/// one grid. The settings were the inspector's 22 point drop-downs with a
/// filled triangle, beside 28 point selects everywhere else.
fn setting_select(
    ui: &mut Ui,
    id_namespace: &str,
    label: &str,
    shown: &str,
    options: &[String],
    enabled: bool,
) -> Option<usize> {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let (label_x, value_x) =
        crate::workbench::design_system::property_row_control_columns(ui, width);
    let row = vec2(width, t.metrics.ctl_h + 4.0);
    ui.allocate_ui_with_layout(row, Layout::left_to_right(Align::Center), |ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.add_space(label_x);
        let (label_rect, label_response) =
            ui.allocate_exact_size(vec2(value_x - label_x, row.y), Sense::hover());
        label_response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label)
        });
        ui.painter().with_clip_rect(label_rect).text(
            label_rect.left_center(),
            egui::Align2::LEFT_CENTER,
            label,
            theme::sans(tokens::FS_0, FontWeight::Regular),
            t.color.text_dim,
        );
        let select_width = (width - value_x - label_x).max(1.0);
        ui.add_enabled_ui(enabled, |ui| {
            crate::ui::widgets::select(
                ui,
                &format!("{id_namespace}.{label}"),
                label,
                shown,
                options,
                select_width,
            )
        })
        .inner
    })
    .inner
}

pub(super) fn field_combo<T: Copy + PartialEq>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, &str)],
    selected: &mut T,
    enabled: bool,
) -> egui::Response {
    combo_field(
        ui,
        "design-management-field",
        label,
        options,
        selected,
        enabled,
    )
}

pub(super) fn id_combo<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    id_namespace: &'static str,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    base: Option<&str>,
    enabled: bool,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(label)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
    let selected_text = selected
        .and_then(|id| options.iter().find(|(candidate, _)| *candidate == id))
        .map_or_else(
            || base.unwrap_or("Select").to_owned(),
            |(_, name)| name.clone(),
        );
    // The base, when there is one, is the list's first choice: no id.
    let choices = base
        .map(|base| (None, base.to_owned()))
        .into_iter()
        .chain(options.iter().map(|(id, name)| (Some(*id), name.clone())))
        .collect::<Vec<_>>();
    let labels = choices
        .iter()
        .map(|(_, name)| name.clone())
        .collect::<Vec<_>>();
    ui.add_enabled_ui(enabled, |ui| {
        let width = ui.available_width();
        let output = crate::ui::widgets::select_with_response(
            ui,
            &format!("{id_namespace}.{label}"),
            label,
            &selected_text,
            &labels,
            width,
        );
        if let Some(index) = output.picked {
            *selected = choices[index].0;
        }
        output.response
    })
    .inner
}

/// [`setting_combo`] over identified choices, any of which may be unset.
pub(super) fn setting_combo_by_id<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    enabled: bool,
) -> bool {
    let shown = selected
        .and_then(|id| options.iter().find(|(candidate, _)| *candidate == id))
        .map_or("", |(_, name)| name.as_str());
    let labels = options
        .iter()
        .map(|(_, name)| name.clone())
        .collect::<Vec<_>>();
    match setting_select(
        ui,
        "design-management-setting-id",
        label,
        shown,
        &labels,
        enabled,
    ) {
        Some(index) if *selected != Some(options[index].0) => {
            *selected = Some(options[index].0);
            true
        }
        _ => false,
    }
}

pub(super) fn field_combo_by_id<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    enabled: bool,
) -> egui::Response {
    id_combo(
        ui,
        "design-management-field-id",
        label,
        options,
        selected,
        None,
        enabled,
    )
}

pub(super) fn field_combo_optional_base<T: Copy + PartialEq + std::fmt::Display>(
    ui: &mut Ui,
    label: &str,
    options: &[(T, String)],
    selected: &mut Option<T>,
    base: &str,
    enabled: bool,
) -> egui::Response {
    id_combo(
        ui,
        "design-management-field-base",
        label,
        options,
        selected,
        Some(base),
        enabled,
    )
}

pub(super) fn apply_domain_result<T, E: ToString>(
    error: &mut Option<String>,
    result: Result<T, E>,
    _receipt: &str,
) {
    *error = result.err().map(|error| error.to_string());
}

pub(super) fn sheet_choices(dialog: &DesignManagementDialogState) -> Vec<(SheetId, String)> {
    dialog
        .draft
        .as_ref()
        .and_then(|draft| draft.sheet_catalog(&dialog.owner_key))
        .map(|catalog| {
            catalog
                .sheets()
                .iter()
                .map(|sheet| (sheet.id(), sheet.name().to_owned()))
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn variant_choices(
    dialog: &DesignManagementDialogState,
) -> Vec<(AssemblyVariantId, String)> {
    dialog
        .draft
        .as_ref()
        .map(|draft| {
            draft
                .variants()
                .variants()
                .iter()
                .map(|variant| (variant.id(), variant.name().to_owned()))
                .collect()
        })
        .unwrap_or_default()
}

pub(super) fn parse_reserved_ranges(
    text: &str,
    dialog: &DesignManagementDialogState,
) -> Result<Vec<AnnotationReservedRange>, String> {
    if text.trim().eq_ignore_ascii_case("Project-owned ranges") {
        return Ok(dialog
            .draft
            .as_ref()
            .map(|draft| {
                draft
                    .annotation()
                    .policy()
                    .definition()
                    .reserved_ranges
                    .clone()
            })
            .unwrap_or_default());
    }
    let mut ranges = Vec::new();
    for segment in text
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let split = segment
            .char_indices()
            .find(|(_, character)| character.is_ascii_digit())
            .map(|(index, _)| index)
            .ok_or_else(|| format!("Range `{segment}` is missing a numeric interval."))?;
        let prefixes = segment[..split]
            .trim()
            .trim_end_matches(',')
            .split(',')
            .map(str::trim)
            .filter(|prefix| !prefix.is_empty())
            .map(str::to_owned)
            .collect::<Vec<_>>();
        if prefixes.is_empty() {
            return Err(format!("Range `{segment}` is missing a device prefix."));
        }
        let interval = segment[split..].trim();
        let bounds = interval
            .split(['…', '-', '–'])
            .map(str::trim)
            .collect::<Vec<_>>();
        if bounds.len() != 2 {
            return Err(format!(
                "Range `{segment}` must have a first and last number."
            ));
        }
        let first = bounds[0]
            .parse::<u32>()
            .map_err(|_| format!("Range `{segment}` has an invalid first number."))?;
        let last = bounds[1]
            .parse::<u32>()
            .map_err(|_| format!("Range `{segment}` has an invalid last number."))?;
        ranges.push(AnnotationReservedRange {
            scope: AnnotationRangeScope::Project,
            prefixes,
            first,
            last,
        });
    }
    if ranges.is_empty() {
        return Err("Enter at least one reserved range.".to_owned());
    }
    Ok(ranges)
}

pub(super) fn semantic_change_map(
    ui: &mut Ui,
    before: &str,
    subject: &str,
    after: &str,
    operation: &str,
) {
    let t = Tokens::get(ui.ctx());
    ui.horizontal(|ui| {
        for (index, (label, value, detail)) in [
            ("Current source", before, "unchanged until commit"),
            ("Proposed transaction", subject, operation),
            ("Result", after, "new stable revision"),
        ]
        .into_iter()
        .enumerate()
        {
            Frame::NONE
                .fill(t.color.bg_inset)
                .stroke(Stroke::new(1.0, t.color.border))
                .inner_margin(Margin::same(8))
                .show(ui, |ui| {
                    ui.set_min_width(116.0);
                    ui.label(
                        egui::RichText::new(label)
                            .size(tokens::FS_0)
                            .color(t.color.text_dim),
                    );
                    ui.label(
                        egui::RichText::new(value)
                            .size(tokens::FS_1)
                            .color(t.color.text),
                    );
                    ui.label(
                        egui::RichText::new(detail)
                            .size(tokens::FS_0)
                            .color(t.color.text_dim),
                    );
                });
            if index < 2 {
                ui.label(egui::RichText::new("→").color(t.color.accent));
            }
        }
    });
}

pub(super) fn schematic_preview(ui: &mut Ui, code: &str) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(ui.available_width(), 150.0), Sense::hover());
    ui.painter().rect_filled(rect, 0.0, t.color.canvas_bg);
    ui.painter().rect_stroke(
        rect,
        0.0,
        Stroke::new(1.0, t.color.border),
        egui::StrokeKind::Inside,
    );
    for x in (0..16).map(|index| rect.left() + 8.0 + index as f32 * 24.0) {
        for y in (0..7).map(|index| rect.top() + 8.0 + index as f32 * 24.0) {
            if rect.contains(egui::pos2(x, y)) {
                ui.painter()
                    .circle_filled(egui::pos2(x, y), 1.0, t.color.canvas_grid);
            }
        }
    }
    let center = rect.center();
    ui.painter().line_segment(
        [center - vec2(94.0, 0.0), center + vec2(94.0, 0.0)],
        Stroke::new(1.5, t.color.wire),
    );
    let opamp_center = center - vec2(42.0, 0.0);
    let opamp = [
        opamp_center + vec2(-24.0, -25.0),
        opamp_center + vec2(-24.0, 25.0),
        opamp_center + vec2(24.0, 0.0),
        opamp_center + vec2(-24.0, -25.0),
    ];
    ui.painter().add(egui::Shape::line(
        opamp.to_vec(),
        Stroke::new(1.5, t.color.symbol),
    ));
    ui.painter().text(
        opamp_center + vec2(-15.0, -10.0),
        egui::Align2::CENTER_CENTER,
        "−",
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.symbol,
    );
    ui.painter().text(
        opamp_center + vec2(-15.0, 10.0),
        egui::Align2::CENTER_CENTER,
        "+",
        theme::sans(tokens::FS_1, FontWeight::Regular),
        t.color.symbol,
    );
    let resistor_center = center + vec2(56.0, 0.0);
    let resistor = [
        resistor_center + vec2(-28.0, 0.0),
        resistor_center + vec2(-21.0, -9.0),
        resistor_center + vec2(-14.0, 9.0),
        resistor_center + vec2(-7.0, -9.0),
        resistor_center + vec2(0.0, 9.0),
        resistor_center + vec2(7.0, -9.0),
        resistor_center + vec2(14.0, 9.0),
        resistor_center + vec2(21.0, -9.0),
        resistor_center + vec2(28.0, 0.0),
    ];
    ui.painter().add(egui::Shape::line(
        resistor.to_vec(),
        Stroke::new(1.5, t.color.symbol),
    ));
    ui.painter().text(
        rect.center_bottom() - vec2(0.0, 14.0),
        egui::Align2::CENTER_BOTTOM,
        code,
        theme::mono(tokens::FS_1, FontWeight::SemiBold),
        t.color.accent,
    );
}

pub(super) fn subflow_before(
    dialog: &DesignManagementDialogState,
    page: DesignManagementPage,
) -> String {
    match page {
        DesignManagementPage::NewSheet | DesignManagementPage::MoveSelection => dialog
            .draft
            .as_ref()
            .and_then(|draft| draft.sheet_catalog(&dialog.owner_key))
            .map_or_else(
                || "no sheet catalog".to_owned(),
                |catalog| format!("sheet revision {}", catalog.revision()),
            ),
        DesignManagementPage::NewVariant
        | DesignManagementPage::CompareVariants
        | DesignManagementPage::VariantMatrix => format!(
            "{} governed variants",
            dialog
                .draft
                .as_ref()
                .map_or(0, |draft| draft.variants().variants().len())
        ),
        DesignManagementPage::RenumberPreview | DesignManagementPage::AnnotationPolicy => format!(
            "annotation policy {}",
            dialog
                .draft
                .as_ref()
                .map_or(0, |draft| draft.annotation().policy().revision())
        ),
        DesignManagementPage::HierarchyAudit => "active hierarchy resolution".to_owned(),
        DesignManagementPage::Manager => "working revision".to_owned(),
    }
}

pub(super) fn subflow_subject(
    dialog: &DesignManagementDialogState,
    page: DesignManagementPage,
) -> String {
    match page {
        DesignManagementPage::NewSheet => dialog.inputs.sheet_name.clone(),
        DesignManagementPage::MoveSelection => dialog.selection_summary.clone(),
        DesignManagementPage::NewVariant => dialog.inputs.variant_name.clone(),
        DesignManagementPage::CompareVariants => "differences between variants".to_owned(),
        DesignManagementPage::VariantMatrix => "governed override matrix".to_owned(),
        DesignManagementPage::RenumberPreview => "stable old-to-new mapping".to_owned(),
        DesignManagementPage::AnnotationPolicy => dialog.inputs.reserved_ranges.clone(),
        DesignManagementPage::HierarchyAudit => "configuration-bound audit".to_owned(),
        DesignManagementPage::Manager => "reviewed aggregate".to_owned(),
    }
}

pub(super) fn subflow_after(
    dialog: &DesignManagementDialogState,
    _page: DesignManagementPage,
) -> String {
    format!(
        "draft revision {}",
        dialog
            .draft
            .as_ref()
            .map_or(0, DesignManagementCatalog::revision)
    )
}

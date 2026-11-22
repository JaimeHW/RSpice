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
        // Too narrow for two panes, as on a phone: one pane at a time, the
        // list and its actions over what the tab sets, each the body's
        // width, and the reader scrolls from one to the other. The second
        // starts under the first's height as the last frame measured it; a
        // frame in which that height changed is laid out again before it is
        // shown.
        let width = (available - padding.x * 2.0).max(1.0);
        let memory = ui.id().with("stacked-first-pane");
        let first_height = ui.data(|data| data.get_temp::<f32>(memory)).unwrap_or(0.0);
        let divider_y = origin.y + padding.y * 2.0 + first_height;
        let mut left_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(egui::Rect::from_min_size(
                    origin + padding,
                    vec2(width, extent),
                ))
                .layout(Layout::top_down(Align::Min)),
        );
        let mut right_ui = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(egui::Rect::from_min_size(
                    egui::pos2(origin.x + padding.x, divider_y + padding.y),
                    vec2(width, extent),
                ))
                .layout(Layout::top_down(Align::Min)),
        );
        content(&mut left_ui, &mut right_ui);
        let measured = left_ui.min_rect().height();
        if (measured - first_height).abs() > 0.5 {
            ui.data_mut(|data| data.insert_temp(memory, measured));
            ui.ctx()
                .request_discard("a stacked design pane changed height");
        }
        ui.painter().hline(
            origin.x..=origin.x + available,
            divider_y,
            Stroke::new(1.0, t.color.border),
        );
        let bottom = right_ui.min_rect().bottom() + padding.y;
        let surface = egui::Rect::from_min_max(origin, egui::pos2(origin.x + available, bottom));
        ui.advance_cursor_after_rect(surface);
        return;
    }

    let (left_width, right_width) = split_widths(available, left_fraction);
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

/// One pane for a page that is fields alone, as wide as the manager's left
/// pane, or the dialog's width when it is narrow. Fields carry no inset of
/// their own, so the pane stands the dialog's whole inset in: its labels
/// start where the title and the footer do.
pub(super) fn single_pane(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let available = ui.available_width().max(1.0);
    let origin = ui.next_widget_position();
    let padding = vec2(crate::ui::widgets::DIALOG_BODY_INSET, tokens::SP_5);
    let width = if available < SPLIT_BREAKPOINT {
        available
    } else {
        split_widths(available, MAIN_SPLIT_LEFT_FRACTION).0
    };
    let mut pane = ui.new_child(
        egui::UiBuilder::new()
            .max_rect(egui::Rect::from_min_size(
                origin + padding,
                vec2((width - padding.x * 2.0).max(1.0), 10_000.0),
            ))
            .layout(Layout::top_down(Align::Min)),
    );
    content(&mut pane);
    let height = pane.min_rect().height() + padding.y * 2.0;
    ui.advance_cursor_after_rect(egui::Rect::from_min_size(
        origin,
        vec2(available, height.max(80.0)),
    ));
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

/// How the annotation policy can allocate prefixes, as the policy page
/// offers them and the Annotation tab states the one in force.
pub(super) const PREFIX_ALLOCATIONS: [(AnnotationPrefixAllocation, &str); 3] = [
    (
        AnnotationPrefixAllocation::ByDeviceFamily,
        "By device family",
    ),
    (AnnotationPrefixAllocation::BySheet, "By sheet"),
    (AnnotationPrefixAllocation::ByHierarchy, "By hierarchy"),
];

/// `count` of `noun`, which takes an "s" for any count but one.
pub fn counted(count: usize, noun: &str) -> String {
    format!("{count} {noun}{}", if count == 1 { "" } else { "s" })
}

pub(super) fn paint_table_header(ui: &mut Ui, weights: &[f32], labels: &[&str]) {
    let values = labels
        .iter()
        .map(|label| (*label).to_owned())
        .collect::<Vec<_>>();
    paint_table_cells(ui, weights, &values, true, None, Sense::hover());
}

pub(super) fn paint_table_row(
    ui: &mut Ui,
    weights: &[f32],
    values: &[String],
    tone: Option<egui::Color32>,
) {
    paint_table_cells(ui, weights, values, false, tone, Sense::hover());
}

/// [`paint_table_row`] for a row that answers `sense`, as a row that opens
/// what it names on a double-click does.
pub(super) fn paint_table_row_sensed(
    ui: &mut Ui,
    weights: &[f32],
    values: &[String],
    tone: Option<egui::Color32>,
    sense: Sense,
) -> egui::Response {
    paint_table_cells(ui, weights, values, false, tone, sense)
}

pub(super) fn paint_table_cells(
    ui: &mut Ui,
    weights: &[f32],
    values: &[String],
    header: bool,
    tone: Option<egui::Color32>,
    sense: Sense,
) -> egui::Response {
    let t = Tokens::get(ui.ctx());
    let height = if header { 27.0 } else { 29.0 };
    let (rect, response) = ui.allocate_exact_size(vec2(ui.available_width(), height), sense);
    // A row reads as its cells, for whoever cannot see them.
    if !header {
        response.widget_info(|| {
            egui::WidgetInfo::labeled(egui::WidgetType::Label, true, values.join(", "))
        });
    }
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
    response
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

/// The ranges the Reserved ranges field holds: none when it is empty.
pub(super) fn parse_reserved_ranges(text: &str) -> Result<Vec<AnnotationReservedRange>, String> {
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
    Ok(ranges)
}

/// The Renumber list's columns: the Keep tick, the name a part has, the
/// name it would get, and its sheet.
const RENUMBER_COLUMNS: [f32; 4] = [0.12, 0.26, 0.26, 0.36];
const RENUMBER_ROW_HEIGHT: f32 = 29.0;
/// How tall the list stands before it scrolls.
const RENUMBER_LIST_HEIGHT: f32 = 12.0 * RENUMBER_ROW_HEIGHT;

/// The Renumber page's list: every part in scope in the order it is
/// numbered, the name it has and the name it would get. A new name reads in
/// the body colour; a name that stays, because the part is kept or already
/// has it, is dimmed. Returns the part whose Keep tick was clicked.
pub(super) fn renumber_list(
    ui: &mut Ui,
    table: &RenumberTable,
    write_allowed: bool,
) -> Option<SchematicObjectKey> {
    section_header(
        ui,
        "Names",
        Some(&format!(
            "{} \u{00b7} {}",
            counted(table.changes(), "change"),
            counted(table.rows.len(), "part")
        )),
    );
    paint_table_header(
        ui,
        &RENUMBER_COLUMNS,
        &["Keep", "Name", "New name", "Sheet"],
    );
    if table.rows.is_empty() {
        empty_table_row(
            ui,
            table
                .problem
                .as_deref()
                .unwrap_or("This scope holds no parts to renumber."),
        );
        return None;
    }
    let mut toggled = None;
    egui::ScrollArea::vertical()
        .id_salt("renumber-names")
        .max_height(RENUMBER_LIST_HEIGHT)
        .auto_shrink([false, true])
        .show_rows(ui, RENUMBER_ROW_HEIGHT, table.rows.len(), |ui, rows| {
            for index in rows {
                let row = &table.rows[index];
                if renumber_row(ui, index, row, write_allowed) {
                    toggled = Some(row.object.clone());
                }
            }
        });
    toggled
}

/// One part's row of the Renumber list; whether its Keep tick was clicked.
fn renumber_row(ui: &mut Ui, index: usize, row: &RenumberRow, write_allowed: bool) -> bool {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width();
    let total = RENUMBER_COLUMNS.iter().sum::<f32>();
    let widths = RENUMBER_COLUMNS.map(|weight| width * weight / total);
    let background = ui.painter().add(egui::Shape::Noop);
    let mut kept = row.kept;
    let cells = ui.allocate_ui_with_layout(
        vec2(width, RENUMBER_ROW_HEIGHT),
        Layout::left_to_right(Align::Center),
        |ui| {
            ui.set_min_size(vec2(width, RENUMBER_ROW_HEIGHT));
            ui.allocate_ui_with_layout(
                vec2(widths[0], RENUMBER_ROW_HEIGHT),
                Layout::left_to_right(Align::Center),
                |ui| {
                    ui.set_min_size(vec2(widths[0], RENUMBER_ROW_HEIGHT));
                    ui.add_space(PANE_INSET);
                    ui.add_enabled(write_allowed, egui::Checkbox::without_text(&mut kept))
                },
            )
            .inner
        },
    );
    let tick = cells.inner;
    let rect = cells.response.rect;
    tick.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::Checkbox,
            write_allowed,
            row.kept,
            format!("Keep {}", row.current),
        )
    });
    // The row reads as the change it makes, for whoever cannot see it.
    ui.interact(rect, ui.id().with(("renumber-row", index)), Sense::hover())
        .widget_info(|| {
            egui::WidgetInfo::labeled(
                egui::WidgetType::Label,
                true,
                if row.kept {
                    format!("{} kept", row.current)
                } else {
                    format!("{} \u{2192} {}", row.current, row.new)
                },
            )
        });
    let painter = ui.painter();
    painter.set(
        background,
        egui::Shape::rect_filled(
            rect,
            0.0,
            if ui.rect_contains_pointer(rect) {
                t.color.bg_hover
            } else {
                t.color.bg_inset
            },
        ),
    );
    painter.hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    let stays = row.current == row.new;
    let mut left = rect.left();
    for (column, cell_width) in widths.iter().enumerate() {
        let cell = egui::Rect::from_min_size(
            egui::pos2(left, rect.top()),
            vec2(*cell_width, rect.height()),
        );
        left += cell_width;
        if column == 0 {
            continue;
        }
        painter.vline(
            cell.left(),
            cell.y_range(),
            Stroke::new(1.0, t.color.border),
        );
        let (text, font, color) = match column {
            1 => (
                row.current.as_str(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                t.color.text,
            ),
            2 => (
                row.new.as_str(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                if stays {
                    t.color.text_dim
                } else {
                    t.color.text
                },
            ),
            _ => (
                row.sheet.as_str(),
                theme::sans(tokens::FS_0, FontWeight::Regular),
                t.color.text_dim,
            ),
        };
        painter
            .with_clip_rect(cell.shrink2(vec2(PANE_INSET, 2.0)))
            .text(
                cell.left_center() + vec2(PANE_INSET, 0.0),
                egui::Align2::LEFT_CENTER,
                text,
                font,
                color,
            );
    }
    tick.clicked()
}

//! The global bindings: the top cell, the DUT instance, what an unresolved
//! cell does, the view list and the stop list, as one band of five fields in
//! equal columns. On Default the same band states the built-in lists and
//! edits nothing.

use super::*;

use crate::state::{
    CellViewRef, ConfigurationSetDefinition, DEFAULT_STOP_VIEW_TYPES, UnresolvedBindingPolicy,
    default_view_search_order,
};
use crate::ui::widgets::{
    OrderedChipsEdit, ordered_chips, ordered_chips_width, required_ordered_chips,
    select_mono_with_response,
};
use rspice_ui_kit::panels::PANEL_INSET;
use rspice_ui_kit::widgets::page_bar_height;

/// The least a field of the band is made: its label and a control wide
/// enough to read a cell's path in. Five stand on one row where each is at
/// least this and its list fits on one line; three, then two, from three
/// times this; one under another below.
const FIELD_MIN_WIDTH: f32 = 200.0;
/// The gap between a field's label and its control.
const LABEL_GAP: f32 = 6.0;

const UNRESOLVED_CHOICES: [(UnresolvedBindingPolicy, &str); 2] = [
    (UnresolvedBindingPolicy::BlockNetlist, "Stop the netlist"),
    (
        UnresolvedBindingPolicy::ExplicitFallbackWithReview,
        "Use the default view list and flag the cell",
    ),
];

/// One committed change to the global bindings.
#[derive(Debug, Clone, PartialEq, Eq)]
enum GridEdit {
    TopCell(CellViewRef),
    Dut(String),
    Unresolved(UnresolvedBindingPolicy),
    Views(OrderedChipsEdit),
    Stops(OrderedChipsEdit),
}

/// What a field draws in its share of the band, and the edit it asks for.
type FieldBody<'a> = Box<dyn FnOnce(&mut Ui) -> Option<GridEdit> + 'a>;

/// One field of the band: its label, whether its control wraps onto more
/// lines (a chip list), how wide its control is on one line where that is
/// more than a field's least, and what it draws, with the edit it asks for.
struct Field<'a> {
    label: &'static str,
    wraps: bool,
    needs: f32,
    body: FieldBody<'a>,
}

impl<'a> Field<'a> {
    fn new(label: &'static str, body: impl FnOnce(&mut Ui) -> Option<GridEdit> + 'a) -> Self {
        Self {
            label,
            wraps: false,
            needs: 0.0,
            body: Box::new(body),
        }
    }

    /// A chip list `needs` wide on one line, which wraps where it has less.
    fn chips(
        label: &'static str,
        needs: f32,
        body: impl FnOnce(&mut Ui) -> Option<GridEdit> + 'a,
    ) -> Self {
        Self {
            label,
            wraps: true,
            needs,
            body: Box::new(body),
        }
    }

    /// A value `needs` wide, which is shortened where it has less.
    fn needs(mut self, needs: f32) -> Self {
        self.needs = needs;
        self
    }

    /// The least width the field is given in a row: its label and its
    /// control on one line, and never less than a field's least.
    fn least(&self, ui: &Ui) -> f32 {
        let label = ui
            .painter()
            .layout_no_wrap(self.label.to_owned(), label_font(), egui::Color32::WHITE)
            .size()
            .x;
        (2.0 * PANEL_INSET + label + LABEL_GAP + self.needs).max(FIELD_MIN_WIDTH)
    }
}

fn label_font() -> egui::FontId {
    theme::sans(tokens::FS_0, FontWeight::Regular)
}

pub(super) fn global_bindings(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
) {
    let ConfigurationSelection::Configuration(id) = selection else {
        let fields = default_fields(ui, cx);
        band(ui, fields);
        problems(ui, resolution, None);
        return;
    };
    let catalog = &cx.model.workspace.content.configuration_sets;
    let Some(definition) = cx.state.configuration.definition(catalog, id) else {
        return;
    };
    let write_allowed = !cx.model.safe_mode.project_read_only();
    let fields = configuration_fields(ui, definition, resolution, write_allowed);
    let edit = band(ui, fields);
    problems(ui, resolution, Some(definition));
    if let Some(edit) = edit {
        cx.state.configuration.edit(
            &cx.model.workspace.content.configuration_sets,
            id,
            |definition| apply(definition, edit),
        );
    }
}

fn apply(definition: &mut ConfigurationSetDefinition, edit: GridEdit) {
    match edit {
        GridEdit::TopCell(root) => definition.root = root,
        GridEdit::Dut(path) => definition.dut_path = path,
        GridEdit::Unresolved(policy) => definition.unresolved_policy = policy,
        GridEdit::Views(edit) => {
            edit.apply(&mut definition.executable_view_policy);
            // A stop is a view the list descends no further at, so a view
            // the list drops cannot stay a stop.
            let views = definition.executable_view_policy.clone();
            definition
                .stop_views
                .retain(|stop| views.iter().any(|view| view.eq_ignore_ascii_case(stop)));
        }
        GridEdit::Stops(edit) => {
            edit.apply(&mut definition.stop_views);
        }
    }
}

/// The band: `fields` in equal columns, all five on one row where each fits
/// its share on one line, else the three choices on one row and the two
/// lists on the next, else one under another. Every row is as tall as its
/// tallest field, and no cell of the band is left empty.
fn band(ui: &mut Ui, fields: Vec<Field<'_>>) -> Option<GridEdit> {
    let width = visible_workspace_width(ui);
    let count = fields.len();
    let share = width / count.max(1) as f32;
    let rows = if fields.iter().all(|field| share >= field.least(ui)) {
        vec![count]
    } else if width >= 3.0 * FIELD_MIN_WIDTH {
        vec![3, count.saturating_sub(3)]
    } else {
        vec![1; count]
    };
    let last = rows.len().saturating_sub(1);
    let mut fields = fields.into_iter();
    let mut edit = None;
    for (index, take) in rows.into_iter().enumerate() {
        let row = fields.by_ref().take(take).collect::<Vec<_>>();
        if let Some(asked) = band_row(ui, width, row, index == last) {
            edit = Some(asked);
        }
    }
    edit
}

/// One row of the band: each field's label, then its control in what the
/// label leaves of an equal share of `width`, a rule between fields, and
/// under the row the band's rule, or its closing rule under the last.
fn band_row(ui: &mut Ui, width: f32, fields: Vec<Field<'_>>, closing: bool) -> Option<GridEdit> {
    let t = Tokens::get(ui.ctx());
    let least = page_bar_height(ui);
    // A wrapping control centres each line on a control's height; starting
    // it that much below the top puts its first line on the label's.
    let inset = ((least - t.metrics.ctl_h) * 0.5).max(0.0);
    let font = label_font();
    let origin = ui.cursor().min;
    let share = (width / fields.len().max(1) as f32).floor();
    let mut bottom = origin.y + least;
    let mut labels = Vec::new();
    let mut edit = None;
    let count = fields.len();
    for (index, field) in fields.into_iter().enumerate() {
        let left = origin.x + share * index as f32;
        let right = if index + 1 == count {
            origin.x + width
        } else {
            left + share
        };
        let galley =
            ui.painter()
                .layout_no_wrap(field.label.to_owned(), font.clone(), t.color.text_dim);
        let body_left = left + PANEL_INSET + galley.size().x + LABEL_GAP;
        let body = Rect::from_min_max(
            pos2(body_left, origin.y + inset),
            pos2(
                (right - PANEL_INSET).max(body_left + 1.0),
                origin.y + least - inset,
            ),
        );
        let layout = if field.wraps {
            Layout::left_to_right(Align::Center).with_main_wrap(true)
        } else {
            Layout::left_to_right(Align::Center)
        };
        let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body).layout(layout));
        child.spacing_mut().item_spacing = vec2(6.0, 4.0);
        if let Some(asked) = (field.body)(&mut child) {
            edit = Some(asked);
        }
        bottom = bottom.max(child.min_rect().bottom() + inset);
        labels.push((left, field.label, galley));
    }
    let (rect, _) = ui.allocate_exact_size(vec2(width, bottom - origin.y), Sense::hover());
    for (index, (left, label, galley)) in labels.into_iter().enumerate() {
        let at = pos2(
            left + PANEL_INSET,
            rect.top() + (least - galley.size().y) * 0.5,
        );
        let said = ui.interact(
            Rect::from_min_size(at, galley.size()),
            ui.id().with(("configuration-band", label)),
            Sense::hover(),
        );
        said.widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), label));
        ui.painter().galley(at, galley, t.color.text_dim);
        if index > 0 {
            ui.painter().vline(
                left.round() + 0.5,
                rect.y_range(),
                Stroke::new(1.0, t.color.border),
            );
        }
    }
    ui.painter().hline(
        rect.x_range(),
        rect.bottom() - 0.5,
        Stroke::new(
            1.0,
            if closing {
                t.color.border_strong
            } else {
                t.color.border
            },
        ),
    );
    edit
}

/// A read-only value in a field: on one line, shortened to the field with
/// the whole value on hover.
fn value(ui: &mut Ui, text: &str, mono: bool) {
    let t = Tokens::get(ui.ctx());
    ui.add(
        egui::Label::new(
            egui::RichText::new(text)
                .font(if mono {
                    theme::mono(tokens::FS_0, FontWeight::Regular)
                } else {
                    theme::sans(tokens::FS_0, FontWeight::Regular)
                })
                .color(t.color.text),
        )
        .truncate(),
    );
}

/// The built-in Default, stated field for field in the same band.
fn default_fields(ui: &Ui, cx: &Cx<'_, ProjectFeature>) -> Vec<Field<'static>> {
    let project = &cx.model.workspace.content.project;
    let top = format!("{}/{}/schematic", project.root_library, project.top_cell);
    let order = default_view_search_order("schematic", false);
    let mut words = vec!["the instance's own view".to_owned()];
    words.extend(order.into_iter().skip(1));
    let view_list = words.join(", then ");
    let stops = DEFAULT_STOP_VIEW_TYPES
        .iter()
        .map(|view_type| view_type.display_name().to_owned())
        .collect::<Vec<_>>();
    let view_list_width = ui
        .painter()
        .layout_no_wrap(
            view_list.clone(),
            theme::sans(tokens::FS_0, FontWeight::Regular),
            egui::Color32::WHITE,
        )
        .size()
        .x;
    let stops_width = ordered_chips_width(ui, &stops, false, false, false);
    vec![
        Field::new("Top cell", move |ui| {
            value(ui, &top, true);
            None
        }),
        Field::new("DUT", |ui| {
            value(ui, "\u{2014}", false);
            None
        }),
        Field::new("Unresolved", |ui| {
            value(ui, UNRESOLVED_CHOICES[0].1, false);
            None
        }),
        Field::new("View list", move |ui| {
            value(ui, &view_list, false);
            None
        })
        .needs(view_list_width),
        Field::chips("Stop list", stops_width, move |ui| {
            ordered_chips(
                ui,
                "default-stop-list",
                "Stop list",
                "View",
                &stops,
                &[],
                false,
            );
            None
        }),
    ]
}

/// A configuration's global bindings, each edited in its field.
fn configuration_fields<'a>(
    ui: &Ui,
    definition: &'a ConfigurationSetDefinition,
    resolution: &'a ConfigurationResolution,
    write_allowed: bool,
) -> Vec<Field<'a>> {
    let tables = &resolution.tables;
    let views_width = ordered_chips_width(
        ui,
        &definition.executable_view_policy,
        !tables.addable_views.is_empty(),
        write_allowed,
        true,
    );
    let stops_width = ordered_chips_width(
        ui,
        &definition.stop_views,
        !tables.addable_stops.is_empty(),
        write_allowed,
        false,
    );
    vec![
        Field::new("Top cell", move |ui| {
            let choices = &tables.top_cell_choices;
            let width = ui.available_width();
            ui.add_enabled_ui(write_allowed, |ui| {
                select_mono_with_response(
                    ui,
                    "configuration-top-cell",
                    "Top cell",
                    &definition.root.display_path(),
                    &tables.top_cell_labels,
                    width,
                )
                .picked
                .filter(|index| choices[*index] != definition.root)
                .map(|index| GridEdit::TopCell(choices[index].clone()))
            })
            .inner
        }),
        Field::new("DUT", move |ui| {
            let choices = &tables.dut_choices;
            let width = ui.available_width();
            ui.add_enabled_ui(write_allowed, |ui| {
                select_mono_with_response(
                    ui,
                    "configuration-dut-instance",
                    "DUT instance",
                    &definition.dut_path,
                    choices,
                    width,
                )
                .picked
                .filter(|index| choices[*index] != definition.dut_path)
                .map(|index| GridEdit::Dut(choices[index].clone()))
            })
            .inner
        }),
        Field::new("Unresolved", move |ui| {
            let labels = UNRESOLVED_CHOICES
                .iter()
                .map(|(_, label)| (*label).to_owned())
                .collect::<Vec<_>>();
            let current = UNRESOLVED_CHOICES
                .iter()
                .find(|(policy, _)| *policy == definition.unresolved_policy)
                .map_or(UNRESOLVED_CHOICES[0].1, |(_, label)| label);
            let width = ui.available_width();
            ui.add_enabled_ui(write_allowed, |ui| {
                crate::ui::widgets::select(
                    ui,
                    "configuration-unresolved-cells",
                    "Unresolved cells",
                    current,
                    &labels,
                    width,
                )
                .filter(|index| UNRESOLVED_CHOICES[*index].0 != definition.unresolved_policy)
                .map(|index| GridEdit::Unresolved(UNRESOLVED_CHOICES[index].0))
            })
            .inner
        }),
        Field::chips("View list", views_width, move |ui| {
            required_ordered_chips(
                ui,
                "configuration-view-list",
                "View list",
                "View",
                &definition.executable_view_policy,
                &tables.addable_views,
                write_allowed,
            )
            .map(GridEdit::Views)
        }),
        Field::chips("Stop list", stops_width, move |ui| {
            // Only a view the list searches can stop it.
            let chosen = ordered_chips(
                ui,
                "configuration-stop-list",
                "Stop list",
                "View",
                &definition.stop_views,
                &tables.addable_stops,
                write_allowed,
            );
            if definition.stop_views.is_empty() && !write_allowed {
                value(ui, "\u{2014}", true);
            }
            chosen.map(GridEdit::Stops)
        }),
    ]
}

/// What stops the edited configuration from binding the design, said once
/// under the grid with what to do about it.
fn problems(
    ui: &mut Ui,
    resolution: &ConfigurationResolution,
    definition: Option<&ConfigurationSetDefinition>,
) {
    let mut lines = Vec::new();
    match &resolution.hierarchy {
        Err(error) => lines.push(format!("The design does not resolve: {error}")),
        Ok(_) => {
            if let Some(definition) = definition
                && !resolution.tables.dut_in_design
            {
                lines.push(format!(
                    "DUT instance {} is not in {}. Pick one of its instances.",
                    definition.dut_path,
                    definition.root.display_path()
                ));
            }
        }
    }
    if lines.is_empty() {
        return;
    }
    egui::Frame::new()
        .inner_margin(Margin::symmetric(10, 7))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            for line in lines {
                problem_mark_line(ui, Problem::Failure, &line);
            }
        });
}

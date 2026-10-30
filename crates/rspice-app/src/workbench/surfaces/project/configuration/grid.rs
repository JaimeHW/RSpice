//! The global bindings: the top cell, the DUT instance, the view list, the
//! stop list and what an unresolved cell does. On Default the same grid
//! states the built-in lists and edits nothing.

use super::*;

use crate::state::{
    CellViewRef, ConfigurationSetDefinition, DEFAULT_STOP_VIEW_TYPES, UnresolvedBindingPolicy,
    default_view_search_order,
};
use crate::ui::widgets::{OrderedChipsEdit, ordered_chips, select_mono_with_response};

const LABEL_WIDTH: f32 = 112.0;
const ROW_HEIGHT: f32 = 32.0;
const PICKER_WIDTH: f32 = 300.0;
/// From this width the five rows sit in two columns.
const TWO_COLUMN_WIDTH: f32 = 760.0;

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

pub(super) fn global_bindings(
    ui: &mut Ui,
    state: &mut AppState,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
) {
    let ConfigurationSelection::Configuration(id) = selection else {
        section_header(ui, "Global bindings", "built in");
        default_grid(ui, state);
        problems(ui, resolution, None);
        return;
    };
    section_header(ui, "Global bindings", "");
    let catalog = &state.model.workspace.content.configuration_sets;
    let Some(definition) = state.project.configuration.definition(catalog, id) else {
        return;
    };
    let write_allowed = !state.model.safe_mode.project_read_only();
    let edit = configuration_grid(ui, definition, resolution, write_allowed);
    problems(ui, resolution, Some(definition));
    if let Some(edit) = edit {
        state.project.configuration.edit(
            &state.model.workspace.content.configuration_sets,
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

/// Lay out `rows` in two columns when the page is wide enough, one below
/// the other otherwise.
fn columns(ui: &mut Ui, left: impl FnOnce(&mut Ui), right: impl FnOnce(&mut Ui)) {
    let width = visible_workspace_width(ui);
    if width < TWO_COLUMN_WIDTH {
        left(ui);
        right(ui);
        return;
    }
    let half = (width * 0.5).floor();
    let shown = ui.horizontal_top(|ui| {
        ui.allocate_ui_with_layout(vec2(half, 0.0), Layout::top_down(Align::Min), |ui| {
            ui.set_width(half);
            left(ui);
        });
        ui.allocate_ui_with_layout(
            vec2(width - half, 0.0),
            Layout::top_down(Align::Min),
            |ui| {
                ui.set_width(width - half);
                right(ui);
            },
        );
    });
    ui.painter().vline(
        shown.response.rect.left() + half + 0.5,
        shown.response.rect.y_range(),
        Stroke::new(1.0, Tokens::get(ui.ctx()).color.border),
    );
}

/// One grid row: its label, then whatever `body` draws, centred on the
/// row's single line. A body that wraps (a long chip list) makes the row
/// taller and keeps its first line on the label's.
fn grid_row<R>(ui: &mut Ui, label: &str, body: impl FnOnce(&mut Ui) -> R) -> R {
    labelled_row(ui, label, false, body)
}

/// A grid row whose body is a chip list, which wraps onto more lines when
/// the list outgrows the row.
fn chip_row<R>(ui: &mut Ui, label: &str, body: impl FnOnce(&mut Ui) -> R) -> R {
    labelled_row(ui, label, true, body)
}

fn labelled_row<R>(ui: &mut Ui, label: &str, wrap: bool, body: impl FnOnce(&mut Ui) -> R) -> R {
    let t = Tokens::get(ui.ctx());
    let width = ui.available_width().max(1.0);
    let top = ui.cursor().min;
    let body_left = top.x + 10.0 + LABEL_WIDTH;
    // A wrapping body centres each line on its tallest control, a control's
    // height; starting it that much below the top puts its first line on the
    // label's.
    let inset = if wrap {
        ((ROW_HEIGHT - t.metrics.ctl_h) * 0.5).max(0.0)
    } else {
        0.0
    };
    let body_rect = Rect::from_min_max(
        pos2(body_left, top.y + inset),
        pos2(
            (top.x + width - 10.0).max(body_left + 1.0),
            top.y + ROW_HEIGHT - inset,
        ),
    );
    let layout = if wrap {
        Layout::left_to_right(Align::Center).with_main_wrap(true)
    } else {
        Layout::left_to_right(Align::Center)
    };
    let mut child = ui.new_child(egui::UiBuilder::new().max_rect(body_rect).layout(layout));
    child.spacing_mut().item_spacing = vec2(6.0, 4.0);
    let inner = body(&mut child);
    let used = child.min_rect().bottom() + inset - top.y;
    let height = used.max(ROW_HEIGHT);
    let (rect, response) = ui.allocate_exact_size(vec2(width, height), Sense::hover());
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Label, ui.is_enabled(), label));
    ui.painter().text(
        pos2(rect.left() + 10.0, rect.top() + ROW_HEIGHT * 0.5),
        Align2::LEFT_CENTER,
        label,
        theme::sans(tokens::FS_0, FontWeight::Regular),
        t.color.text_dim,
    );
    ui.painter().hline(
        rect.x_range(),
        rect.bottom(),
        Stroke::new(1.0, t.color.border),
    );
    inner
}

/// A read-only value in a grid row.
fn value(ui: &mut Ui, text: &str, mono: bool) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(if mono {
                theme::mono(tokens::FS_0, FontWeight::Regular)
            } else {
                theme::sans(tokens::FS_0, FontWeight::Regular)
            })
            .color(t.color.text),
    );
}

/// The built-in Default, stated row for row in the same grid.
fn default_grid(ui: &mut Ui, state: &AppState) {
    let project = &state.model.workspace.content.project;
    let top = format!("{}/{}/schematic", project.root_library, project.top_cell);
    let order = default_view_search_order("schematic", false);
    let mut words = vec!["the instance's own view".to_owned()];
    words.extend(order.into_iter().skip(1));
    let view_list = words.join(", then ");
    let stops = DEFAULT_STOP_VIEW_TYPES
        .iter()
        .map(|view_type| view_type.display_name().to_owned())
        .collect::<Vec<_>>();
    columns(
        ui,
        |ui| {
            grid_row(ui, "Top cell", |ui| value(ui, &top, true));
            grid_row(ui, "DUT instance", |ui| value(ui, "\u{2014}", false));
            grid_row(ui, "Unresolved cells", |ui| {
                value(ui, UNRESOLVED_CHOICES[0].1, false);
            });
        },
        |ui| {
            grid_row(ui, "View list", |ui| value(ui, &view_list, false));
            chip_row(ui, "Stop list", |ui| {
                ordered_chips(
                    ui,
                    "default-stop-list",
                    "Stop list",
                    "View",
                    &stops,
                    &[],
                    false,
                );
            });
        },
    );
}

fn configuration_grid(
    ui: &mut Ui,
    definition: &ConfigurationSetDefinition,
    resolution: &ConfigurationResolution,
    write_allowed: bool,
) -> Option<GridEdit> {
    let mut left_edit = None;
    let mut right_edit = None;
    let picker_width = |ui: &Ui| PICKER_WIDTH.min(ui.available_width()).max(120.0);
    columns(
        ui,
        |ui| {
            grid_row(ui, "Top cell", |ui| {
                let mut choices = resolution.top_cells.clone();
                if !choices.contains(&definition.root) {
                    choices.insert(0, definition.root.clone());
                }
                let labels = choices
                    .iter()
                    .map(CellViewRef::display_path)
                    .collect::<Vec<_>>();
                let width = picker_width(ui);
                ui.add_enabled_ui(write_allowed, |ui| {
                    if let Some(index) = select_mono_with_response(
                        ui,
                        "configuration-top-cell",
                        "Top cell",
                        &definition.root.display_path(),
                        &labels,
                        width,
                    )
                    .picked
                        && choices[index] != definition.root
                    {
                        left_edit = Some(GridEdit::TopCell(choices[index].clone()));
                    }
                });
            });
            grid_row(ui, "DUT instance", |ui| {
                let mut choices = resolution
                    .instances
                    .iter()
                    .filter(|row| row.depth > 0)
                    .map(|row| row.path.clone())
                    .collect::<Vec<_>>();
                if !choices
                    .iter()
                    .any(|path| path.eq_ignore_ascii_case(&definition.dut_path))
                {
                    choices.insert(0, definition.dut_path.clone());
                }
                let width = picker_width(ui);
                ui.add_enabled_ui(write_allowed, |ui| {
                    if let Some(index) = select_mono_with_response(
                        ui,
                        "configuration-dut-instance",
                        "DUT instance",
                        &definition.dut_path,
                        &choices,
                        width,
                    )
                    .picked
                        && choices[index] != definition.dut_path
                    {
                        left_edit = Some(GridEdit::Dut(choices[index].clone()));
                    }
                });
            });
            grid_row(ui, "Unresolved cells", |ui| {
                let labels = UNRESOLVED_CHOICES
                    .iter()
                    .map(|(_, label)| (*label).to_owned())
                    .collect::<Vec<_>>();
                let current = UNRESOLVED_CHOICES
                    .iter()
                    .find(|(policy, _)| *policy == definition.unresolved_policy)
                    .map_or(UNRESOLVED_CHOICES[0].1, |(_, label)| label);
                let width = (PICKER_WIDTH + 40.0).min(ui.available_width()).max(120.0);
                ui.add_enabled_ui(write_allowed, |ui| {
                    if let Some(index) = crate::ui::widgets::select(
                        ui,
                        "configuration-unresolved-cells",
                        "Unresolved cells",
                        current,
                        &labels,
                        width,
                    ) && UNRESOLVED_CHOICES[index].0 != definition.unresolved_policy
                    {
                        left_edit = Some(GridEdit::Unresolved(UNRESOLVED_CHOICES[index].0));
                    }
                });
            });
        },
        |ui| {
            chip_row(ui, "View list", |ui| {
                let addable = resolution
                    .view_names
                    .iter()
                    .filter(|name| {
                        !definition
                            .executable_view_policy
                            .iter()
                            .any(|view| view.eq_ignore_ascii_case(name))
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                if let Some(chosen) = ordered_chips(
                    ui,
                    "configuration-view-list",
                    "View list",
                    "View",
                    &definition.executable_view_policy,
                    &addable,
                    write_allowed,
                ) {
                    right_edit = Some(GridEdit::Views(chosen));
                }
            });
            chip_row(ui, "Stop list", |ui| {
                // Only a view the list searches can stop it.
                let addable = definition
                    .executable_view_policy
                    .iter()
                    .filter(|view| {
                        !definition
                            .stop_views
                            .iter()
                            .any(|stop| stop.eq_ignore_ascii_case(view))
                    })
                    .cloned()
                    .collect::<Vec<_>>();
                if let Some(chosen) = ordered_chips(
                    ui,
                    "configuration-stop-list",
                    "Stop list",
                    "View",
                    &definition.stop_views,
                    &addable,
                    write_allowed,
                ) {
                    right_edit = Some(GridEdit::Stops(chosen));
                }
                if definition.stop_views.is_empty() && !write_allowed {
                    value(ui, "\u{2014}", true);
                }
            });
        },
    );
    left_edit.or(right_edit)
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
                && resolution.instance(&definition.dut_path).is_none()
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

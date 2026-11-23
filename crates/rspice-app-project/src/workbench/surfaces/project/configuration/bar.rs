//! The Configuration page's one bar. On the left, the configuration in front
//! of the reader and what is done to it: the picker, the unsaved dot while
//! it has edits not applied, Apply, Revert and More; then the table the page
//! shows, the table's filter and, on Instances, Add pattern binding. On the
//! right, how much of the design the configuration binds, and whether runs
//! use it.

use super::*;
use crate::ui::accessibility::{counted, grouped};
use crate::ui::widgets::{SegmentedWidth, segmented};
use crate::workbench::app::dialogs::project::configuration_dialogs::ConfigurationDialogRequest;
use crate::workbench::project::configuration_state::{ConfigurationTab, used_for_runs};
use rspice_ui_kit::widgets::{PageBar, PageBarFilter};

use super::instances::TableAction;

/// What the bar asked for this frame.
#[derive(Debug, Clone, PartialEq)]
pub(super) enum BarIntent {
    /// Put this configuration in front, asking first about edits that are
    /// not applied.
    Select(ConfigurationSelection),
    UseForRuns(ConfigurationSelection),
    Apply,
    Revert,
    Open(ConfigurationDialogRequest),
    Table(TableAction),
}

/// What the dot beside the picker says, and the hover over it.
pub(super) const EDITS_NOT_APPLIED: &str = "Edits not applied";
const READ_ONLY: &str = "The project is open read-only.";
const CHECKING: &str = "The edits are being checked.";
/// The widest the picker shows a configuration's name; a longer one is
/// shortened, whole on hover and in the list.
const PICKER_NAME_WIDTH: f32 = 220.0;
/// The unsaved dot: the table's, so an unapplied edit reads alike wherever
/// it is marked.
const UNSAVED_RADIUS: f32 = 3.0;

/// One configuration the picker lists.
struct Choice {
    selection: ConfigurationSelection,
    name: String,
    used_for_runs: bool,
    unsaved: bool,
}

/// What the bar states and offers this frame.
struct BarState {
    selection: ConfigurationSelection,
    name: String,
    used_for_runs: bool,
    write_allowed: bool,
    /// The configuration in front has edits that are not applied.
    dirty: bool,
    checking: bool,
    out_of_date: bool,
    /// Add pattern binding is offered: an editable configuration on
    /// Instances.
    adds: bool,
    tab: ConfigurationTab,
}

impl BarState {
    const fn custom(&self) -> bool {
        matches!(self.selection, ConfigurationSelection::Configuration(_))
    }

    /// Why Apply is not offered, or `None` while it is.
    const fn apply_refusal(&self) -> Option<&'static str> {
        if !self.write_allowed {
            Some(READ_ONLY)
        } else if self.checking {
            Some(CHECKING)
        } else if self.out_of_date {
            Some(
                "The configuration changed in a part these edits change too. Revert, then edit \
                 again.",
            )
        } else if !self.dirty {
            Some("There are no edits to apply.")
        } else {
            None
        }
    }

    /// Why Use for runs is not offered, or `None` while it is.
    const fn runs_refusal(&self) -> Option<&'static str> {
        if self.used_for_runs {
            Some("Runs use this configuration already.")
        } else if !self.write_allowed {
            Some(READ_ONLY)
        } else if self.checking {
            Some(CHECKING)
        } else {
            None
        }
    }
}

pub(super) fn configuration_bar(
    ui: &mut Ui,
    cx: &mut Cx<'_, ProjectFeature>,
    selection: ConfigurationSelection,
    resolution: &ConfigurationResolution,
) -> Option<BarIntent> {
    let catalog = &cx.model.workspace.content.configuration_sets;
    let page = &cx.state.configuration;
    let write_allowed = !cx.model.safe_mode.project_read_only();
    let editable = match selection {
        ConfigurationSelection::Configuration(id) => page.definition(catalog, id).is_some(),
        ConfigurationSelection::Default => false,
    } && write_allowed;
    let state = BarState {
        selection,
        name: selection_name(cx, selection),
        used_for_runs: used_for_runs(catalog, selection),
        write_allowed,
        dirty: resolution.tables.dirty,
        checking: page.is_checking(),
        out_of_date: page.is_out_of_date(catalog),
        adds: editable && page.tab == ConfigurationTab::Instances,
        tab: page.tab,
    };
    let edited = page
        .is_dirty(catalog)
        .then(|| page.draft().map(|draft| draft.id))
        .flatten();
    let choices = std::iter::once((ConfigurationSelection::Default, "Default".to_owned()))
        .chain(catalog.configurations().iter().map(|configuration| {
            (
                ConfigurationSelection::Configuration(configuration.id()),
                configuration.name().to_owned(),
            )
        }))
        .map(|(target, name)| Choice {
            selection: target,
            name,
            used_for_runs: used_for_runs(catalog, target),
            unsaved: edited.is_some_and(|id| target == ConfigurationSelection::Configuration(id)),
        })
        .collect::<Vec<_>>();
    let (report, problem) = report(&state, resolution);
    let page = &mut cx.state.configuration;
    PageBar::new().report(report, problem).show(ui, |ui| {
        let mut asked = picker(ui, &state, &choices).map(BarIntent::Select);
        if state.dirty {
            unsaved_dot(ui);
        }
        if state.custom() {
            if bar_button(ui, "Apply", state.apply_refusal()) {
                asked = Some(BarIntent::Apply);
            }
            if bar_button(
                ui,
                "Revert",
                (!state.dirty).then_some("There are no edits to revert."),
            ) {
                asked = Some(BarIntent::Revert);
            }
        }
        if let Some(intent) = more(ui, &state) {
            asked = Some(intent);
        }
        divider(ui);
        let mut index = match state.tab {
            ConfigurationTab::Instances => 0,
            ConfigurationTab::Cells => 1,
        };
        if segmented(
            ui,
            "configuration-tables",
            &["Instances", "Cells"],
            &mut index,
            SegmentedWidth::Natural,
        ) {
            asked = Some(BarIntent::Table(TableAction::Tab(if index == 0 {
                ConfigurationTab::Instances
            } else {
                ConfigurationTab::Cells
            })));
        }
        let (filter, name) = match state.tab {
            ConfigurationTab::Instances => (&mut page.instances_table.filter, "Filter instances"),
            ConfigurationTab::Cells => (&mut page.cells_table.filter, "Filter cells"),
        };
        PageBarFilter::new(("workbench.project.configuration", name), name).show(ui, filter);
        if state.adds && bar_button(ui, "Add pattern binding", None) {
            asked = Some(BarIntent::Table(TableAction::AddPattern));
        }
        asked
    })
}

/// The configuration in front, as a button that lists every configuration:
/// Default first, the one runs use saying so, one with edits not applied
/// carrying the unsaved dot. Returns the configuration picked.
fn picker(ui: &mut Ui, state: &BarState, choices: &[Choice]) -> Option<ConfigurationSelection> {
    let t = Tokens::get(ui.ctx());
    let font = theme::sans(tokens::FS_1, FontWeight::Regular);
    let shown = elide_text(ui, &state.name, &font, PICKER_NAME_WIDTH);
    let spoken = format!("Configuration: {}", state.name);
    let button = Button::new(&shown)
        .menu()
        .accessible_label(&spoken)
        .show(ui);
    let button = if shown == state.name {
        button
    } else {
        button.on_hover_text(&state.name)
    };
    let mut picked = None;
    egui::Popup::menu(&button).show(|ui| {
        ui.set_min_width(button.rect.width().max(200.0));
        for choice in choices {
            let selected = choice.selection == state.selection;
            // The dot is a slot of the item's own layout after the name, so
            // it stands clear of the name's last letter in any face.
            let dot = ui.id().with(("unsaved", choice.name.as_str()));
            let mut item = if choice.unsaved {
                egui::Button::selectable(
                    selected,
                    (
                        &*choice.name,
                        egui::Atom::custom(dot, egui::Vec2::splat(2.0 * UNSAVED_RADIUS)),
                    ),
                )
            } else {
                egui::Button::selectable(selected, &*choice.name)
            };
            if choice.used_for_runs {
                item = item.shortcut_text("used for runs");
            }
            let response = item.atom_ui(ui);
            if choice.unsaved {
                if let Some(slot) = response.rect(dot) {
                    ui.painter()
                        .circle_filled(slot.center(), UNSAVED_RADIUS, t.color.text_dim);
                }
                let spoken = format!("{}, unsaved changes", choice.name);
                ui.ctx()
                    .accesskit_node_builder(response.id, |node| node.set_label(spoken));
            }
            if response.clicked() {
                picked = Some(choice.selection);
                ui.close();
            }
        }
    });
    picked
}

/// The unsaved dot beside the picker: the configuration in front has edits
/// that are not applied.
fn unsaved_dot(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, response) =
        ui.allocate_exact_size(vec2(2.0 * UNSAVED_RADIUS, t.metrics.ctl_h), Sense::hover());
    response
        .widget_info(|| WidgetInfo::labeled(WidgetType::Label, ui.is_enabled(), EDITS_NOT_APPLIED));
    ui.painter()
        .circle_filled(rect.center(), UNSAVED_RADIUS, t.color.text_dim);
    let _ = response.on_hover_text(EDITS_NOT_APPLIED);
}

/// More: New…, and for a configuration of the project's own Copy…, Rename…
/// and Delete…; then Use for runs.
fn more(ui: &mut Ui, state: &BarState) -> Option<BarIntent> {
    let side = Tokens::get(ui.ctx()).metrics.ctl_h;
    let more = crate::ui::panels::icon_button(
        ui,
        WorkbenchIcon::More,
        "More configuration actions",
        false,
        vec2(side, side),
    );
    let write = (!state.write_allowed).then_some(READ_ONLY);
    let mut asked = None;
    egui::Popup::menu(&more).show(|ui| {
        ui.set_min_width(160.0);
        if menu_item(ui, "New\u{2026}", write) {
            asked = Some(BarIntent::Open(ConfigurationDialogRequest::New));
        }
        if let ConfigurationSelection::Configuration(id) = state.selection {
            if menu_item(ui, "Copy\u{2026}", write) {
                asked = Some(BarIntent::Open(ConfigurationDialogRequest::Clone(id)));
            }
            if menu_item(ui, "Rename\u{2026}", write) {
                asked = Some(BarIntent::Open(ConfigurationDialogRequest::Rename(id)));
            }
            if menu_item(ui, "Delete\u{2026}", write) {
                asked = Some(BarIntent::Open(ConfigurationDialogRequest::Delete(id)));
            }
        }
        ui.separator();
        if menu_item(ui, "Use for runs", state.runs_refusal()) {
            asked = Some(BarIntent::UseForRuns(state.selection));
        }
    });
    asked
}

/// The rule between what acts on the configuration and what acts on the
/// table under it.
fn divider(ui: &mut Ui) {
    let t = Tokens::get(ui.ctx());
    let (rect, _) = ui.allocate_exact_size(vec2(1.0, t.metrics.ctl_h), Sense::hover());
    ui.painter().vline(
        rect.center().x,
        rect.y_range(),
        Stroke::new(1.0, t.color.border),
    );
}

/// A button of the bar, disabled with its reason under the pointer.
fn bar_button(ui: &mut Ui, label: &str, refusal: Option<&str>) -> bool {
    let response = Button::new(label).enabled(refusal.is_none()).show(ui);
    let response = match refusal {
        Some(refusal) => response.on_disabled_hover_text(refusal),
        None => response,
    };
    response.clicked()
}

/// One item of a menu, disabled with its reason under the pointer; taking
/// it closes the menu.
fn menu_item(ui: &mut Ui, label: &str, refusal: Option<&str>) -> bool {
    let taken = ui
        .add_enabled(refusal.is_none(), egui::Button::new(label))
        .on_disabled_hover_text(refusal.unwrap_or_default())
        .clicked();
    if taken {
        ui.close();
    }
    taken
}

/// What the bar says on its right, the fact that matters most last: whether
/// runs use the configuration, on Cells the bindings that bind nothing,
/// what binds by fallback, the edits' state, and how many instances are
/// bound, behind a problem's mark while any is not.
fn report(state: &BarState, resolution: &ConfigurationResolution) -> (String, Option<Problem>) {
    let tables = &resolution.tables;
    let mut facts = Vec::new();
    if state.used_for_runs {
        facts.push("used for runs".to_owned());
    }
    if state.tab == ConfigurationTab::Cells && tables.bindings_not_in_design > 0 {
        facts.push(format!(
            "{} not in this design",
            counted(tables.bindings_not_in_design, "binding", "bindings")
        ));
    }
    if tables.fallback_instances > 0 {
        facts.push(format!(
            "{} bound by fallback",
            grouped(tables.fallback_instances)
        ));
    }
    if state.checking {
        facts.push("checking\u{2026}".to_owned());
    } else if state.out_of_date {
        facts.push("edits out of date".to_owned());
    }
    let (count, problem) = if resolution.hierarchy.is_err() {
        (
            "hierarchy does not resolve".to_owned(),
            Some(Problem::Failure),
        )
    } else {
        let total = tables.total_instances;
        let unbound = tables.unbound_instances;
        // A count is read as a magnitude: 1,082,401, grouped.
        let of_total = counted(total, "instance", "instances");
        if unbound == 0 {
            (format!("{} of {of_total} bound", grouped(total)), None)
        } else {
            (
                format!("{} of {of_total} unbound", grouped(unbound)),
                Some(Problem::Failure),
            )
        }
    };
    facts.push(count);
    let problem = problem.or((state.out_of_date && !state.checking).then_some(Problem::Warning));
    (facts.join(" \u{b7} "), problem)
}

//! Paths: the path variables, the one table a library kept in a folder of
//! its own and a model path name a root by, `$NAME/…`. This page is that
//! table's only editor. A value set here beats an environment variable of
//! the same name, and the table shows both, with each root a project's
//! libraries name even while nothing sets it.
//!
//! A change goes through the models feature, which applies the table the
//! way the model-path dialog once applied its copy: kept on disk first, then
//! live, with the model libraries read again through it.

use std::collections::{BTreeSet, HashMap};

use egui::{Sense, Ui, vec2};

use super::super::preferences_shell::{searchable_row, section_heading, value_column};
use super::{PreferencePageActions, PreferencesDialogs};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{
    Button, PROBLEM_MARK_ADVANCE, PROBLEM_MARK_SIDE, Problem, paint_problem_mark, text_field,
};

/// Where the page keeps its selection and the editor's fields between
/// frames: a pointer's state and a draft, not settings.
const STATE_ID: &str = "preferences.paths";

/// One variable as the table lists it: its value here and in the
/// environment, either of which may be missing.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Variable {
    name: String,
    preferences: Option<String>,
    environment: Option<String>,
}

/// What the page keeps between frames.
#[derive(Debug, Clone, Default)]
struct Draft {
    selected: Option<String>,
    name: String,
    value: String,
}

pub fn page(ui: &mut Ui, state: &mut PreferencesDialogs<'_>, actions: &mut PreferencePageActions) {
    section_heading(ui, "Path variables");
    searchable_row(
        ui,
        "Roots",
        "A library or model path names its root as $NAME. A value set here beats the environment's.",
        &[
            "root",
            "variable",
            "environment",
            "library",
            "model",
            "pdk",
            "folder",
        ],
        |ui| {
            value_column(ui, |ui| {
                if cfg!(target_arch = "wasm32") {
                    note(
                        ui,
                        "A browser reads no folder by path, so no root is read here.",
                    );
                    return;
                }
                let variables = variables(state);
                let id = egui::Id::new(STATE_ID);
                let mut draft = ui
                    .data(|data| data.get_temp::<Draft>(id))
                    .unwrap_or_default();
                let edit = table(ui, &variables, &mut draft);
                ui.data_mut(|data| data.insert_temp(id, draft.clone()));
                let Some(edit) = edit else {
                    return;
                };
                let mut next = state.model.pdk_config.environment_variables.clone();
                match edit {
                    Edit::Set(name, value) => {
                        if let Some(refusal) = path_variable_refusal(&name, &value) {
                            state.prompt(refusal);
                            return;
                        }
                        next.insert(name.clone(), value);
                        draft = Draft {
                            selected: Some(name),
                            ..Draft::default()
                        };
                    }
                    Edit::Remove(name) => {
                        next.remove(&name);
                        draft.selected = None;
                    }
                }
                ui.data_mut(|data| data.insert_temp(id, draft));
                actions.path_variables = Some(next);
            });
        },
    );
}

/// Restore defaults: no value set here, so every root is the environment's.
pub fn restore_defaults(state: &PreferencesDialogs<'_>, actions: &mut PreferencePageActions) {
    if !state.model.pdk_config.environment_variables.is_empty() {
        actions.path_variables = Some(HashMap::new());
    }
}

/// Why `value` cannot be root `name`'s, if it cannot: a name is letters,
/// digits and underscores, not led by a digit; a value is an absolute path
/// and names no other root.
pub fn path_variable_refusal(name: &str, value: &str) -> Option<String> {
    let mut letters = name.chars();
    let named = letters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic() || first == '_')
        && letters.all(|rest| rest.is_ascii_alphanumeric() || rest == '_');
    if name.is_empty() {
        return Some("Name the variable.".to_owned());
    }
    if !named {
        return Some(format!(
            "'{name}' is not a variable's name: use letters, digits and underscores, not led by a digit."
        ));
    }
    if value.is_empty() {
        return Some(format!("Give ${name} a folder."));
    }
    if value.contains('$') {
        return Some(format!("${name} cannot name another root."));
    }
    if !std::path::Path::new(value).is_absolute() {
        return Some(format!("${name} must be an absolute path."));
    }
    None
}

/// Every variable set here, and every root the open project's libraries
/// name, by name, with its value here and in the environment.
fn variables(state: &PreferencesDialogs<'_>) -> Vec<Variable> {
    let set = &state.model.pdk_config.environment_variables;
    let mut names = set.keys().cloned().collect::<BTreeSet<_>>();
    names.extend(
        state
            .model
            .library_manager
            .references()
            .filter_map(|reference| reference.path.root_name().map(str::to_owned)),
    );
    names
        .into_iter()
        .map(|name| Variable {
            preferences: set.get(&name).cloned(),
            environment: environment(&name),
            name,
        })
        .collect()
}

#[cfg(not(target_arch = "wasm32"))]
fn environment(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(target_arch = "wasm32")]
fn environment(_: &str) -> Option<String> {
    None
}

/// What a press on the page asks for.
enum Edit {
    /// Set a variable here, adding it or replacing its value.
    Set(String, String),
    /// Take a variable's value here away; the environment's answers again.
    Remove(String),
}

/// The variables, one row each, then the editor under them, every line as
/// wide as the list: the name; the folder, with Browse… on the desktop; and
/// Remove (for the selected row) and Set, of one width, at the right.
fn table(ui: &mut Ui, variables: &[Variable], draft: &mut Draft) -> Option<Edit> {
    let t = Tokens::get(ui.ctx());
    if !variables.is_empty() {
        ui.scope(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            let width = ui.available_width();
            let top = ui.cursor().top();
            for variable in variables {
                let selected = draft.selected.as_deref() == Some(variable.name.as_str());
                if variable_row(ui, variable, selected, width) {
                    draft.selected = Some(variable.name.clone());
                    // Picking a row puts it in the editor, to change it.
                    draft.name.clone_from(&variable.name);
                    draft.value = variable
                        .preferences
                        .clone()
                        .or_else(|| variable.environment.clone())
                        .unwrap_or_default();
                }
            }
            let well = egui::Rect::from_min_max(
                egui::pos2(ui.min_rect().left(), top),
                egui::pos2(ui.min_rect().left() + width, ui.cursor().top()),
            );
            ui.painter().rect_stroke(
                well,
                0.0,
                egui::Stroke::new(1.0, t.color.border),
                egui::StrokeKind::Inside,
            );
        });
        ui.add_space(tokens::SP_3);
    }
    let mut edit = None;
    let height = t.metrics.ctl_h;
    let gap = tokens::SP_3;
    let full = ui.available_width();
    ui.scope(|ui| {
        ui.spacing_mut().item_spacing = vec2(gap, gap);
        text_field(
            ui,
            egui::TextEdit::singleline(&mut draft.name)
                .font(theme::field_mono())
                .hint_text("NAME"),
            full,
        );
        let value = ui
            .horizontal(|ui| {
                #[cfg(not(target_arch = "wasm32"))]
                let browse = Button::new("Browse\u{2026}").min_height(height);
                #[cfg(not(target_arch = "wasm32"))]
                let browse_width = browse.measured_width(ui) + gap;
                #[cfg(target_arch = "wasm32")]
                let browse_width = 0.0;
                let value = text_field(
                    ui,
                    egui::TextEdit::singleline(&mut draft.value)
                        .font(theme::field_mono())
                        .hint_text("Folder"),
                    (full - browse_width).max(0.0),
                );
                #[cfg(not(target_arch = "wasm32"))]
                if browse.show(ui).clicked()
                    && let Some(picked) = rfd::FileDialog::new()
                        .set_title("Pick the root's folder")
                        .pick_folder()
                {
                    draft.value = picked.display().to_string();
                }
                value
            })
            .inner;
        let typed = draft.name.trim().trim_start_matches('$').to_owned();
        let entered = value.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
        let removable = draft.selected.clone().filter(|name| {
            variables
                .iter()
                .any(|variable| &variable.name == name && variable.preferences.is_some())
        });
        let button = Button::new("Remove")
            .measured_width(ui)
            .max(Button::new("Set").measured_width(ui));
        ui.allocate_ui_with_layout(
            vec2(full, height),
            egui::Layout::right_to_left(egui::Align::Center),
            |ui| {
                if (Button::new("Set")
                    .min_height(height)
                    .min_width(button)
                    .enabled(!typed.is_empty())
                    .show(ui)
                    .clicked()
                    || entered)
                    && !typed.is_empty()
                {
                    edit = Some(Edit::Set(typed.clone(), draft.value.trim().to_owned()));
                }
                if Button::new("Remove")
                    .min_height(height)
                    .min_width(button)
                    .enabled(removable.is_some())
                    .destructive(true)
                    .show(ui)
                    .clicked()
                    && let Some(name) = removable.clone()
                {
                    edit = Some(Edit::Remove(name));
                }
            },
        );
    });
    edit
}

/// One variable: `$NAME`, then the value it takes here. A value set here
/// reads plainly; one only the environment gives says so; a root a library
/// names that nothing sets is marked.
fn variable_row(ui: &mut Ui, variable: &Variable, selected: bool, width: f32) -> bool {
    let t = Tokens::get(ui.ctx());
    let (value, source) = match (&variable.preferences, &variable.environment) {
        (Some(value), Some(_)) => (value.as_str(), "beats the environment"),
        (Some(value), None) => (value.as_str(), ""),
        (None, Some(value)) => (value.as_str(), "environment"),
        (None, None) => ("", "not set on this computer"),
    };
    let label = format!("${} {value} {source}", variable.name);
    let (rect, response) = ui.allocate_exact_size(vec2(width, t.metrics.row_h), Sense::click());
    response.widget_info(|| {
        egui::WidgetInfo::selected(
            egui::WidgetType::SelectableLabel,
            ui.is_enabled(),
            selected,
            label.trim(),
        )
    });
    if ui.is_rect_visible(rect) {
        let fill = if selected {
            t.color.accent_dim
        } else if response.hovered() {
            t.color.bg_hover
        } else {
            t.color.bg_inset
        };
        ui.painter().rect_filled(rect, 0.0, fill);
        let inset = tokens::SP_4;
        let mut right = rect.right() - inset;
        if !source.is_empty() {
            let width = ui
                .painter()
                .text(
                    egui::pos2(right, rect.center().y),
                    egui::Align2::RIGHT_CENTER,
                    source,
                    theme::sans(tokens::FS_0, FontWeight::Regular),
                    t.color.text_dim,
                )
                .width();
            right -= width;
            if variable.preferences.is_none() && variable.environment.is_none() {
                paint_problem_mark(
                    ui.painter(),
                    egui::Rect::from_center_size(
                        egui::pos2(
                            right - PROBLEM_MARK_ADVANCE + PROBLEM_MARK_SIDE * 0.5,
                            rect.center().y,
                        ),
                        egui::Vec2::splat(PROBLEM_MARK_SIDE),
                    ),
                    Problem::Failure,
                );
                right -= PROBLEM_MARK_ADVANCE;
            }
            right -= inset;
        }
        let name = format!("${}", variable.name);
        let name_width = ui
            .painter()
            .text(
                egui::pos2(rect.left() + inset, rect.center().y),
                egui::Align2::LEFT_CENTER,
                &name,
                theme::mono(tokens::FS_0, FontWeight::Medium),
                t.color.text,
            )
            .width();
        let left = rect.left() + inset + name_width + tokens::SP_4;
        let shown = ui.fonts_mut(|fonts| {
            let mut job = egui::text::LayoutJob::simple_singleline(
                value.to_owned(),
                theme::mono(tokens::FS_0, FontWeight::Regular),
                egui::Color32::PLACEHOLDER,
            );
            job.wrap = egui::text::TextWrapping::truncate_at_width((right - left).max(0.0));
            fonts.layout_job(job)
        });
        ui.painter().galley(
            egui::pos2(left, rect.center().y - shown.size().y * 0.5),
            shown,
            t.color.text,
        );
    }
    theme::paint_focus_ring(ui, &response, rect);
    let hover = match (&variable.preferences, &variable.environment) {
        (Some(_), Some(environment)) => format!("The environment says {environment}"),
        _ => value.to_owned(),
    };
    if hover.is_empty() {
        response.clicked()
    } else {
        response.on_hover_text(hover).clicked()
    }
}

/// A line of dim words under the row's title.
fn note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_dim),
    );
}

#[cfg(test)]
mod tests {
    use super::path_variable_refusal;

    #[test]
    fn a_root_is_a_named_absolute_folder_that_names_no_other_root() {
        let absolute = if cfg!(windows) {
            "C:/libraries"
        } else {
            "/libraries"
        };
        assert_eq!(path_variable_refusal("RSPICE_LIBS", absolute), None);
        assert!(path_variable_refusal("", absolute).is_some());
        assert!(path_variable_refusal("9LIBS", absolute).is_some());
        assert!(path_variable_refusal("MY-LIBS", absolute).is_some());
        assert!(path_variable_refusal("LIBS", "").is_some());
        assert!(path_variable_refusal("LIBS", "libraries/analog").is_some());
        assert!(path_variable_refusal("LIBS", "$OTHER/analog").is_some());
    }
}

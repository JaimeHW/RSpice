//! Signal sets: named sets of signals the reader saves with the project and
//! puts back on the plot in one step, from the browser's Sets menu.

use egui::Ui;

use crate::ui::accessibility::{counted, plural_suffix};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::workbench::documents::result_document::{AnalysisPresentationKey, ResultsParts};
use rspice_results::result_presentation::{
    ResultArtifactPresentationKey, SignalSet, SignalSetMember, SourceWaveformPresentationKey,
};
use rspice_results_ui::selection::ResultBrowserSelectionKey;

use crate::workbench::documents::result_document::signal_sets::{
    MAX_SIGNAL_SET_MEMBERS, MAX_SIGNAL_SETS, signal_set_index, signal_set_name,
};

use super::super::super::design_system::WorkbenchIcon;
use super::results_actions::set_result_quantity_ticked;
use super::results_rows::retained_result_artifacts;

/// The member a browser row names: the kind of analysis that retained it and
/// the name it is listed under.
fn member_of(state: &ResultsParts<'_>, key: &ResultBrowserSelectionKey) -> Option<SignalSetMember> {
    let runs = &state.model.simulation.retained.runs;
    match key {
        ResultBrowserSelectionKey::Waveform(key) => {
            let (run, analysis, _, waveform) = key.resolve(runs)?;
            Some(SignalSetMember {
                analysis: runs[run].analyses[analysis].analysis_type,
                name: waveform.name.clone(),
                artifact: false,
            })
        }
        ResultBrowserSelectionKey::Artifact(key) => {
            let (_, _, analysis) = key.resolve(runs)?;
            Some(SignalSetMember {
                analysis: analysis.analysis_type,
                name: key.canonical_name().to_owned(),
                artifact: true,
            })
        }
    }
}

/// Save `keys` as the set `name`. Saving under a name already in use
/// replaces that set's signals, as saving a file over one does.
pub(super) fn save_signal_set(
    state: &mut ResultsParts<'_>,
    name: &str,
    keys: &[ResultBrowserSelectionKey],
) -> Result<String, String> {
    let name = signal_set_name(name)?;
    let mut members = Vec::new();
    for member in keys.iter().filter_map(|key| member_of(state, key)) {
        if !members.contains(&member) {
            members.push(member);
        }
    }
    if members.is_empty() {
        return Err("Tick the signals the set should hold first.".to_owned());
    }
    members.truncate(MAX_SIGNAL_SET_MEMBERS);
    let count = counted(members.len(), "signal", "signals");
    let sets = &mut state.results.signal_sets;
    match signal_set_index(sets, &name) {
        Some(index) => {
            sets[index].members = members;
            Ok(format!("Saved {count} over the set {name}."))
        }
        None if sets.len() >= MAX_SIGNAL_SETS => Err(format!(
            "A project keeps at most {MAX_SIGNAL_SETS} signal sets. Delete one first."
        )),
        None => {
            sets.push(SignalSet {
                name: name.clone(),
                members,
            });
            Ok(format!("Saved {count} as the set {name}."))
        }
    }
}

/// Add `keys` to the set at `index`, keeping the signals it already holds.
pub(super) fn add_to_signal_set(
    state: &mut ResultsParts<'_>,
    index: usize,
    keys: &[ResultBrowserSelectionKey],
) -> Result<String, String> {
    let added = keys
        .iter()
        .filter_map(|key| member_of(state, key))
        .collect::<Vec<_>>();
    let set = state
        .results
        .signal_sets
        .get_mut(index)
        .ok_or_else(|| "That set no longer exists.".to_owned())?;
    let before = set.members.len();
    for member in added {
        if set.members.len() < MAX_SIGNAL_SET_MEMBERS && !set.members.contains(&member) {
            set.members.push(member);
        }
    }
    Ok(format!(
        "Added {} signals to the set {}.",
        set.members.len() - before,
        set.name
    ))
}

pub(super) fn rename_signal_set(
    state: &mut ResultsParts<'_>,
    index: usize,
    name: &str,
) -> Result<(), String> {
    let name = signal_set_name(name)?;
    let sets = &mut state.results.signal_sets;
    if signal_set_index(sets, &name).is_some_and(|existing| existing != index) {
        return Err(format!("A set named {name} already exists."));
    }
    let set = sets
        .get_mut(index)
        .ok_or_else(|| "That set no longer exists.".to_owned())?;
    set.name = name;
    Ok(())
}

pub(super) fn delete_signal_set(state: &mut ResultsParts<'_>, index: usize) {
    if index < state.results.signal_sets.len() {
        state.results.signal_sets.remove(index);
    }
}

/// Put the set at `index` on the plot of the active run, as its plot.
///
/// In every analysis of the active run whose kind the set names, the set's
/// signals go on and every other signal comes off; analyses the set does not
/// name are left as they are. A member the run did not retain is counted and
/// named, never matched by a near name.
pub(super) fn apply_signal_set(
    state: &mut ResultsParts<'_>,
    index: usize,
) -> Result<String, String> {
    let set = state
        .results
        .signal_sets
        .get(index)
        .cloned()
        .ok_or_else(|| "That set no longer exists.".to_owned())?;
    let simulation = &state.model.simulation;
    let run_index = simulation
        .view
        .active_run_idx
        .or_else(|| simulation.retained.runs.len().checked_sub(1))
        .ok_or_else(|| "There is no run to apply the set to.".to_owned())?;
    let run = &simulation.retained.runs[run_index];
    let mut plot = Vec::new();
    let mut found = std::collections::HashSet::new();
    for analysis in &run.analyses {
        if !set
            .members
            .iter()
            .any(|member| member.analysis == analysis.analysis_type)
        {
            continue;
        }
        let analysis_key = AnalysisPresentationKey::new(run.dataset_id, analysis);
        for waveform in &analysis.waveforms {
            let member = SignalSetMember {
                analysis: analysis.analysis_type,
                name: waveform.name.clone(),
                artifact: false,
            };
            let wanted = set.members.contains(&member);
            if wanted {
                found.insert(member);
            }
            plot.push((
                ResultBrowserSelectionKey::Waveform(SourceWaveformPresentationKey::new(
                    analysis_key,
                    waveform.name.clone(),
                )),
                wanted,
            ));
        }
        // Event nets and buses are the artifacts a set can hold; the
        // browser plots them as lanes.
        for artifact in retained_result_artifacts(analysis, analysis_key) {
            let canonical = artifact.identity.canonical_name().to_owned();
            if !super::results_rows::is_plottable_artifact(&canonical) {
                continue;
            }
            let member = SignalSetMember {
                analysis: analysis.analysis_type,
                name: canonical.clone(),
                artifact: true,
            };
            let wanted = set.members.contains(&member);
            if wanted {
                found.insert(member);
            }
            plot.push((
                ResultBrowserSelectionKey::Artifact(ResultArtifactPresentationKey::new(
                    analysis_key,
                    &canonical,
                )),
                wanted,
            ));
        }
    }
    // Off first, then on, so a net a bus shares comes out plotted.
    for (key, _) in plot.iter().filter(|(_, wanted)| !wanted) {
        set_result_quantity_ticked(state, key, false);
    }
    for (key, _) in plot.iter().filter(|(_, wanted)| *wanted) {
        set_result_quantity_ticked(state, key, true);
    }
    let plotted = counted(found.len(), "signal", "signals");
    let missing = set
        .members
        .iter()
        .filter(|member| !found.contains(*member))
        .map(|member| member_label(&member.name))
        .collect::<Vec<_>>();
    Ok(match missing.as_slice() {
        [] => format!("Plotted the set {}: {plotted}.", set.name),
        [one] => format!(
            "Plotted the set {}: {plotted}; this run did not retain {one}.",
            set.name
        ),
        [first, rest @ ..] => format!(
            "Plotted the set {}: {plotted}; this run did not retain {first} and {} more.",
            set.name,
            rest.len()
        ),
    })
}

/// The name a member is read under: an event net's or a bus's own name
/// rather than the canonical row name it is stored by.
fn member_label(name: &str) -> &str {
    name.rsplit('/').next().unwrap_or(name)
}

fn draft_id(salt: impl std::hash::Hash + std::fmt::Debug) -> egui::Id {
    egui::Id::new(("workbench.results.signal-set-draft", salt))
}

fn take_draft(ui: &Ui, id: egui::Id, initial: &str) -> String {
    ui.ctx()
        .data(|data| data.get_temp::<String>(id))
        .unwrap_or_else(|| initial.to_owned())
}

fn store_draft(ui: &Ui, id: egui::Id, draft: String) {
    ui.ctx().data_mut(|data| data.insert_temp(id, draft));
}

fn forget_draft(ui: &Ui, id: egui::Id) {
    ui.ctx().data_mut(|data| data.remove::<String>(id));
}

/// The Sets menu on the browser's scope band: save the ticked signals under
/// a name, and apply, extend, rename or delete each saved set.
pub(super) fn signal_sets_menu(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    ticked: &[ResultBrowserSelectionKey],
) {
    let count = state.results.signal_sets.len();
    let label = if count == 0 {
        "Sets".to_owned()
    } else {
        format!("Sets {count}")
    };
    // A click into a name field must not close the menu it sits in, so the
    // menu closes on a click outside it and each action closes it itself.
    let config = egui::containers::menu::MenuConfig::new()
        .close_behavior(egui::PopupCloseBehavior::CloseOnClickOutside);
    let (response, _) = egui::containers::menu::MenuButton::new(label)
        .config(config)
        .ui(ui, |ui| signal_sets_menu_body(ui, state, ticked));
    response.on_hover_text("Named sets of signals, saved with the project");
}

fn signal_sets_menu_body(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    ticked: &[ResultBrowserSelectionKey],
) {
    let t = Tokens::get(ui.ctx());
    ui.set_min_width(220.0);
    let new_id = draft_id("new");
    let mut draft = take_draft(ui, new_id, "");
    let mut save = false;
    ui.horizontal(|ui| {
        let field = ui.add(
            egui::TextEdit::singleline(&mut draft)
                .hint_text("New set name")
                .font(theme::field_sans())
                .desired_width(150.0),
        );
        save |= field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
        let can_save = !ticked.is_empty() && !draft.trim().is_empty();
        let button = ui.add_enabled(can_save, egui::Button::new("Save"));
        let button = if ticked.is_empty() {
            button.on_disabled_hover_text("Tick the signals the set should hold first")
        } else {
            button.on_hover_text(format!(
                "Save the {} ticked signal{}",
                ticked.len(),
                plural_suffix(ticked.len())
            ))
        };
        save |= button.clicked();
    });
    if save {
        match save_signal_set(state, &draft, ticked) {
            Ok(message) => {
                state.workbench.prompt(message);
                forget_draft(ui, new_id);
                ui.close();
                return;
            }
            Err(error) => state.workbench.prompt(error),
        }
    }
    store_draft(ui, new_id, draft);
    ui.separator();
    if state.results.signal_sets.is_empty() {
        ui.label(
            egui::RichText::new("No saved sets. Tick signals, name them, and save.")
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(t.color.text_dim),
        );
        return;
    }
    let sets = state
        .results
        .signal_sets
        .iter()
        .map(|set| (set.name.clone(), set.members.len()))
        .collect::<Vec<_>>();
    for (index, (name, members)) in sets.into_iter().enumerate() {
        submenu(ui, format!("{name} · {members}"), |ui| {
            signal_set_submenu(ui, state, ticked, index, &name);
        });
    }
}

fn signal_set_submenu(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    ticked: &[ResultBrowserSelectionKey],
    index: usize,
    name: &str,
) {
    if ui
        .button("Apply")
        .on_hover_text("Plot exactly this set in the active run")
        .clicked()
    {
        let message = apply_signal_set(state, index).unwrap_or_else(|error| error);
        state.workbench.prompt(message);
        ui.close();
        return;
    }
    let add = ui.add_enabled(
        !ticked.is_empty(),
        egui::Button::new(format!("Add the {} ticked", ticked.len())),
    );
    if add.clicked() {
        let message = add_to_signal_set(state, index, ticked).unwrap_or_else(|error| error);
        state.workbench.prompt(message);
        ui.close();
        return;
    }
    ui.separator();
    let rename_id = draft_id(("rename", index));
    let mut draft = take_draft(ui, rename_id, name);
    let field = ui.add(
        egui::TextEdit::singleline(&mut draft)
            .font(theme::field_sans())
            .desired_width(160.0),
    );
    let field = field.on_hover_text("Rename: type a name and press Enter");
    if field.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter)) {
        match rename_signal_set(state, index, &draft) {
            Ok(()) => {
                forget_draft(ui, rename_id);
                ui.close();
                return;
            }
            Err(error) => state.workbench.prompt(error),
        }
    }
    store_draft(ui, rename_id, draft);
    ui.separator();
    if ui.button("Delete").clicked() {
        delete_signal_set(state, index);
        forget_draft(ui, rename_id);
        state.workbench.prompt(format!("Deleted the set {name}."));
        ui.close();
    }
}

/// A submenu row with the workbench's own chevron: the bundled faces have no
/// glyph for egui's arrow, which would draw as a missing-glyph box.
fn submenu(ui: &mut Ui, label: String, content: impl FnOnce(&mut Ui)) {
    const CHEVRON_SIDE: f32 = 12.0;
    let button = egui::Button::new(label).right_text("   ");
    let (response, _) = egui::containers::menu::SubMenuButton::from_button(button).ui(ui, content);
    let rect = response.rect;
    WorkbenchIcon::ChevronRight.paint(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(
                rect.right() - ui.spacing().button_padding.x - CHEVRON_SIDE / 2.0,
                rect.center().y,
            ),
            egui::Vec2::splat(CHEVRON_SIDE),
        ),
        Tokens::get(ui.ctx()).color.text_dim,
    );
}

/// "Add to set" on a row's menu: every saved set, and a new one named after
/// the row.
pub(super) fn add_to_set_menu(
    ui: &mut Ui,
    state: &mut ResultsParts<'_>,
    key: &ResultBrowserSelectionKey,
) {
    submenu(ui, "Add to set".to_owned(), |ui| {
        let sets = state
            .results
            .signal_sets
            .iter()
            .map(|set| set.name.clone())
            .collect::<Vec<_>>();
        for (index, name) in sets.iter().enumerate() {
            if ui.button(name).clicked() {
                let message = add_to_signal_set(state, index, std::slice::from_ref(key))
                    .unwrap_or_else(|error| error);
                state.workbench.prompt(message);
                ui.close();
            }
        }
        if !sets.is_empty() {
            ui.separator();
        }
        if ui.button("New set").clicked() {
            let base = member_of(state, key).map_or_else(
                || "Signals".to_owned(),
                |member| member_label(&member.name).to_owned(),
            );
            let mut name = base.clone();
            let mut suffix = 2;
            while signal_set_index(&state.results.signal_sets, &name).is_some() {
                name = format!("{base} {suffix}");
                suffix += 1;
            }
            let message = save_signal_set(state, &name, std::slice::from_ref(key))
                .unwrap_or_else(|error| error);
            state.workbench.prompt(message);
            ui.close();
        }
    });
}

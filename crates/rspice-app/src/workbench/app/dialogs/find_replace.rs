//! Find and replace on instance names, net-label names and property values.
//!
//! The form asks what to find, what to put in its place, where to look and
//! how to read the find text. While it is open the Navigator lists every
//! match with the text it would become, so the reader sees the whole change
//! before Replace all makes it. Replace all is one undo step in each cell
//! view it changes, and nothing changes anywhere unless every new name in
//! every one of them is legal.

use std::collections::BTreeSet;

use egui::{Context, Id};
use rspice_design::schematic::document::SchematicDocument;
use rspice_design::schematic::find_replace::{
    FindMatch, FindOptions, FindQuery, FindTargets, find_in_document,
};

use crate::schematic::view::sheet_visibility::object_is_on_active_sheet;
use crate::state::CellViewRef;
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;

const TITLE: &str = "Find and replace";
const PRIMARY: &str = "Replace all";
const DESCRIPTION: &str = "Replace text in instance names, net labels and property values.";
const FIND_FIELD: &str = "find-replace-find";
const REPLACE_FIELD: &str = "find-replace-replace";
const SCOPES: [&str; 3] = ["Sheet", "Cell view", "Hierarchy below"];

/// Where Replace all reaches.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FindScope {
    /// The sheet on the canvas.
    Sheet,
    /// Every sheet of the cell view on the canvas.
    CellView,
    /// The cell view on the canvas and every editable cell view placed
    /// below it.
    Hierarchy,
}

/// The form's draft. It outlives one opening, as a find box does.
#[derive(Debug, Clone)]
pub(crate) struct FindReplaceDialogState {
    pub(crate) open: bool,
    pub(crate) find: String,
    pub(crate) replace: String,
    /// An index into the scopes, as the segmented control keeps it.
    pub(crate) scope: usize,
    pub(crate) targets: FindTargets,
    pub(crate) options: FindOptions,
}

impl Default for FindReplaceDialogState {
    fn default() -> Self {
        Self {
            open: false,
            find: String::new(),
            replace: String::new(),
            scope: 1,
            targets: FindTargets::default(),
            options: FindOptions::default(),
        }
    }
}

impl FindReplaceDialogState {
    pub(crate) fn scope(&self) -> FindScope {
        match self.scope {
            0 => FindScope::Sheet,
            2 => FindScope::Hierarchy,
            _ => FindScope::CellView,
        }
    }
}

/// The matches in one cell view.
#[derive(Debug, Clone)]
pub(crate) struct DocumentMatches {
    pub(crate) reference: CellViewRef,
    pub(crate) key: String,
    pub(crate) path: String,
    pub(crate) matches: Vec<FindMatch>,
}

pub(crate) fn open_find_replace(state: &mut AppState) {
    if state.dialogs.application_modal_open() {
        return;
    }
    state.dialogs.find_replace.open = true;
    // The matches are listed in the Navigator, so it comes forward.
    state.workbench.navigator_visible = true;
    state.workbench.design_panel = crate::workbench::state::DesignPanel::Navigator;
}

/// Every text the form's query changes, cell view by cell view, the active
/// one first; or why the query cannot run.
pub(crate) fn find_replace_matches(state: &AppState) -> Result<Vec<DocumentMatches>, String> {
    let form = &state.dialogs.find_replace;
    let query = FindQuery::new(&form.find, &form.replace, form.options, form.targets)?;
    let active = state.workspace.content.active_schematic_reference();
    let scope = form.scope();
    let mut found = Vec::new();
    for reference in documents_in_scope(state, scope) {
        let Some(document) = document_for(state, &reference) else {
            continue;
        };
        let on_canvas = reference == active;
        let matches = find_in_document(document, &query, |id| {
            !(on_canvas && scope == FindScope::Sheet) || object_is_on_active_sheet(state, id)
        });
        if !matches.is_empty() {
            found.push(DocumentMatches {
                reference: reference.clone(),
                key: reference.key(),
                path: reference.display_path(),
                matches,
            });
        }
    }
    Ok(found)
}

/// The cell views `scope` reaches: the one on the canvas, then, for the
/// hierarchy, every editable schematic placed below it, each once.
fn documents_in_scope(state: &AppState, scope: FindScope) -> Vec<CellViewRef> {
    let active = state.workspace.content.active_schematic_reference();
    let mut order = vec![active.clone()];
    if scope != FindScope::Hierarchy {
        return order;
    }
    let mut seen = BTreeSet::from([active.key()]);
    let mut next = 0;
    while let Some(reference) = order.get(next).cloned() {
        next += 1;
        let Some(document) = document_for(state, &reference) else {
            continue;
        };
        for binding in document
            .components
            .iter()
            .filter_map(|component| component.library_cell.as_ref())
        {
            let child = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
            if state
                .workspace
                .content
                .schematic_buffers
                .contains_key(&child.key())
                && !read_only(state, &child)
                && seen.insert(child.key())
            {
                order.push(child);
            }
        }
    }
    order
}

fn document_for<'a>(state: &'a AppState, reference: &CellViewRef) -> Option<&'a SchematicDocument> {
    if *reference == state.workspace.content.active_schematic_reference() {
        Some(state.schematic.document())
    } else {
        state
            .workspace
            .content
            .schematic_buffers
            .get(&reference.key())
            .map(|buffer| buffer.document())
    }
}

fn read_only(state: &AppState, reference: &CellViewRef) -> bool {
    state.workbench.safe_mode.project_read_only()
        || state
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| library.read_only)
}

/// Make every listed change, one undo step per cell view. Every cell view
/// is checked before any changes. Returns the number of texts changed and
/// of cell views they were in.
pub(crate) fn replace_all(state: &mut AppState) -> Result<(usize, usize), String> {
    if state.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    if state.schematic_edit_in_progress() {
        return Err("Finish or cancel the edit in progress first.".to_owned());
    }
    let found = find_replace_matches(state)?;
    if found.is_empty() {
        return Err("Nothing matches.".to_owned());
    }
    for document in &found {
        let source = document_for(state, &document.reference)
            .ok_or_else(|| format!("{} is no longer open.", document.path))?;
        if let Some(refusal) =
            rspice_design::schematic::find_replace::refusal(source, &document.matches)
        {
            return Err(if found.len() > 1 {
                format!("{}: {refusal}", document.path)
            } else {
                refusal
            });
        }
    }
    let active = state.workspace.content.active_schematic_reference().key();
    let mut changed = 0;
    for document in &found {
        changed += if document.key == active {
            state.schematic.replace_text(&document.matches)?
        } else if let Some(mut buffer) = state.workspace.schematic_editor_mut(&document.key) {
            buffer.editor.replace_text(&document.matches)?
        } else {
            0
        };
    }
    state.sync_active_schematic_to_workspace();
    Ok((changed, found.len()))
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_find_replace_dialog(&mut self, ctx: &Context) {
        if !self.state.dialogs.find_replace.open {
            return;
        }
        let found = find_replace_matches(&self.state);
        let count: usize = found.as_ref().map_or(0, |found| {
            found.iter().map(|document| document.matches.len()).sum()
        });
        let summary = match &found {
            Ok(found) if count > 0 => Ok(Some(match found.len() {
                1 => format!("{count} {}.", plural(count, "match", "matches")),
                documents => format!("{count} matches in {documents} cell views."),
            })),
            Ok(_) if self.state.dialogs.find_replace.find.is_empty() => Ok(None),
            Ok(_) => Ok(Some("Nothing matches.".to_owned())),
            Err(reason) => Err(reason.clone()),
        };
        let status = match &summary {
            Ok(Some(text)) => Ok(Some(text.as_str())),
            Ok(None) => Ok(None),
            Err(reason) => Err(reason.as_str()),
        };
        let view_path = self.state.workspace.content.active_view.display_path();
        let form = &mut self.state.dialogs.find_replace;
        let choice = CommandForm::new(TITLE, PRIMARY)
            .context(Some(view_path.as_str()))
            .describe(DESCRIPTION)
            .primary_enabled(count > 0)
            .status(status)
            .status_lines(2)
            .show(ctx, |rows| {
                let find = Id::new(FIND_FIELD);
                rows.text("Find", find, &mut form.find, "text or pattern");
                rows.text("Replace", Id::new(REPLACE_FIELD), &mut form.replace, "");
                rows.checks(
                    "Look in",
                    &mut [
                        ("Instance names", &mut form.targets.instance_names),
                        ("Net labels", &mut form.targets.net_labels),
                        ("Values", &mut form.targets.values),
                    ],
                );
                rows.segmented("Scope", "find-replace-scope", &SCOPES, &mut form.scope);
                rows.checks(
                    "Match",
                    &mut [
                        ("Whole word", &mut form.options.whole_word),
                        ("Case", &mut form.options.match_case),
                        ("Regular expression", &mut form.options.regex),
                    ],
                );
                Some(find)
            });
        match choice {
            DialogChoice::Primary => match replace_all(&mut self.state) {
                Ok((texts, documents)) => {
                    self.state.dialogs.find_replace.open = false;
                    self.state.prompt(match documents {
                        1 => format!("Replaced {texts} {}.", plural(texts, "match", "matches")),
                        _ => format!("Replaced {texts} matches in {documents} cell views."),
                    });
                }
                Err(reason) => self.state.prompt(reason),
            },
            DialogChoice::Ghost | DialogChoice::Cancelled => {
                self.state.dialogs.find_replace.open = false;
            }
            DialogChoice::None | DialogChoice::Secondary => {}
        }
    }
}

fn plural<'a>(count: usize, one: &'a str, many: &'a str) -> &'a str {
    if count == 1 { one } else { many }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{
        Component, ComponentType, LibraryCellInstance, NetLabel, Point, SchematicState,
    };

    fn part(id: u64, name: &str, value: &str) -> Component {
        let mut component =
            Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        component.name = name.to_owned();
        component.value = value.to_owned();
        component
    }

    /// The form in each state worth reading, beside the Navigator rows it
    /// lists, at desktop and phone widths, in both themes.
    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    #[ignore = "writes PNGs for a human to look at; run with --ignored"]
    fn render_find_replace() {
        use crate::ui::tokens::Mode;
        let directory = std::env::var("RSPICE_RASTER_DIR")
            .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
        std::fs::create_dir_all(&directory).expect("raster output directory");
        for (stem, find, regex) in [
            ("empty", "", false),
            ("matches", "10k", false),
            ("refusal", "R(", true),
        ] {
            for (size_stem, size) in [
                ("1024x640", egui::vec2(1024.0, 640.0)),
                ("390x844", egui::vec2(390.0, 844.0)),
            ] {
                for (mode_stem, mode) in [("dark", Mode::Dark), ("light", Mode::Light)] {
                    let mut app = RSpiceApp::test_instance();
                    let document = app.state.schematic.document_mut_for_test();
                    document.components = vec![part(1, "R1", "10k"), part(2, "R2", "10k")];
                    open_find_replace(&mut app.state);
                    app.state.dialogs.find_replace.options.regex = regex;
                    ask(&mut app.state, find, "22k", FindScope::CellView);
                    let canvas = crate::ui::raster::render_themed(
                        crate::ui::Theme {
                            mode,
                            ..crate::ui::Theme::default()
                        },
                        size,
                        |ui, _background| app.render_find_replace_dialog(ui.ctx()),
                    );
                    let height = canvas.content_height().max(1);
                    let path =
                        directory.join(format!("find-replace-{stem}-{size_stem}-{mode_stem}.png"));
                    std::fs::write(&path, canvas.png(height)).expect("write dialog render");
                }
            }
        }
    }

    fn ask(state: &mut AppState, find: &str, replace: &str, scope: FindScope) {
        let form = &mut state.dialogs.find_replace;
        form.find = find.to_owned();
        form.replace = replace.to_owned();
        form.scope = match scope {
            FindScope::Sheet => 0,
            FindScope::CellView => 1,
            FindScope::Hierarchy => 2,
        };
    }

    fn values(state: &AppState) -> Vec<String> {
        state
            .schematic
            .document()
            .components
            .iter()
            .map(|component| component.value.clone())
            .collect()
    }

    /// Replace all changes every match in the cell view as one undo step
    /// (audit G-7).
    #[test]
    fn replace_all_in_the_cell_view_is_one_undo_step() {
        let mut state = AppState::default();
        let document = state.schematic.document_mut_for_test();
        document.components = vec![part(1, "R1", "10k"), part(2, "R2", "10k")];
        document.wires.clear();
        state.schematic.init_undo_history();
        ask(&mut state, "10k", "22k", FindScope::CellView);

        assert_eq!(
            find_replace_matches(&state)
                .expect("query")
                .iter()
                .map(|document| document.matches.len())
                .sum::<usize>(),
            2
        );
        assert_eq!(replace_all(&mut state), Ok((2, 1)));
        assert_eq!(values(&state), ["22k", "22k"]);
        assert!(state.schematic.undo());
        assert_eq!(values(&state), ["10k", "10k"]);
        assert!(!state.schematic.can_undo());
    }

    /// The Sheet scope leaves another sheet of the same cell view alone.
    #[test]
    fn the_sheet_scope_leaves_the_other_sheets_alone() {
        let mut state = crate::schematic::view::sheet_visibility::two_sheet_state(&[1], &[2]);
        let document = state.schematic.document_mut_for_test();
        document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
        document.wires.clear();
        state.schematic.init_undo_history();
        ask(&mut state, "1k", "2k", FindScope::Sheet);
        assert_eq!(replace_all(&mut state), Ok((1, 1)));
        assert_eq!(values(&state), ["2k", "1k"]);
        ask(&mut state, "1k", "2k", FindScope::CellView);
        assert_eq!(replace_all(&mut state), Ok((1, 1)));
        assert_eq!(values(&state), ["2k", "2k"]);
    }

    /// A replacement that would give two instances one name changes nothing.
    #[test]
    fn a_clash_anywhere_refuses_the_whole_replacement() {
        let mut state = AppState::default();
        let document = state.schematic.document_mut_for_test();
        document.components = vec![part(1, "R1", "1k"), part(2, "R2", "1k")];
        state.schematic.init_undo_history();
        let options = &mut state.dialogs.find_replace.options;
        options.regex = true;
        ask(&mut state, r"R\d", "RA", FindScope::CellView);
        assert_eq!(
            replace_all(&mut state),
            Err("Two instances would both be named RA.".to_owned())
        );
        assert_eq!(state.schematic.document().components[0].name, "R1");
        assert!(!state.schematic.can_undo());
    }

    /// Hierarchy below reaches the master an instance places, and the
    /// master's change is its own undo step there.
    #[test]
    fn the_hierarchy_scope_reaches_the_cell_views_placed_below() {
        let mut state = AppState::default();
        let top = state.workspace.content.active_schematic_reference();
        let child = CellViewRef::new(&top.library, "amp", "schematic");
        let mut master = SchematicState::default();
        master.document_mut_for_test().net_labels = vec![NetLabel::new(5, Point::origin(), "vin")];
        master.init_undo_history();
        state.workspace.insert_schematic_editor(child.key(), master);
        state.schematic.document_mut_for_test().components.clear();
        state.schematic.document_mut_for_test().wires.clear();
        state.schematic.add_library_cell_component(
            Point::origin(),
            LibraryCellInstance::new(&top.library, "amp", "schematic"),
        );
        state.schematic.document_mut_for_test().net_labels =
            vec![NetLabel::new(9, Point::new(0, 50), "vin")];
        state.schematic.init_undo_history();

        ask(&mut state, "vin", "vin_a", FindScope::CellView);
        assert_eq!(find_replace_matches(&state).expect("query").len(), 1);
        ask(&mut state, "vin", "vin_a", FindScope::Hierarchy);
        let found = find_replace_matches(&state).expect("query");
        assert_eq!(
            found
                .iter()
                .map(|document| document.path.clone())
                .collect::<Vec<_>>(),
            [top.display_path(), child.display_path()]
        );
        assert_eq!(replace_all(&mut state), Ok((2, 2)));

        let mut stored = state
            .workspace
            .schematic_editor_mut(&child.key())
            .expect("child");
        assert_eq!(stored.editor.document().net_labels[0].name, "vin_a");
        assert!(stored.editor.undo(), "the master's change is its own step");
        assert_eq!(stored.editor.document().net_labels[0].name, "vin");
        drop(stored);
        assert_eq!(state.schematic.document().net_labels[0].name, "vin_a");
    }
}

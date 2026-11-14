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
    FindField, FindMatch, FindOptions, FindQuery, FindTargets, find_in_document,
};

use crate::schematic::view::sheet_visibility::object_is_on_active_sheet;
use crate::state::CellViewRef;
use crate::ui::widgets::{CommandForm, DialogChoice};
use crate::workbench::app::actions::workspace::sync_active_schematic;
use crate::workbench::app_state::Model;
use crate::workbench::state::WorkbenchSession;

const TITLE: &str = "Find and replace";
const PRIMARY: &str = "Replace all";
const DESCRIPTION: &str = "Replace text in instance names, net labels and property values.";
const FIND_FIELD: &str = "find-replace-find";
const REPLACE_FIELD: &str = "find-replace-replace";
pub(crate) const SCOPES: [&str; 3] = ["Sheet", "Cell view", "Hierarchy below"];

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

/// Every text the form's query changes, cell view by cell view, the active
/// one first; or why the query cannot run.
pub(crate) fn find_replace_matches(
    form: &FindReplaceDialogState,
    model: &Model,
) -> Result<Vec<DocumentMatches>, String> {
    let query = FindQuery::new(&form.find, &form.replace, form.options, form.targets)?;
    let active = model.workspace.content.active_schematic_reference();
    let scope = form.scope();
    let mut found = Vec::new();
    for reference in documents_in_scope(model, scope) {
        let Some(document) = document_for(model, &reference) else {
            continue;
        };
        let on_canvas = reference == active;
        let matches = find_in_document(document, &query, |id| {
            !(on_canvas && scope == FindScope::Sheet) || object_is_on_active_sheet(model, id)
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
fn documents_in_scope(model: &Model, scope: FindScope) -> Vec<CellViewRef> {
    let active = model.workspace.content.active_schematic_reference();
    let mut order = vec![active.clone()];
    if scope != FindScope::Hierarchy {
        return order;
    }
    let mut seen = BTreeSet::from([active.key()]);
    let mut next = 0;
    while let Some(reference) = order.get(next).cloned() {
        next += 1;
        let Some(document) = document_for(model, &reference) else {
            continue;
        };
        for binding in document
            .components
            .iter()
            .filter_map(|component| component.library_cell.as_ref())
        {
            let child = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
            if model
                .workspace
                .content
                .schematic_buffers
                .contains_key(&child.key())
                && !read_only(model, &child)
                && seen.insert(child.key())
            {
                order.push(child);
            }
        }
    }
    order
}

fn document_for<'a>(model: &'a Model, reference: &CellViewRef) -> Option<&'a SchematicDocument> {
    if *reference == model.workspace.content.active_schematic_reference() {
        Some(model.schematic.document())
    } else {
        model
            .workspace
            .content
            .schematic_buffers
            .get(&reference.key())
            .map(|buffer| buffer.document())
    }
}

fn read_only(model: &Model, reference: &CellViewRef) -> bool {
    model.safe_mode.project_read_only()
        || model
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| library.read_only)
}

/// Make every listed change, one undo step per cell view. Every cell view
/// is checked before any changes. Returns the number of texts changed and
/// of cell views they were in.
pub(crate) fn replace_all(
    form: &FindReplaceDialogState,
    edit_in_progress: bool,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<(usize, usize), String> {
    if model.schematic_edit_read_only() {
        return Err("The active schematic is read-only.".to_owned());
    }
    if edit_in_progress {
        return Err("Finish or cancel the edit in progress first.".to_owned());
    }
    let found = find_replace_matches(form, model)?;
    if found.is_empty() {
        return Err("Nothing matches.".to_owned());
    }
    for document in &found {
        let source = document_for(model, &document.reference)
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
    let active = model.workspace.content.active_schematic_reference().key();
    let mut changed = 0;
    for document in &found {
        let renames = document
            .matches
            .iter()
            .any(|found| found.field == FindField::InstanceName);
        changed += if renames {
            // An instance renamed here is renamed as the Rename form
            // renames it, references and all.
            model.replace_text_carrying_references(&document.reference, &document.matches)?
        } else if document.key == active {
            model.schematic.replace_text(&document.matches)?
        } else if let Some(mut buffer) = model.workspace.schematic_editor_mut(&document.key) {
            buffer.editor.replace_text(&document.matches)?
        } else {
            0
        };
    }
    sync_active_schematic(model, workbench);
    Ok((changed, found.len()))
}

/// Draw Find and replace over its form and the model it searches.
/// `edit_in_progress` is whether a schematic edit is under way, which Replace
/// all waits for.
pub(in crate::workbench) fn render_find_replace_dialog(
    dialog: &mut FindReplaceDialogState,
    edit_in_progress: bool,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
    ctx: &Context,
) {
    if !dialog.open {
        return;
    }
    let found = find_replace_matches(dialog, model);
    let count: usize = found.as_ref().map_or(0, |found| {
        found.iter().map(|document| document.matches.len()).sum()
    });
    let summary = find_replace_summary(&dialog.find, &found, count);
    let status = match &summary {
        Ok(Some(text)) => Ok(Some(text.as_str())),
        Ok(None) => Ok(None),
        Err(reason) => Err(reason.as_str()),
    };
    let form = &mut *dialog;
    // Three options a row, in the same three columns on every row: what to
    // look in, where, and how to match.
    let choice = CommandForm::new(TITLE, PRIMARY)
        .option_columns()
        .describe(DESCRIPTION)
        .primary_enabled(count > 0)
        .status(status)
        .show(ctx, |rows| {
            let find = Id::new(FIND_FIELD);
            rows.text("Find", find, &mut form.find, "text or pattern");
            rows.text("Replace", Id::new(REPLACE_FIELD), &mut form.replace, "");
            rows.check_columns(
                "Look in",
                &mut [
                    ("Instance names", &mut form.targets.instance_names),
                    ("Net labels", &mut form.targets.net_labels),
                    ("Values", &mut form.targets.values),
                ],
            );
            rows.segmented_columns("Scope", "find-replace-scope", &SCOPES, &mut form.scope);
            rows.check_columns(
                "Match",
                &mut [
                    ("Whole word", &mut form.options.whole_word),
                    ("Match case", &mut form.options.match_case),
                    ("Regular expression", &mut form.options.regex),
                ],
            );
            Some(find)
        });
    match choice {
        DialogChoice::Primary => match replace_all(dialog, edit_in_progress, model, workbench) {
            Ok((texts, documents)) => {
                dialog.open = false;
                workbench.prompt(match documents {
                    1 => format!("Replaced {texts} {}.", plural(texts, "match", "matches")),
                    _ => format!("Replaced {texts} matches in {documents} cell views."),
                });
            }
            Err(reason) => workbench.prompt(reason),
        },
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            dialog.open = false;
        }
        DialogChoice::None | DialogChoice::Secondary => {}
    }
}

/// What the footer says about the draft: how many matches, that there are
/// none, or why Replace all is refused. A form nobody has typed into has
/// refused nothing, so it states what Replace all is waiting for, without the
/// failure mark.
pub(crate) fn find_replace_summary(
    find: &str,
    found: &Result<Vec<DocumentMatches>, String>,
    count: usize,
) -> Result<Option<String>, String> {
    if find.is_empty() {
        return Ok(Some("Enter the text to find.".to_owned()));
    }
    match found {
        Ok(found) if count > 0 => Ok(Some(match found.len() {
            1 => format!("{count} {}", plural(count, "match", "matches")),
            documents => format!("{count} matches in {documents} cell views"),
        })),
        Ok(_) => Ok(Some("No matches".to_owned())),
        Err(reason) => Err(reason.clone()),
    }
}

fn plural<'a>(count: usize, one: &'a str, many: &'a str) -> &'a str {
    if count == 1 { one } else { many }
}

#[cfg(test)]
pub(crate) mod tests {

    use crate::state::{Component, ComponentType, Point};

    pub(crate) fn part(id: u64, name: &str, value: &str) -> Component {
        let mut component =
            Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        component.name = name.to_owned();
        component.value = value.to_owned();
        component
    }
}

//! Find: one tool over the whole design, as KiCad's Find and Virtuoso's
//! Search are.
//!
//! The Navigator's search box is its field. Every instance whose name,
//! value or parameter matches, and every net label whose name does, is a
//! hit, in the design's root cell view and in every cell view placed below
//! it, read-only ones included. Enter and F3 step through the hits; each
//! opens its cell view and sheet, selects the object and centres it.
//! Unfolded, the Replace row replaces the hits that are ticked, as one undo
//! step however many cell views they are in, and nothing changes anywhere
//! unless every new name in every one of them is legal.

use std::collections::BTreeSet;

use rspice_design::schematic::document::SchematicDocument;
use rspice_design::schematic::find_replace::{
    FindField, FindMatch, FindOptions, FindQuery, FindTargets, find_in_document, refusal,
};
use rspice_design::schematic::selection::Selection;

use crate::schematic::view::sheet_visibility::object_is_on_active_sheet;
use crate::state::{CellViewRef, InstancePath, Point};
use crate::workbench::app::actions::workspace::sync_active_schematic;
use crate::workbench::app_state::Model;
use crate::workbench::design::navigator::hierarchy_tree::open_occurrence;
use crate::workbench::framework::requests::ShowOnSchematic;
use crate::workbench::slots::{DesignState, RunsState};
use crate::workbench::state::WorkbenchSession;

/// Find's draft. It outlives one search, as a find box does; the text it
/// finds is the Navigator's search box.
#[derive(Debug, Clone, Default)]
pub struct FindState {
    /// Whether the Replace row is unfolded under the hits' band.
    pub replacing: bool,
    pub replace: String,
    pub options: FindOptions,
    /// The hit Enter, F3 or a click last showed.
    pub(crate) current: Option<HitKey>,
    /// The hits the reader unticked, which Replace leaves alone.
    pub(crate) unticked: BTreeSet<HitKey>,
}

/// One hit, across frames: which text of which object of which cell view.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct HitKey {
    cell_view: String,
    object_id: u64,
    field: FindField,
}

/// The hits in one cell view.
#[derive(Debug, Clone)]
pub struct DocumentMatches {
    pub(crate) reference: CellViewRef,
    pub(crate) key: String,
    pub path: String,
    /// Where the design places the cell view first, which a hit in it opens.
    pub(crate) occurrence: InstancePath,
    /// Whether nothing in the cell view may change, so Replace passes it by.
    pub read_only: bool,
    pub matches: Vec<FindMatch>,
}

impl DocumentMatches {
    pub fn hit_key(&self, found: &FindMatch) -> HitKey {
        HitKey {
            cell_view: self.key.clone(),
            object_id: found.object_id,
            field: found.field.clone(),
        }
    }
}

impl FindState {
    /// Whether Replace changes `found` of `document`.
    pub(crate) fn ticks(&self, document: &DocumentMatches, found: &FindMatch) -> bool {
        !document.read_only && !self.unticked.contains(&document.hit_key(found))
    }

    pub fn toggle_tick(&mut self, key: HitKey) {
        if !self.unticked.remove(&key) {
            self.unticked.insert(key);
        }
    }

    /// Where the current hit stands among `found`, counting from zero.
    pub(crate) fn current_position(&self, found: &[DocumentMatches]) -> Option<usize> {
        let current = self.current.as_ref()?;
        hits(found).position(|(document, found)| document.hit_key(found) == *current)
    }
}

/// Every hit in `found`, in the order Enter steps through them.
pub(crate) fn hits(
    found: &[DocumentMatches],
) -> impl Iterator<Item = (&DocumentMatches, &FindMatch)> {
    found
        .iter()
        .flat_map(|document| document.matches.iter().map(move |found| (document, found)))
}

/// Every hit of `find` under the form's options, cell view by cell view,
/// the design's root first; or why the text cannot be searched for.
pub fn find_hits(
    find: &str,
    form: &FindState,
    model: &Model,
) -> Result<Vec<DocumentMatches>, String> {
    let query = FindQuery::new(find, &form.replace, form.options, FindTargets::default())?;
    let mut found = Vec::new();
    for (reference, occurrence) in documents_in_design(model) {
        let Some(document) = document_for(model, &reference) else {
            continue;
        };
        let matches = find_in_document(document, &query, |_| true);
        if !matches.is_empty() {
            found.push(DocumentMatches {
                read_only: read_only(model, &reference),
                key: reference.key(),
                path: reference.display_path(),
                reference,
                occurrence,
                matches,
            });
        }
    }
    Ok(found)
}

/// The cell views Find reaches, each once with the first occurrence that
/// places it: the root of the design the canvas is in, then every cell view
/// placed below it. The cell view in front is reached even when it was
/// opened on its own.
fn documents_in_design(model: &Model) -> Vec<(CellViewRef, InstancePath)> {
    let content = &model.workspace.content;
    let active = content.active_schematic_reference();
    let root = content.active_occurrence_or_root().root;
    let root = if document_for(model, &root).is_some() {
        root
    } else {
        active.clone()
    };
    let mut seen = BTreeSet::from([root.key()]);
    let mut order = vec![(root, InstancePath::root())];
    let mut next = 0;
    while let Some((reference, path)) = order.get(next).cloned() {
        next += 1;
        let Some(document) = document_for(model, &reference) else {
            continue;
        };
        for component in &document.components {
            let Some(binding) = component.library_cell.as_ref() else {
                continue;
            };
            let child = CellViewRef::new(&binding.library, &binding.cell, &binding.view);
            if document_for(model, &child).is_none() || seen.contains(&child.key()) {
                continue;
            }
            let Ok(occurrence) = path.child(&component.name) else {
                continue;
            };
            seen.insert(child.key());
            order.push((child, occurrence));
        }
    }
    if !seen.contains(&active.key()) {
        order.insert(0, (active, content.occurrence_path()));
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
        || (*reference == model.workspace.content.active_schematic_reference()
            && model.schematic_edit_read_only())
        || model
            .library_manager
            .get_library(&reference.library)
            .is_some_and(|library| library.read_only)
}

/// Where `found` is drawn in `document`.
fn hit_position(document: &SchematicDocument, found: &FindMatch) -> Option<Point> {
    match found.field {
        FindField::NetLabel => document
            .net_labels
            .iter()
            .find(|label| label.id == found.object_id)
            .map(|label| label.pos),
        _ => document
            .components
            .iter()
            .find(|component| component.id == found.object_id)
            .map(|component| component.pos),
    }
}

/// Show one hit: open its cell view where the design places it, unless it
/// is the one in front, then bring forward the sheet that draws it, select
/// it and centre on it.
pub(crate) fn show_hit(
    design: &mut DesignState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    runs: &RunsState,
    document: &DocumentMatches,
    found: &FindMatch,
) -> bool {
    let active = model.workspace.content.active_schematic_reference();
    if !document.key.eq_ignore_ascii_case(&active.key()) {
        open_occurrence(design, model, wb, runs, &document.occurrence);
    }
    let Some(point) =
        document_for(model, &document.reference).and_then(|drawing| hit_position(drawing, found))
    else {
        wb.prompt(format!(
            "{} is no longer in {}.",
            found.owner, document.path
        ));
        return false;
    };
    let sheet = model
        .workspace
        .content
        .design_management
        .sheet_catalog(&document.key)
        .filter(|catalog| catalog.sheets().len() > 1)
        .and_then(|catalog| catalog.sheet_for_object(found.object_id));
    let mut selection = Selection::default();
    match found.field {
        FindField::NetLabel => selection.select_net_label(found.object_id),
        _ => selection.select_component(found.object_id),
    }
    design
        .navigation_parts(model, wb, runs)
        .show_on_schematic(ShowOnSchematic {
            cell_view: document.reference.clone(),
            sheet,
            point,
            selection,
        })
}

/// Step to the hit after the current one (`forward`) or before it, wrapping
/// at the ends, and show it. With no current hit the first or the last is
/// shown.
pub(crate) fn step_hit(
    design: &mut DesignState,
    model: &mut Model,
    wb: &mut WorkbenchSession,
    runs: &RunsState,
    find: &str,
    forward: bool,
) {
    let found = match find_hits(find, &design.find_replace, model) {
        Ok(found) => found,
        Err(reason) => {
            wb.prompt(reason);
            return;
        }
    };
    let count = hits(&found).count();
    if count == 0 {
        wb.prompt(format!("Nothing matches {find}."));
        return;
    }
    let next = match (design.find_replace.current_position(&found), forward) {
        (Some(at), true) => (at + 1) % count,
        (Some(at), false) => (at + count - 1) % count,
        (None, true) => 0,
        (None, false) => count - 1,
    };
    let Some((document, hit)) = hits(&found).nth(next) else {
        return;
    };
    design.find_replace.current = Some(document.hit_key(hit));
    show_hit(design, model, wb, runs, document, hit);
}

/// Select every hit the sheet in view draws, and say how many more lie on
/// other sheets or in other cell views, where nothing can be selected from
/// here.
pub(crate) fn select_all_hits(
    model: &mut Model,
    wb: &mut WorkbenchSession,
    found: &[DocumentMatches],
) {
    let active = model.workspace.content.active_schematic_reference().key();
    let mut selection = Selection::default();
    let mut elsewhere = 0;
    for (document, found) in hits(found) {
        if document.key.eq_ignore_ascii_case(&active)
            && object_is_on_active_sheet(&*model, found.object_id)
        {
            match found.field {
                FindField::NetLabel => selection.select_net_label(found.object_id),
                _ => selection.select_component(found.object_id),
            }
        } else {
            elsewhere += 1;
        }
    }
    let selected = selection.count();
    let editor = &mut model.schematic.session.editor;
    editor.net_highlight.clear();
    editor.selection = selection;
    wb.prompt(match (selected, elsewhere) {
        (0, _) => "No hit is on this sheet.".to_owned(),
        (selected, 0) => format!(
            "Selected {selected} {}.",
            plural(selected, "object", "objects")
        ),
        (selected, elsewhere) => format!(
            "Selected {selected} {}; {elsewhere} more {} on other sheets or cell views.",
            plural(selected, "object", "objects"),
            plural(elsewhere, "hit is", "hits are"),
        ),
    });
}

/// The ticked hits of the cell views that may change, cell view by cell
/// view.
fn ticked(form: &FindState, found: &[DocumentMatches]) -> Vec<(CellViewRef, Vec<FindMatch>)> {
    found
        .iter()
        .filter_map(|document| {
            let matches = document
                .matches
                .iter()
                .filter(|found| form.ticks(document, found))
                .cloned()
                .collect::<Vec<_>>();
            (!matches.is_empty()).then(|| (document.reference.clone(), matches))
        })
        .collect()
}

/// Why Replace would be refused, or `None`. It is a pure reading of the
/// hits, so the Replace row states it, with the button disabled, before
/// anyone presses it.
pub fn replace_refusal(
    form: &FindState,
    found: &Result<Vec<DocumentMatches>, String>,
    edit_in_progress: bool,
    model: &Model,
) -> Option<String> {
    let found = match found {
        Ok(found) => found,
        Err(reason) => return Some(reason.clone()),
    };
    if found.is_empty() {
        return Some("Nothing matches.".to_owned());
    }
    if edit_in_progress {
        return Some("Finish or cancel the edit in progress first.".to_owned());
    }
    let edits = ticked(form, found);
    if edits.is_empty() {
        return Some(if found.iter().all(|document| document.read_only) {
            "Every hit is in a read-only cell view.".to_owned()
        } else {
            "Tick the hits to replace.".to_owned()
        });
    }
    edits.iter().find_map(|(reference, matches)| {
        let source = document_for(model, reference)?;
        let refused = refusal(source, matches)?;
        Some(if edits.len() > 1 {
            format!("{}: {refused}", reference.display_path())
        } else {
            refused
        })
    })
}

/// Replace the ticked hits of `find`: one undo step, wherever they are.
/// Every cell view is checked before anything changes. Returns how many
/// texts changed and in how many cell views.
pub fn replace_ticked(
    form: &mut FindState,
    find: &str,
    edit_in_progress: bool,
    model: &mut Model,
    workbench: &mut WorkbenchSession,
) -> Result<(usize, usize), String> {
    let found = find_hits(find, form, model);
    if let Some(reason) = replace_refusal(form, &found, edit_in_progress, model) {
        return Err(reason);
    }
    let edits = ticked(form, &found?);
    let active = model.workspace.content.active_schematic_reference().key();
    let changed = match edits.as_slice() {
        // Text alone in one cell view is that cell view's own step.
        [(reference, matches)]
            if !matches
                .iter()
                .any(|found| found.field == FindField::InstanceName) =>
        {
            if reference.key() == active {
                model.schematic.replace_text(matches)?
            } else if let Some(mut buffer) = model.workspace.schematic_editor_mut(&reference.key())
            {
                buffer.editor.replace_text(matches)?
            } else {
                0
            }
        }
        // Renames carry the references that name the instance; several
        // cell views change as one project step.
        _ => model.replace_text_across_cell_views(&edits)?,
    };
    form.current = None;
    form.unticked.clear();
    sync_active_schematic(model, workbench);
    Ok((changed, edits.len()))
}

/// What Replace says once it has replaced.
pub fn replaced_summary(texts: usize, documents: usize) -> String {
    match documents {
        1 => format!("Replaced {texts} {}.", plural(texts, "hit", "hits")),
        _ => format!("Replaced {texts} hits in {documents} cell views."),
    }
}

fn plural<'a>(count: usize, one: &'a str, many: &'a str) -> &'a str {
    if count == 1 { one } else { many }
}

#[cfg(any(test, feature = "test-support"))]
pub mod tests {

    use crate::state::{Component, ComponentType, Point};

    pub fn part(id: u64, name: &str, value: &str) -> Component {
        let mut component =
            Component::new(id, ComponentType::Resistor, Point::new(id as i32 * 100, 0));
        component.name = name.to_owned();
        component.value = value.to_owned();
        component
    }
}

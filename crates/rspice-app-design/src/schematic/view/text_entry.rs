//! The entry a double-click on a part's name or value opens where the text
//! stands: the text it holds is selected, so typing replaces it; Enter
//! commits what was typed as one edit, and Escape or a click elsewhere leaves
//! the part as it was. KiCad edits a symbol field the same way, from a
//! double-click on the field itself.

use super::parts::CanvasParts;
use super::sheet_visibility::object_is_on_active_sheet;
use super::{SchematicSymbolContext, viewport::Viewport};
use crate::state::{Component, ComponentType};
use crate::ui::widgets::{CanvasEntry, CanvasEntryChoice};
use crate::workbench::app::dialogs::selection::rename::{
    RenameSelectionTarget, RenameValidation, apply_commit, validate_draft,
};
use rspice_design::schematic::label_position::InstanceText;
use rspice_schematic_editor::view::instance_text::instance_texts_at;
use rspice_vocabulary::quantity::engineering::parse_engineering_value;

/// An open entry over one part's name or value.
#[derive(Debug, Clone)]
pub struct InstanceTextEntry {
    /// The part as it stood when the entry opened: an edit made meanwhile
    /// is not overwritten.
    pub(crate) expected: Component,
    pub(crate) text: InstanceText,
    pub(crate) draft: String,
    /// Where on the screen the double-click was.
    pub(crate) anchor: egui::Pos2,
    caret_placed: bool,
}

/// Whether a double-click on `part`'s value edits it where it stands: the
/// value is a quantity (`10k`) or an expression (`{rload*2}`), as a passive
/// part's is. A source letters a summary of its waveform, a port's value is
/// its interface name, a cell's comes from its library, and a device's names
/// a model the library must hold: each of those opens the part's Properties
/// instead, where the change is checked whole.
pub(crate) fn value_edits_in_place(part: &Component) -> bool {
    let value = part.value.trim();
    part.library_cell.is_none()
        && !part.kind.is_source()
        && !matches!(
            part.kind,
            ComponentType::Port | ComponentType::CellInstance | ComponentType::Ground
        )
        && ((value.starts_with('{') && value.ends_with('}'))
            || parse_engineering_value(value).is_ok())
}

/// Which of part `id`'s texts a double-click at `pointer` edits where it
/// stands: its name, or its value when that edits in place. `None` on a
/// read-only view, which opens the part's Properties to read instead.
pub(crate) fn editable_text_at(
    state: &CanvasParts<'_>,
    symbols: &SchematicSymbolContext,
    viewport: &Viewport,
    ctx: &egui::Context,
    pointer: egui::Pos2,
    id: u64,
) -> Option<InstanceText> {
    if state.schematic_edit_read_only() {
        return None;
    }
    let view = state.design_view();
    let components = view.visible_objects(&view.document.components, |part| part.id);
    let hit = instance_texts_at(
        &view,
        &components,
        symbols,
        viewport,
        ctx.pixels_per_point(),
        pointer,
        false,
    )
    .into_iter()
    .find(|hit| hit.component == id)?;
    let part = view.document.components.iter().find(|part| part.id == id)?;
    (hit.text == InstanceText::Name || value_edits_in_place(part)).then_some(hit.text)
}

/// Open the entry over `part`'s `text`, holding what the part letters there.
pub(crate) fn open(
    state: &mut CanvasParts<'_>,
    part: &Component,
    text: InstanceText,
    at: egui::Pos2,
) {
    let draft = match text {
        InstanceText::Name => part.name.clone(),
        InstanceText::Value => part.value.clone(),
    };
    state.design.interaction.text_entry = Some(InstanceTextEntry {
        expected: part.clone(),
        text,
        draft,
        anchor: at,
        caret_placed: false,
    });
}

/// Show the open entry, if one is open, and act on what the reader did.
pub(crate) fn show(ctx: &egui::Context, state: &mut CanvasParts<'_>) {
    let Some(open) = state.design.interaction.text_entry.as_ref() else {
        return;
    };
    let (label, refusal) = match open.text {
        InstanceText::Name => ("Name", name_refusal(state, open)),
        InstanceText::Value => ("Value", None),
    };
    let field = egui::Id::new("schematic.instance-text-entry");
    let Some(entry) = state.design.interaction.text_entry.as_mut() else {
        return;
    };
    let take_caret = !std::mem::replace(&mut entry.caret_placed, true);
    let choice = CanvasEntry::new("instance-text", label, entry.anchor)
        .commit_enabled(refusal.is_none() && !entry.draft.trim().is_empty())
        .status(refusal.as_deref().map_or(Ok(None), Err))
        .show(ctx, |rows| {
            let response = rows.text(label, field, &mut entry.draft, "");
            // Escape is the entry's: it closes the entry, and the selection
            // behind it stays as it was.
            if response.has_focus() {
                ctx.memory_mut(|memory| {
                    memory.set_focus_lock_filter(
                        field,
                        egui::EventFilter {
                            horizontal_arrows: true,
                            escape: true,
                            ..Default::default()
                        },
                    );
                });
            }
            crate::workbench::app_state::report_escape_owner(&response);
            Some(field)
        });
    if take_caret {
        ctx.memory_mut(|memory| memory.request_focus(field));
        let mut edit = egui::text_edit::TextEditState::load(ctx, field).unwrap_or_default();
        edit.cursor
            .set_char_range(Some(egui::text::CCursorRange::two(
                egui::text::CCursor::new(0),
                egui::text::CCursor::new(entry.draft.chars().count()),
            )));
        edit.store(ctx, field);
    }
    match choice {
        CanvasEntryChoice::Commit => {
            if let Some(entry) = state.design.interaction.text_entry.take() {
                commit(state, entry);
            }
            crate::workbench::app_state::request_engineering_canvas_focus(ctx);
        }
        CanvasEntryChoice::Cancelled => {
            state.design.interaction.text_entry = None;
            crate::workbench::app_state::request_engineering_canvas_focus(ctx);
        }
        CanvasEntryChoice::None => {}
    }
}

/// Why the typed name cannot be the part's, as Rename would say it.
fn name_refusal(state: &CanvasParts<'_>, entry: &InstanceTextEntry) -> Option<String> {
    match validate_draft(
        &state.model.schematic,
        &RenameSelectionTarget::Component(Box::new(entry.expected.clone())),
        &entry.draft,
        &|id| object_is_on_active_sheet(&*state.model, id),
    ) {
        RenameValidation::Invalid(message) => Some(message),
        RenameValidation::Valid(_) | RenameValidation::Unchanged => None,
    }
}

/// Commit what the entry holds as one edit, and say what changed.
fn commit(state: &mut CanvasParts<'_>, entry: InstanceTextEntry) {
    let typed = entry.draft.trim().to_owned();
    let previous = match entry.text {
        InstanceText::Name => entry.expected.name.clone(),
        InstanceText::Value => entry.expected.value.clone(),
    };
    if typed == previous {
        return;
    }
    let outcome = match entry.text {
        InstanceText::Name => match validate_draft(
            &state.model.schematic,
            &RenameSelectionTarget::Component(Box::new(entry.expected.clone())),
            &typed,
            &|id| object_is_on_active_sheet(&*state.model, id),
        ) {
            RenameValidation::Valid(commit) => apply_commit(state.model, *commit),
            RenameValidation::Invalid(message) => Err(message),
            RenameValidation::Unchanged => Ok(false),
        },
        InstanceText::Value => {
            let mut candidate = entry.expected.clone();
            candidate.value.clone_from(&typed);
            state
                .model
                .edit_component_transaction(&entry.expected, candidate, "edit value")
        }
    };
    match outcome {
        Ok(true) => {
            state.sync_active_schematic_to_workspace();
            let what = match entry.text {
                InstanceText::Name => format!("Renamed {previous} to {typed}."),
                InstanceText::Value => {
                    format!("{}: value {previous} is now {typed}.", entry.expected.name)
                }
            };
            state.prompt(what);
        }
        Ok(false) => {}
        Err(error) => state.prompt(error),
    }
}

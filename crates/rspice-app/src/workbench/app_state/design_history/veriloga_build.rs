//! Build settings, saved as one step.
//!
//! Saving Build settings writes the build profile a Verilog-A view's source
//! holds and, when the entry module moved, binds the view to the module the
//! profile now names. A profile naming one module while the view compiles
//! another can only fail to compile, so the two are one step: undo puts back
//! both, or neither.
//!
//! Each half is guarded by what it owns and nothing else: the profile by the
//! revision of the source bundle, the binding by the view's
//! `veriloga.module`. A compile after the save appends qualification evidence
//! and rewrites the view's ports, neither of which this step owns, so it does
//! not stand in the way; an edit of the source or another module choice
//! does, and the step then refuses rather than overwrite it.

use super::*;
use crate::state::ProjectSourceId;

/// The view metadata key naming the module a Verilog-A view compiles.
const MODULE_KEY: &str = "veriloga.module";

/// What one save of Build settings changed, handed over by the save.
pub(crate) struct VerilogABuildSettingsHistoryEntry {
    pub(crate) bundle: ProjectSourceId,
    /// The bundle's revision before the save and after it; equal when the
    /// profile already read as the form saved it.
    pub(crate) before_revision: ObjectRevision,
    pub(crate) after_revision: ObjectRevision,
    /// The view the save bound to another module, when it did.
    pub(crate) binding: Option<VerilogAModuleBinding>,
}

/// A view's `veriloga.module` on each side of a save.
#[derive(Debug, Clone)]
pub(crate) struct VerilogAModuleBinding {
    pub(crate) view: CellViewRef,
    pub(crate) before: Option<String>,
    pub(crate) after: String,
}

#[derive(Debug, Clone)]
pub(super) struct VerilogABuildSettingsRecord {
    pub(super) description: String,
    profile: Option<ProfileStep>,
    binding: Option<VerilogAModuleBinding>,
}

/// The profile half: two retained revisions of one source bundle.
///
/// The bundle restores a retained revision as a new one, so after each
/// crossing the revision just left becomes the one to restore next: it is
/// the newest the bundle retains with that content.
#[derive(Debug, Clone)]
struct ProfileStep {
    bundle: ProjectSourceId,
    /// The retained revision undo restores.
    before: ObjectRevision,
    /// The retained revision redo restores.
    after: ObjectRevision,
    /// The revision the bundle stands at while the step can be undone.
    undo_guard: ObjectRevision,
    /// The revision the bundle stands at while the step can be redone.
    redo_guard: Option<ObjectRevision>,
}

impl AppState {
    /// Record one save of Build settings as one undo step. A save that
    /// changed nothing records nothing.
    pub(crate) fn record_veriloga_build_settings(
        &mut self,
        entry: VerilogABuildSettingsHistoryEntry,
    ) {
        let profile = (entry.before_revision != entry.after_revision).then_some(ProfileStep {
            bundle: entry.bundle,
            before: entry.before_revision,
            after: entry.after_revision,
            undo_guard: entry.after_revision,
            redo_guard: None,
        });
        let binding = entry
            .binding
            .filter(|binding| binding.before.as_deref() != Some(binding.after.as_str()));
        if profile.is_none() && binding.is_none() {
            return;
        }
        self.push_project_record(
            RecordHeader::project_only(),
            ProjectDesignBody::VerilogABuildSettings(Box::new(VerilogABuildSettingsRecord {
                description: "Build settings".to_owned(),
                profile,
                binding,
            })),
        );
    }
}

impl VerilogABuildSettingsRecord {
    pub(super) fn after_design_matches(&self, state: &AppState) -> bool {
        self.profile.as_ref().is_none_or(|profile| {
            bundle_revision(state, profile.bundle) == Some(profile.undo_guard)
        }) && self
            .binding
            .as_ref()
            .is_none_or(|binding| bound_module(state, &binding.view) == Some(&binding.after))
    }

    pub(super) fn before_design_matches(&self, state: &AppState) -> bool {
        self.profile.as_ref().is_none_or(|profile| {
            profile.redo_guard.is_some()
                && bundle_revision(state, profile.bundle) == profile.redo_guard
        }) && self
            .binding
            .as_ref()
            .is_none_or(|binding| bound_module(state, &binding.view) == binding.before.as_ref())
    }

    pub(super) fn validate_mutation(
        &self,
        state: &AppState,
        operation: &str,
    ) -> Result<(), String> {
        if !state.project_lifecycle.is_open() {
            return Err(format!(
                "Build settings cannot be {operation} without an open project."
            ));
        }
        if state.workbench.safe_mode.project_read_only() {
            return Err(format!(
                "Build settings cannot be {operation} while the project is read-only."
            ));
        }
        if let Some(binding) = &self.binding {
            let library = state
                .library_manager
                .get_library(&binding.view.library)
                .ok_or_else(|| format!("Library '{}' no longer exists.", binding.view.library))?;
            if library.read_only {
                return Err(format!(
                    "Build settings cannot be {operation} because library '{}' is read-only.",
                    binding.view.library
                ));
            }
        }
        Ok(())
    }

    pub(super) fn apply_before(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.after_design_matches(state) {
            return Err(
                "Build settings cannot be undone because the source or its module changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "undone")?;
        if let Some(profile) = &mut self.profile {
            let restored =
                restore_bundle(state, profile.bundle, profile.undo_guard, profile.before)?;
            profile.after = profile.undo_guard;
            profile.redo_guard = Some(restored);
        }
        if let Some(binding) = &self.binding {
            bind_module(state, &binding.view, binding.before.as_deref());
        }
        Ok(())
    }

    pub(super) fn apply_after(&mut self, state: &mut AppState) -> Result<(), String> {
        if !self.before_design_matches(state) {
            return Err(
                "Build settings cannot be redone because the source or its module changed."
                    .to_owned(),
            );
        }
        self.validate_mutation(state, "redone")?;
        if let Some(profile) = &mut self.profile {
            let current = profile
                .redo_guard
                .expect("a step on the redo stack was undone");
            let restored = restore_bundle(state, profile.bundle, current, profile.after)?;
            profile.before = current;
            profile.undo_guard = restored;
            profile.redo_guard = None;
        }
        if let Some(binding) = &self.binding {
            bind_module(state, &binding.view, Some(&binding.after));
        }
        Ok(())
    }
}

fn bundle_revision(state: &AppState, bundle: ProjectSourceId) -> Option<ObjectRevision> {
    state
        .workspace
        .content
        .project_sources
        .get_bundle(bundle)
        .map(|bundle| bundle.revision())
}

fn bound_module<'a>(state: &'a AppState, view: &CellViewRef) -> Option<&'a String> {
    state
        .library_manager
        .get_library(&view.library)?
        .get_cell(&view.cell)?
        .get_view(&view.view)?
        .metadata
        .get(MODULE_KEY)
}

/// Put the bundle back at `retained`, from `current`, and say what it became.
/// What it compiled to is evidence of the text it left, so that goes too.
fn restore_bundle(
    state: &mut AppState,
    bundle: ProjectSourceId,
    current: ObjectRevision,
    retained: ObjectRevision,
) -> Result<ObjectRevision, String> {
    state
        .workspace
        .content
        .restore_project_source_bundle_revision(bundle, current, retained)
        .map_err(|error| format!("Build settings could not be restored: {error}"))?;
    let restored = bundle_revision(state, bundle)
        .ok_or_else(|| "The Verilog-A source no longer exists.".to_owned())?;
    state.ui.code_workspace.veriloga.void_evidence();
    let owner = state
        .workspace
        .content
        .project_sources
        .get_bundle(bundle)
        .map(|bundle| bundle.owner().clone());
    if let Some(crate::state::ProjectSourceOwner::CellView { reference }) = owner
        && let Some(open) = state
            .workspace
            .content
            .open_views
            .iter_mut()
            .find(|open| open.reference == reference)
    {
        open.dirty = true;
    }
    Ok(restored)
}

/// Bind `view` to `module`, or to none.
fn bind_module(state: &mut AppState, view: &CellViewRef, module: Option<&str>) {
    let Some(mut library) = state.library_manager.edit_library(&view.library) else {
        return;
    };
    let Some(mut metadata) = library
        .library()
        .get_cell(&view.cell)
        .and_then(|cell| cell.get_view(&view.view))
        .map(|view| view.metadata.clone())
    else {
        return;
    };
    match module {
        Some(module) => metadata.insert(MODULE_KEY.to_owned(), module.to_owned()),
        None => metadata.remove(MODULE_KEY),
    };
    library.replace_view_metadata(&view.cell, &view.view, metadata);
    library.mark_view_modified(&view.cell, &view.view);
    state.workspace.content.project_metadata_dirty = true;
}

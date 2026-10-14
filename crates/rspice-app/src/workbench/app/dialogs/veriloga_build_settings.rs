//! Build settings: the open Verilog-A view's build profile, as a form.
//!
//! The profile is a document of the view's source bundle, and Compile reads
//! it without asking anything; this form is where it is changed. Saving
//! writes that document through the edit every source change takes, so it
//! marks the view's tab, voids the compile evidence of the old text, keeps
//! that text in the source's revision history, and is refused on a read-only
//! project. A view still on the compiler's built-in defaults gains the
//! document on its first save.
//!
//! Each list field states why it refuses an entry in a line under it, there
//! only while the entry is refused: an empty line under every list would put
//! a hole in the form for an error that is rarely there. A refused entry holds
//! Save until it is fixed or cleared, since saving without it would drop what
//! the reader typed. The status line is left for what belongs to no field: a
//! read-only project, or a save the project refused.
//!
//! The entry module is offered only when the source declares more than one
//! top module. Choosing another makes it the module this view compiles: the
//! profile names it, and the view binds it. A profile naming one module while
//! the view compiles another could only fail, so the save is one step, and
//! one Undo takes back the profile and the binding together.

use std::collections::BTreeMap;

use egui::{Context, Id};
use rspice_simulation::project_veriloga::build_profile::{
    VerilogABuildProfile, VerilogACheckProfile, VerilogAFallbackPolicy, is_portable_include_path,
    is_preprocessor_symbol, resolve_veriloga_build_profile,
};

use crate::diagnostics::ConsoleMessage;
use crate::state::{ProjectSourceLanguage, ProjectSourceOwner, ViewType};
use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine, FormRows, ListWords};
use crate::workbench::app::{RSpiceApp, VERILOGA_LIBRARY_NAME};
use crate::workbench::app_state::{VerilogABuildSettingsHistoryEntry, VerilogAModuleBinding};
use crate::workbench::documents::code_workspace::{self, SelectedVerilogASource};
use crate::workbench::{MessageCatalog, MessageId};

/// The label column, as wide as "If a target fails" needs. Every row starts
/// its control at this one x.
const LABEL_COLUMN: f32 = 112.0;

const ENTRY_MODULE_ID: &str = "workbench.veriloga-build.entry-module";

/// The view metadata key naming the module a Verilog-A view compiles.
const MODULE_KEY: &str = "veriloga.module";

fn include_id() -> Id {
    Id::new("workbench.veriloga-build.include")
}

fn define_id() -> Id {
    Id::new("workbench.veriloga-build.define")
}

fn undefine_id() -> Id {
    Id::new("workbench.veriloga-build.undefine")
}

/// The form's draft: the profile as read, and what the reader has made of
/// each field it shows.
#[derive(Debug, Clone)]
pub(crate) struct VerilogABuildSettingsDraft {
    /// The closure the form was opened on. A save is bound to it, and refused
    /// if the source moved underneath.
    source: SelectedVerilogASource,
    /// The profile as read, carrying the policy the form does not show.
    profile: VerilogABuildProfile,
    /// The top modules the source declares: the entry module's choices.
    pub(crate) modules: Vec<String>,
    pub(crate) entry_module: String,
    pub(crate) generated_rust: bool,
    pub(crate) native_x64_jit: bool,
    pub(crate) reject_fallback: bool,
    pub(crate) include_paths: Vec<String>,
    pub(crate) include_draft: String,
    pub(crate) defines: Vec<(String, String)>,
    pub(crate) define_draft: String,
    pub(crate) undefines: Vec<String>,
    pub(crate) undefine_draft: String,
    /// Whether the profile carried undefines when the form opened. The row is
    /// shown only then: almost no profile has one.
    show_undefines: bool,
    pub(crate) checks: VerilogACheckProfile,
    /// Why the last save was refused, when no field owns the reason.
    pub(crate) error: Option<String>,
}

impl VerilogABuildSettingsDraft {
    /// The profile the draft describes, with any entry still in a list's
    /// field added to its list.
    fn profile(&self, messages: &MessageCatalog) -> Result<VerilogABuildProfile, String> {
        let mut draft = self.clone();
        draft.take_pending(messages)?;
        let mut profile = draft.profile;
        if draft.modules.len() > 1 {
            profile.entry_modules = vec![draft.entry_module];
        }
        profile.targets.generated_rust = draft.generated_rust;
        profile.targets.native_x64_jit = draft.native_x64_jit;
        profile.targets.fallback = if draft.reject_fallback {
            VerilogAFallbackPolicy::Reject
        } else {
            VerilogAFallbackPolicy::Allow
        };
        profile.include_paths = draft.include_paths;
        profile.preprocessor.defines = draft.defines.into_iter().collect::<BTreeMap<_, _>>();
        profile.preprocessor.undefines = draft.undefines;
        profile.checks = draft.checks;
        Ok(profile)
    }

    /// Move what each list's field holds into its list, or say why it cannot.
    fn take_pending(&mut self, messages: &MessageCatalog) -> Result<(), String> {
        if !self.include_draft.trim().is_empty() {
            self.add_include(messages)?;
        }
        if !self.define_draft.trim().is_empty() {
            self.add_define(messages)?;
        }
        if !self.undefine_draft.trim().is_empty() {
            self.add_undefine(messages)?;
        }
        Ok(())
    }

    fn include_refusal(&self, messages: &MessageCatalog) -> Option<String> {
        let path = self.include_draft.trim();
        if path.is_empty() {
            return None;
        }
        if !is_portable_include_path(path) {
            return Some(messages.text(MessageId::VerilogABuildSettingsIncludeInvalid));
        }
        self.include_paths
            .iter()
            .any(|existing| existing.eq_ignore_ascii_case(path))
            .then(|| {
                messages.format(
                    MessageId::VerilogABuildSettingsIncludeTwice,
                    &[("path", path)],
                )
            })
    }

    fn define_refusal(&self, messages: &MessageCatalog) -> Option<String> {
        if self.define_draft.trim().is_empty() {
            return None;
        }
        let (name, _) = typed_define(&self.define_draft);
        symbol_refusal(&name, messages).or_else(|| {
            if self.defines.iter().any(|(existing, _)| *existing == name) {
                Some(messages.format(
                    MessageId::VerilogABuildSettingsDefineTwice,
                    &[("name", &name)],
                ))
            } else if self.undefines.contains(&name) {
                Some(messages.format(
                    MessageId::VerilogABuildSettingsDefinedAndUndefined,
                    &[("name", &name)],
                ))
            } else {
                None
            }
        })
    }

    fn undefine_refusal(&self, messages: &MessageCatalog) -> Option<String> {
        let name = self.undefine_draft.trim();
        if name.is_empty() {
            return None;
        }
        symbol_refusal(name, messages).or_else(|| {
            if self.undefines.iter().any(|existing| existing == name) {
                Some(messages.format(
                    MessageId::VerilogABuildSettingsUndefineTwice,
                    &[("name", name)],
                ))
            } else if self.defines.iter().any(|(existing, _)| existing == name) {
                Some(messages.format(
                    MessageId::VerilogABuildSettingsDefinedAndUndefined,
                    &[("name", name)],
                ))
            } else {
                None
            }
        })
    }

    fn add_include(&mut self, messages: &MessageCatalog) -> Result<(), String> {
        if let Some(refusal) = self.include_refusal(messages) {
            return Err(refusal);
        }
        self.include_paths
            .push(std::mem::take(&mut self.include_draft).trim().to_owned());
        Ok(())
    }

    fn add_define(&mut self, messages: &MessageCatalog) -> Result<(), String> {
        if let Some(refusal) = self.define_refusal(messages) {
            return Err(refusal);
        }
        self.defines
            .push(typed_define(&std::mem::take(&mut self.define_draft)));
        Ok(())
    }

    fn add_undefine(&mut self, messages: &MessageCatalog) -> Result<(), String> {
        if let Some(refusal) = self.undefine_refusal(messages) {
            return Err(refusal);
        }
        self.undefines
            .push(std::mem::take(&mut self.undefine_draft).trim().to_owned());
        Ok(())
    }
}

/// `NAME=value` as typed, or a bare `NAME`, which defines it empty.
fn typed_define(typed: &str) -> (String, String) {
    match typed.split_once('=') {
        Some((name, value)) => (name.trim().to_owned(), value.trim().to_owned()),
        None => (typed.trim().to_owned(), String::new()),
    }
}

/// How a define reads in its list: `NAME=value`, or `NAME` when it is empty.
fn define_entry((name, value): &(String, String)) -> String {
    if value.is_empty() {
        name.clone()
    } else {
        format!("{name}={value}")
    }
}

fn symbol_refusal(name: &str, messages: &MessageCatalog) -> Option<String> {
    if name.is_empty() {
        Some(messages.text(MessageId::VerilogABuildSettingsDefineShape))
    } else if !is_preprocessor_symbol(name) {
        Some(messages.format(
            MessageId::VerilogABuildSettingsSymbolInvalid,
            &[("name", name)],
        ))
    } else {
        None
    }
}

/// Whether Build settings has a profile to show: a Verilog-A view in front,
/// with its source.
pub(crate) fn veriloga_build_settings_available(app: &RSpiceApp) -> bool {
    app.state.project_lifecycle.is_open()
        && app.state.workspace.content.active_view_type() == ViewType::VerilogA
        && code_workspace::active_source_bundle(
            &app.state.workspace,
            ProjectSourceLanguage::VerilogA,
        )
        .is_some()
}

/// Open Build settings on the profile of the Verilog-A view in front.
pub(crate) fn open_veriloga_build_settings(app: &mut RSpiceApp) -> Result<(), String> {
    let source = code_workspace::selected_veriloga_source(app)?;
    let resolved = resolve_veriloga_build_profile(source.bundle())?;
    let profile = resolved.profile;
    // A source that does not parse yet still has settings to edit; its entry
    // module is then the one the view or the profile already names.
    let declared = crate::state::workspace::survey_veriloga_bundle(source.bundle())
        .map(|survey| survey.tops.into_iter().map(|module| module.name).collect())
        .unwrap_or_else(|_| Vec::<String>::new());
    let entry_module = source
        .selected_module()
        .map(str::to_owned)
        .or_else(|| profile.entry_modules.first().cloned())
        .or_else(|| declared.first().cloned())
        .unwrap_or_default();
    let mut modules = declared;
    if !entry_module.is_empty() && !modules.contains(&entry_module) {
        modules.insert(0, entry_module.clone());
    }
    app.state.dialogs.veriloga_build_settings = Some(VerilogABuildSettingsDraft {
        modules,
        entry_module,
        generated_rust: profile.targets.generated_rust,
        native_x64_jit: profile.targets.native_x64_jit,
        reject_fallback: profile.targets.fallback == VerilogAFallbackPolicy::Reject,
        include_paths: profile.include_paths.clone(),
        include_draft: String::new(),
        defines: profile
            .preprocessor
            .defines
            .iter()
            .map(|(name, value)| (name.clone(), value.clone()))
            .collect(),
        define_draft: String::new(),
        show_undefines: !profile.preprocessor.undefines.is_empty(),
        undefines: profile.preprocessor.undefines.clone(),
        undefine_draft: String::new(),
        checks: profile.checks,
        error: None,
        source,
        profile,
    });
    Ok(())
}

/// Write the open form's profile, and close the form. Returns whether
/// anything changed.
///
/// The profile and the module the view binds are written as one step, which
/// one Undo takes back whole.
///
/// Refused, with the form left open, on a read-only project, for an entry a
/// list's field still holds that the list refuses, for a profile the build
/// refuses, and for a source that moved since the form opened.
pub(crate) fn save_veriloga_build_settings(app: &mut RSpiceApp) -> Result<bool, String> {
    let messages = app.state.ui.messages();
    if app.state.workbench.safe_mode.project_read_only() {
        return Err(messages.text(MessageId::VerilogABuildSettingsReadOnly));
    }
    let draft = app
        .state
        .dialogs
        .veriloga_build_settings
        .clone()
        .ok_or_else(|| "Build settings is not open.".to_owned())?;
    let profile = draft.profile(&messages)?;
    profile.to_toml()?;
    let rebind = module_rebind(app, &draft)?;
    let bundle = draft.source.bundle().id();
    let before_revision = draft.source.bundle().revision();
    code_workspace::write_veriloga_build_profile(app, &draft.source, &profile)?;
    let after_revision = app
        .state
        .workspace
        .content
        .project_sources
        .get_bundle(bundle)
        .map_or(before_revision, |bundle| bundle.revision());
    let binding = rebind.and_then(|rebind| rebind.apply(app));
    let changed = after_revision != before_revision || binding.is_some();
    app.state
        .record_veriloga_build_settings(VerilogABuildSettingsHistoryEntry {
            bundle,
            before_revision,
            after_revision,
            binding,
        });
    app.state.dialogs.veriloga_build_settings = None;
    Ok(changed)
}

/// The view's module binding a save moves to the chosen entry module, when
/// the view binds another.
struct ModuleRebind {
    reference: crate::state::CellViewRef,
    module: String,
}

/// What the chosen entry module asks of the view's binding, checked before
/// anything is written so a save is never half made.
fn module_rebind(
    app: &RSpiceApp,
    draft: &VerilogABuildSettingsDraft,
) -> Result<Option<ModuleRebind>, String> {
    let bound = draft.source.selected_module();
    if draft.modules.len() < 2 || bound.is_none_or(|bound| bound == draft.entry_module) {
        return Ok(None);
    }
    let ProjectSourceOwner::CellView { reference } = draft.source.bundle().owner() else {
        return Ok(None);
    };
    let library = app
        .state
        .library_manager
        .get_library(&reference.library)
        .ok_or_else(|| format!("Library '{}' no longer exists.", reference.library))?;
    if library.read_only {
        return Err(format!(
            "Library '{}' is read-only, so {} cannot compile another module.",
            reference.library,
            reference.display_path()
        ));
    }
    Ok(Some(ModuleRebind {
        reference: reference.clone(),
        module: draft.entry_module.clone(),
    }))
}

impl ModuleRebind {
    /// Bind the view to the module. Returns the binding on each side, or
    /// `None` when the view is gone.
    fn apply(self, app: &mut RSpiceApp) -> Option<VerilogAModuleBinding> {
        let mut metadata = app
            .state
            .library_manager
            .get_library(&self.reference.library)
            .and_then(|library| library.get_cell(&self.reference.cell))
            .and_then(|cell| cell.get_view(&self.reference.view))
            .map(|view| view.metadata.clone())?;
        let before = metadata.insert(MODULE_KEY.to_owned(), self.module.clone());
        {
            let mut edit = app
                .state
                .library_manager
                .edit_library(&self.reference.library)?;
            edit.replace_view_metadata(&self.reference.cell, &self.reference.view, metadata);
            edit.mark_view_modified(&self.reference.cell, &self.reference.view);
        }
        app.state.workspace.content.project_metadata_dirty = true;
        if self.reference.library == VERILOGA_LIBRARY_NAME {
            app.persist_global_veriloga_library_with_feedback();
        }
        Some(VerilogAModuleBinding {
            view: self.reference,
            before,
            after: self.module,
        })
    }
}

/// The refusal each list's field states, read once per frame.
struct FieldRefusals {
    include: Option<String>,
    define: Option<String>,
    undefine: Option<String>,
}

impl FieldRefusals {
    fn of(draft: &VerilogABuildSettingsDraft, messages: &MessageCatalog) -> Self {
        Self {
            include: draft.include_refusal(messages),
            define: draft.define_refusal(messages),
            undefine: draft.undefine_refusal(messages),
        }
    }

    fn any(&self) -> bool {
        self.include.is_some() || self.define.is_some() || self.undefine.is_some()
    }
}

/// State `refusal` in the line under the list field `field`, when there is
/// one. Without one the row ends at its field, an ordinary row gap above the
/// next.
fn refusal_line(rows: &mut FormRows<'_>, field: Id, refusal: Option<&String>) {
    if let Some(refusal) = refusal {
        rows.field_line(Some(field), FieldLine::Error(refusal));
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_veriloga_build_settings(&mut self, ctx: &Context) {
        let Some(mut draft) = self.state.dialogs.veriloga_build_settings.take() else {
            return;
        };
        // The form belongs to one source. If that source is gone, so is the
        // form: there is nothing left for it to save to.
        if self
            .state
            .workspace
            .content
            .project_sources
            .get_bundle(draft.source.bundle().id())
            .is_none()
        {
            return;
        }
        let messages = self.state.ui.messages();
        let read_only = self.state.workbench.safe_mode.project_read_only();
        let refusals = FieldRefusals::of(&draft, &messages);
        let can_save = !read_only && !refusals.any();
        let status = if read_only {
            Some(messages.text(MessageId::VerilogABuildSettingsReadOnly))
        } else {
            draft.error.clone()
        };
        let title = messages.text(MessageId::VerilogABuildSettingsTitle);
        let primary = messages.text(MessageId::VerilogABuildSettingsPrimary);
        let save = messages.text(MessageId::VerilogABuildSettingsSave);
        let description = messages.text(MessageId::VerilogABuildSettingsDescription);

        let mut edited = false;
        let choice = CommandForm::new(&title, &primary)
            .describe(&description)
            .secondary(&save)
            .label_column(LABEL_COLUMN)
            .primary_enabled(can_save)
            .secondary_enabled(can_save)
            .fields_enabled(!read_only)
            .status(match status.as_deref() {
                Some(message) => Err(message),
                None => Ok(None),
            })
            .status_lines(2)
            .status_fixed_at_open()
            .show(ctx, |rows| {
                edited = form_body(rows, &mut draft, &refusals, &messages);
                None
            });
        if edited {
            draft.error = None;
        }
        match choice {
            DialogChoice::Primary | DialogChoice::Secondary if can_save => {
                self.state.dialogs.veriloga_build_settings = Some(draft);
                match save_veriloga_build_settings(self) {
                    Ok(_) if choice == DialogChoice::Primary => {
                        if let Err(error) = code_workspace::request_veriloga_compile(self) {
                            self.state.push_user_message(ConsoleMessage::error(error));
                        }
                    }
                    Ok(_) => {}
                    Err(error) => {
                        if let Some(draft) = self.state.dialogs.veriloga_build_settings.as_mut() {
                            draft.error = Some(error);
                        }
                    }
                }
            }
            DialogChoice::Ghost | DialogChoice::Cancelled => {}
            DialogChoice::None | DialogChoice::Primary | DialogChoice::Secondary => {
                self.state.dialogs.veriloga_build_settings = Some(draft);
            }
        }
    }
}

/// The form's rows, in the order a build reads them. Returns whether the
/// reader changed anything this frame.
fn form_body(
    rows: &mut FormRows<'_>,
    draft: &mut VerilogABuildSettingsDraft,
    refusals: &FieldRefusals,
    messages: &MessageCatalog,
) -> bool {
    let text = |id| messages.text(id);
    let mut edited = false;

    if draft.modules.len() > 1
        && let Some(index) = rows.select(
            &text(MessageId::VerilogAEntryModule),
            ENTRY_MODULE_ID,
            &draft.entry_module,
            &draft.modules,
        )
        && let Some(module) = draft.modules.get(index)
    {
        draft.entry_module = module.clone();
        edited = true;
    }

    let (generated, jit) = (
        text(MessageId::VerilogABuildSettingsGeneratedRust),
        text(MessageId::VerilogABuildSettingsNativeJit),
    );
    edited |= rows.checks(
        &text(MessageId::VerilogABuildSettingsTargets),
        &mut [
            (generated.as_str(), &mut draft.generated_rust),
            (jit.as_str(), &mut draft.native_x64_jit),
        ],
    );

    let (interpreter, fail) = (
        text(MessageId::VerilogABuildSettingsUseInterpreter),
        text(MessageId::VerilogABuildSettingsFailCompile),
    );
    let mut fallback = usize::from(draft.reject_fallback);
    if rows.choice(
        &text(MessageId::VerilogABuildSettingsFallback),
        &[interpreter.as_str(), fail.as_str()],
        &mut fallback,
    ) {
        draft.reject_fallback = fallback == 1;
        edited = true;
    }

    let remove = text(MessageId::VerilogABuildSettingsRemove);
    let (include_hint, add_include) = (
        text(MessageId::VerilogABuildSettingsIncludeHint),
        text(MessageId::VerilogABuildSettingsAddInclude),
    );
    let entries = draft.include_paths.clone();
    let edit = rows.list(
        &text(MessageId::VerilogABuildSettingsIncludePaths),
        include_id(),
        &entries,
        &mut draft.include_draft,
        ListWords {
            hint: &include_hint,
            add: &add_include,
            remove: &remove,
        },
    );
    edited |= edit.field.changed() || edit.add || edit.remove.is_some();
    if let Some(index) = edit.remove {
        draft.include_paths.remove(index);
    }
    if edit.add {
        let _ = draft.add_include(messages);
    }
    refusal_line(rows, include_id(), refusals.include.as_ref());

    let (define_hint, add_define) = (
        text(MessageId::VerilogABuildSettingsDefineHint),
        text(MessageId::VerilogABuildSettingsAddDefine),
    );
    let entries = draft.defines.iter().map(define_entry).collect::<Vec<_>>();
    let edit = rows.list(
        &text(MessageId::VerilogABuildSettingsDefines),
        define_id(),
        &entries,
        &mut draft.define_draft,
        ListWords {
            hint: &define_hint,
            add: &add_define,
            remove: &remove,
        },
    );
    edited |= edit.field.changed() || edit.add || edit.remove.is_some();
    if let Some(index) = edit.remove {
        draft.defines.remove(index);
    }
    if edit.add {
        let _ = draft.add_define(messages);
    }
    refusal_line(rows, define_id(), refusals.define.as_ref());

    if draft.show_undefines {
        let (undefine_hint, add_undefine) = (
            text(MessageId::VerilogABuildSettingsUndefineHint),
            text(MessageId::VerilogABuildSettingsAddUndefine),
        );
        let entries = draft.undefines.clone();
        let edit = rows.list(
            &text(MessageId::VerilogABuildSettingsUndefines),
            undefine_id(),
            &entries,
            &mut draft.undefine_draft,
            ListWords {
                hint: &undefine_hint,
                add: &add_undefine,
                remove: &remove,
            },
        );
        edited |= edit.field.changed() || edit.add || edit.remove.is_some();
        if let Some(index) = edit.remove {
            draft.undefines.remove(index);
        }
        if edit.add {
            let _ = draft.add_undefine(messages);
        }
        refusal_line(rows, undefine_id(), refusals.undefine.as_ref());
    }

    let [units, convergence, discontinuities, hidden, portability] = [
        MessageId::VerilogABuildSettingsUnitsRanges,
        MessageId::VerilogABuildSettingsConvergence,
        MessageId::VerilogABuildSettingsDiscontinuities,
        MessageId::VerilogABuildSettingsHiddenState,
        MessageId::VerilogABuildSettingsPortability,
    ]
    .map(text);
    let checks = &mut draft.checks;
    edited |= rows.checks(
        &text(MessageId::VerilogABuildSettingsChecks),
        &mut [
            (units.as_str(), &mut checks.units_and_ranges),
            (convergence.as_str(), &mut checks.convergence),
            (discontinuities.as_str(), &mut checks.discontinuities),
            (hidden.as_str(), &mut checks.hidden_state),
            (portability.as_str(), &mut checks.portability),
        ],
    );
    edited
}

//! Rename symbol, Go to definition and Find references for Verilog-A and
//! Automation sources, from the caret of the source editor on screen.
//!
//! Go to definition moves the caret to the definition, opening its file when
//! it lives in another file of the bundle. Find references lists every
//! occurrence in the bottom dock's References page, one row each with its
//! file, line, column and text, and states the count on the status bar.
//! Rename symbol is a command form: the new name, and how many occurrences in
//! how many files it rewrites.
//!
//! These used to be views of a nine-tab "Source navigation and language
//! tools" dialog (document symbols, completion, hover, signature help,
//! definitions, declarations, references, workspace symbols, code actions)
//! that had to be opened to use any of them. The dialog is deleted; what it
//! answered that a reader acts on is here, and Format document formats from
//! the editor.
//!
//! Every answer is bound to the exact closure the index was built from. A
//! rename refuses rather than applying to a newer revision, and says so.

#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used))]
#![cfg_attr(not(test), deny(clippy::panic, clippy::unreachable))]

use egui::{Context, Id};

use crate::ui::widgets::{CommandForm, DialogChoice, FieldLine};
use crate::workbench::app_state::{Model, PlatformState};
use crate::workbench::code::commands::CodeCommand;
use crate::workbench::documents::code_workspace::{
    self, CodeWorkspaceParts, CodeWorkspaceRuntimeState,
};

const RENAME_TITLE: &str = "Rename symbol";
const RENAME_PRIMARY: &str = "Rename";
const NEW_NAME_FIELD: &str = "workbench.rename-symbol.new-name";

/// Go to definition, Find references and Rename symbol from the caret of the
/// source editor on screen, in the `language` bundle it shows, through the
/// project source index. A netlist answers through its own language service
/// instead, which the shell asks.
pub(crate) fn navigate_source_bundle(
    parts: &mut CodeWorkspaceParts<'_>,
    language: crate::state::ProjectSourceLanguage,
    command: CodeCommand,
) -> Result<(), String> {
    let bundle_id = parts
        .model
        .workspace
        .active_source_bundle(language)
        .map(crate::state::ProjectSourceBundle::id)
        .ok_or_else(|| format!("This project owns no {} source yet.", language.label()))?;
    let logical_path =
        code_workspace::active_bundle_path(&parts.model.workspace, parts.code, language)
            .ok_or_else(|| format!("This project owns no {} source yet.", language.label()))?;
    // The caret the editor last reported for this document.
    let caret = parts
        .code
        .source_carets
        .get(&(bundle_id, logical_path.clone()))
        .copied()
        .unwrap_or(0);
    // Built from the exact closure on screen. Rename keeps it as the form's
    // state; the other two borrow it and leave an open form as they found it.
    let previous = parts.code.language_tools.take();
    if let Err(error) = code_workspace::open_language_tools_in_bundle(
        parts,
        language,
        bundle_id,
        &logical_path,
        caret,
    ) {
        parts.code.language_tools = previous;
        return Err(error);
    }
    let caret_name = parts
        .code
        .language_tools
        .as_ref()
        .map(|tools| tools.query.clone())
        .unwrap_or_default();
    if command == CodeCommand::RenameSymbol {
        if caret_name.is_empty() {
            parts.code.language_tools = previous;
            return Err("Place the caret on a name to rename it.".to_owned());
        }
        if let Some(tools) = parts.code.language_tools.as_mut() {
            tools.replacement = caret_name;
        }
        return Ok(());
    }
    let tools = std::mem::replace(&mut parts.code.language_tools, previous)
        .ok_or_else(|| "The source index could not be built.".to_owned())?;
    if command == CodeCommand::FindReferences {
        if caret_name.is_empty() {
            return Err("Place the caret on a name to find its references.".to_owned());
        }
        list_references(parts, &tools.index, &logical_path, caret, &caret_name);
        return Ok(());
    }
    if caret_name.is_empty() {
        return Err("Place the caret on a name to go to its definition.".to_owned());
    }
    let definition = tools
        .index
        .definition_at(&logical_path, caret, &caret_name)
        .ok_or_else(|| format!("No definition of {caret_name} exists in this source bundle."))?;
    if !crate::state::project_source_paths_equal(&definition.logical_path, &logical_path) {
        match language {
            crate::state::ProjectSourceLanguage::VerilogA => {
                parts.code.veriloga.selected_file = Some(code_workspace::VerilogAFileSelection {
                    bundle_id,
                    logical_path: definition.logical_path.clone(),
                });
            }
            crate::state::ProjectSourceLanguage::RSpiceAutomation => {
                parts.code.automation.selected_file = Some(definition.logical_path.clone());
            }
        }
    }
    // The page hands this to the editor it draws next, which is the
    // definition's file.
    parts.wb.outbox.push(
        crate::workbench::framework::requests::SourceEditorRequest::RevealLine(definition.line),
    );
    Ok(())
}

/// Find references: one row per occurrence on the References page of the
/// bottom dock, the page brought forward, and the count on the status bar.
fn list_references(
    parts: &mut CodeWorkspaceParts<'_>,
    index: &code_workspace::LanguageServiceIndex,
    logical_path: &str,
    caret: usize,
    name: &str,
) {
    let references = index.references_at(logical_path, caret, name);
    let files = references
        .iter()
        .map(|reference| crate::state::project_source_path_key(&reference.logical_path))
        .collect::<std::collections::BTreeSet<_>>()
        .len();
    let rows = references
        .iter()
        .map(|reference| crate::workbench::state::SourceReference {
            logical_path: reference.logical_path.clone(),
            line: reference.line,
            column: reference.column,
            text: reference.line_preview.clone(),
        })
        .collect::<Vec<_>>();
    let revision = parts
        .model
        .workspace
        .content
        .project_sources
        .get_bundle(index.bundle_id)
        .map_or(0, |bundle| bundle.revision().get());
    if !rows.is_empty() {
        parts
            .wb
            .show_references(crate::workbench::state::SourceReferences::new(
                name.to_owned(),
                index.bundle_id,
                revision,
                rows,
            ));
    }
    parts.wb.prompt(match references.len() {
        0 => format!("No references to {name} in this source bundle."),
        count => format!(
            "{count} reference{} to {name} in {files} file{}.",
            if count == 1 { "" } else { "s" },
            if files == 1 { "" } else { "s" }
        ),
    });
}

/// Rename symbol: the name under the caret, the new name, and what the
/// rename rewrites, over the Code workspace's state, the model and the
/// platform's state. Enter renames; Esc and Cancel close. Returns whether
/// the reader asked for the rename.
pub(in crate::workbench) fn render_rename_symbol_form(
    code: &mut CodeWorkspaceRuntimeState,
    model: &Model,
    platform: &PlatformState,
    ctx: &Context,
) -> bool {
    let Some(tools) = code.language_tools.clone() else {
        return false;
    };
    let stale = model
        .workspace
        .content
        .project_sources
        .get_bundle(tools.index.bundle_id)
        .is_none_or(|bundle| {
            !tools
                .index
                .is_current(model.workspace.content.project.id(), bundle)
        });
    let blocked = code_workspace::source_file_mutation_block_reason_in(
        model,
        code,
        platform,
        tools.index.language,
    )
    .map(str::to_owned)
    .or_else(|| {
        (!code_workspace::source_bundle_document_is_editable_in(
            model,
            code,
            platform,
            tools.index.language,
            tools.index.bundle_id,
            &tools.active_path,
        ))
        .then(|| {
            format!(
                "{} is read-only, so nothing in it is renamed.",
                tools.active_path
            )
        })
    });
    let new_name = tools.replacement.trim();
    let refusal = if let Some(error) = tools.error.clone() {
        Some(error)
    } else if stale {
        Some("The source changed after Rename opened. Close it and rename again.".to_owned())
    } else if let Some(reason) = blocked {
        Some(reason)
    } else if new_name.is_empty() || new_name == tools.query {
        None
    } else {
        tools
            .index
            .rename_preview_at(&tools.active_path, tools.caret, &tools.query, new_name)
            .err()
    };
    let unchanged = new_name.is_empty() || new_name == tools.query;
    let ready = refusal.is_none() && !unchanged;
    let (occurrences, files) =
        tools
            .index
            .rename_extent_at(&tools.active_path, tools.caret, &tools.query);
    let extent = format!(
        "Renames {}",
        code_workspace::extent_words(occurrences, files)
    );
    let status = match (refusal.as_deref(), unchanged) {
        (Some(refusal), _) => Err(refusal),
        (None, true) => Ok(Some("Enter a new name.")),
        (None, false) => Ok(None),
    };

    let mut replacement = tools.replacement.clone();
    let mut edited = false;
    let choice = CommandForm::new(RENAME_TITLE, RENAME_PRIMARY)
        .primary_enabled(ready)
        .status(status)
        .show(ctx, |rows| {
            let field = Id::new(NEW_NAME_FIELD);
            rows.value_mono("Symbol", &tools.query);
            edited = rows.text("New name", field, &mut replacement, "").changed();
            rows.field_line(Some(field), FieldLine::Derived(&extent));
            Some(field)
        });
    if let Some(open) = code.language_tools.as_mut() {
        open.replacement = replacement;
        if edited {
            open.error = None;
        }
    }
    match choice {
        DialogChoice::Primary => true,
        DialogChoice::Ghost | DialogChoice::Secondary | DialogChoice::Cancelled => {
            code.language_tools = None;
            false
        }
        DialogChoice::None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::app::RSpiceApp;

    /// `resolve_veriloga_compile` was written to gate the compile command and
    /// never attached to it, so the button stayed enabled while a compile was
    /// already running and the surface then refused the request.
    #[test]
    fn compiling_verilog_a_is_refused_while_a_compile_is_already_pending() {
        let mut app = RSpiceApp::test_instance();
        // A Verilog-A cell view open in Design: the place Compile is offered.
        let reference = crate::state::CellViewRef::new("behavioral", "gain", "veriloga");
        let mut view = crate::state::View::new("veriloga", crate::state::ViewType::VerilogA);
        view.metadata
            .insert("veriloga.module".to_owned(), "gain".to_owned());
        let mut cell = crate::state::Cell::new("gain");
        cell.add_view(view);
        let mut library = crate::state::Library::new("behavioral");
        library.add_cell(cell);
        app.state.model.library_manager.add_library(library);
        app.state
            .model
            .workspace
            .content
            .project_sources
            .insert_bundle(
                crate::state::ProjectSourceBundle::try_new(
                    crate::state::ProjectSourceOwner::cell_view(reference.clone()),
                    crate::state::ProjectSourceLanguage::VerilogA,
                    "gain.va",
                    "module gain(p, n); inout p, n; electrical p, n; endmodule\n",
                    [],
                    [],
                )
                .unwrap(),
            )
            .unwrap();
        app.state.open_workspace_view(reference);
        app.state
            .workbench
            .activate(crate::workbench::state::Workspace::Design);
        let enabled_before = app.command_key_enabled(CodeCommand::CompileVerilogA.key());
        assert!(
            enabled_before,
            "a writable project compiles the Verilog-A cell view in front"
        );

        app.state.model.safe_mode.active = true;
        app.state.model.safe_mode.applied.open_project_read_only = true;
        assert!(
            !app.command_key_enabled(CodeCommand::CompileVerilogA.key()),
            "a read-only project cannot compile; it was enabled before as {enabled_before}"
        );
    }

    /// An Automation project whose entry defines `helper` and whose second
    /// file, `shared.py`, uses it; `shared.py` is the environment lock when
    /// `locked`, so a rename may change the entry but not it. The Rename
    /// form is open on `helper` with `assist` typed.
    fn rename_form_over_two_files(locked: bool) -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        app.state.code.code_workspace.page =
            crate::workbench::documents::code_workspace::CodeWorkspacePage::Automation;
        let language = crate::state::ProjectSourceLanguage::RSpiceAutomation;
        let owner = crate::state::ProjectSourceOwner::code_workspace(language);
        let content = &mut app.state.model.workspace.content;
        let bundle = content
            .project_sources
            .bundle_for_owner(&owner)
            .expect("the fixture project owns an Automation bundle");
        let (bundle_id, root) = (bundle.id(), bundle.root().logical_path().to_owned());
        let locks = bundle
            .paths_for_role(crate::state::ProjectSourceRole::AutomationEnvironmentLock)
            .map(str::to_owned)
            .collect::<Vec<_>>();
        let entry = "def helper():
    return 1

helper()
";
        content
            .replace_project_source_bundle_files_transactionally(
                bundle_id,
                [(root.clone(), entry.to_owned())],
            )
            .expect("the entry is rewritten");
        content
            .add_project_source_bundle_file(
                bundle_id,
                &root,
                crate::state::ProjectSourceFile::try_new(
                    "shared.py",
                    "value = helper()
",
                )
                .expect("a valid file"),
            )
            .expect("the second file joins the bundle");
        if locked {
            for lock in &locks {
                content
                    .set_project_source_bundle_non_entry_role(bundle_id, lock, None)
                    .expect("the old lock is released");
            }
            content
                .set_project_source_bundle_non_entry_role(
                    bundle_id,
                    "shared.py",
                    Some(crate::state::ProjectSourceRole::AutomationEnvironmentLock),
                )
                .expect("the second file is the lock");
        }
        code_workspace::open_language_tools_in_bundle(
            &mut app.code_workspace_parts(),
            language,
            bundle_id,
            &root,
            "def h".chars().count(),
        )
        .expect("the language tools open on helper");
        let tools = app
            .state
            .code
            .code_workspace
            .language_tools
            .as_mut()
            .expect("the form is open");
        assert_eq!(tools.query, "helper");
        tools.replacement = "assist".to_owned();
        app
    }

    /// One frame of the frame's dialogs: whether it asked for the next one
    /// straight away, and every label and value it put on the screen.
    fn dialog_frame(
        ctx: &Context,
        app: &mut RSpiceApp,
        events: Vec<egui::Event>,
    ) -> (bool, Vec<String>) {
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(egui::Rect::from_min_size(
                    egui::Pos2::ZERO,
                    egui::vec2(1440.0, 900.0),
                )),
                events,
                ..egui::RawInput::default()
            },
            |ui| app.render_frame_dialogs(ui.ctx()),
        );
        let immediate = output
            .viewport_output
            .get(&egui::ViewportId::ROOT)
            .is_some_and(|viewport| viewport.repaint_delay.is_zero());
        let text = output
            .platform_output
            .accesskit_update
            .into_iter()
            .flat_map(|update| update.nodes)
            .flat_map(|(_, node)| {
                [node.label(), node.value()]
                    .into_iter()
                    .flatten()
                    .map(str::to_owned)
                    .collect::<Vec<_>>()
            })
            .collect();
        (immediate, text)
    }

    /// The rename is carried out after the form has drawn, as the frame's
    /// dialogs answer what they raised, so its outcome is drawn on the frame
    /// after the one that confirmed it. That frame must come with no input
    /// in between: a refused rename states why on the form, and one that
    /// applies has closed it.
    #[test]
    fn a_confirmed_rename_shows_its_outcome_on_the_next_frame_without_input() {
        let enter = egui::Event::Key {
            key: egui::Key::Enter,
            physical_key: Some(egui::Key::Enter),
            pressed: true,
            repeat: false,
            modifiers: egui::Modifiers::NONE,
        };
        for locked in [true, false] {
            let mut app = rename_form_over_two_files(locked);
            let ctx = Context::default();
            ctx.enable_accesskit();
            crate::ui::Theme::default().apply(&ctx);
            for _ in 0..2 {
                let (_, text) = dialog_frame(&ctx, &mut app, Vec::new());
                assert!(
                    text.iter().any(|text| text == RENAME_TITLE),
                    "the form is open"
                );
            }
            let (immediate, _) = dialog_frame(&ctx, &mut app, vec![enter.clone()]);
            assert!(
                immediate,
                "the frame that confirmed the rename asks for the next one"
            );
            let (_, text) = dialog_frame(&ctx, &mut app, Vec::new());
            let open = text.iter().any(|text| text == RENAME_TITLE);
            if locked {
                assert!(open, "a refused rename leaves the form open");
                assert!(
                    text.iter()
                        .any(|text| text.contains("governed read-only source 'shared.py'")),
                    "the refusal is on the form: {text:?}"
                );
            } else {
                assert!(!open, "a rename that applied closed the form");
                assert!(
                    app.state.code.code_workspace.language_tools.is_none(),
                    "the transaction is done"
                );
            }
        }
    }
}

//! Add library > From file…: a model file taken straight onto the plan.
//!
//! A simulator's model-library setup takes any file and a section, so an
//! engineer with a vendor's cards or a corpus deck's `.model` lines does not
//! first go through a library manager. The file is read off the interface
//! thread into a copy of the project's model catalog with the same import the
//! Models workspace runs, published to the project as that workspace
//! publishes it, and bound to the active plan in the same gesture. A file with
//! `.lib` sections binds in the section its import selects (`tt`, or the
//! first), shown on its row, where the row's section control picks another.
//! A desktop read: the path is what resolves a model file's includes, and a
//! browser session reads model files in the Models workspace.
//!
//! Every outcome is stated on the page: the file being read, and a refusal
//! naming the file and why. A refusal also goes where a refused plan edit
//! goes, the plan's status and the log.

use std::sync::{Arc, Mutex};

use egui::{Context, Id, Ui};

use crate::product::ProjectId;
use crate::state::model_library::ModelLibraryManager;
use crate::workbench::framework::feature::Cx;
use crate::workbench::simulate::{SimulateFeature, record_plan_refusal};

use super::super::page_kit::{Tone, note_line};
use super::{ModelBindingAction, apply_model_binding_action};

/// What one read produced: the library the file became and the catalog that
/// holds it, or why it could not be read.
type Read = Result<(String, Box<ModelLibraryManager>), String>;

/// Where this page keeps its one read, between the press that starts it and
/// the frame that finishes it. Session state of this page's picker, never the
/// project's.
#[derive(Clone)]
enum FileImport {
    /// The file is being read; the reader thread posts into `outcome`.
    Reading {
        file: String,
        path: std::path::PathBuf,
        project: ProjectId,
        outcome: Arc<Mutex<Option<Read>>>,
    },
    /// The last file was not added, and why.
    Refused(String),
}

fn slot() -> Id {
    Id::new("simulation.models.from-file")
}

fn current(ctx: &Context) -> Option<FileImport> {
    ctx.data(|data| data.get_temp::<FileImport>(slot()))
}

/// Why From file… cannot be taken now, if it cannot.
pub(super) fn unavailable(ctx: &Context) -> Option<&'static str> {
    matches!(current(ctx), Some(FileImport::Reading { .. }))
        .then_some("A model file is being read.")
}

/// The line under the card's head: the file being read, or why the last one
/// was not added.
pub(super) fn note(ui: &mut Ui) {
    match current(ui.ctx()) {
        Some(FileImport::Reading { file, .. }) => {
            note_line(ui, &format!("Reading {file}\u{2026}"), Tone::Dim);
        }
        Some(FileImport::Refused(message)) => {
            note_line(ui, &message, Tone::Error);
        }
        None => {}
    }
}

/// What the picker offers.
const MODEL_SOURCE: crate::io::file_exchange::FileKind = crate::io::file_exchange::FileKind {
    label: "SPICE model source",
    extensions: crate::state::pdk_config::MODEL_FILE_EXTENSIONS,
    subject: "The model file",
    fallback_name: "model file",
};

/// Ask for a model file and start reading it.
pub(super) fn start_read(ctx: &Context, cx: &mut Cx<'_, SimulateFeature>) {
    let Some(path) = crate::io::file_exchange::pick_source_path(MODEL_SOURCE) else {
        return;
    };
    let file = path.file_name().map_or_else(
        || path.display().to_string(),
        |name| name.to_string_lossy().into_owned(),
    );
    let outcome = Arc::new(Mutex::new(None));
    let posted = Arc::clone(&outcome);
    let mut candidate = cx.model.model_library_manager.clone();
    let source = path.clone();
    let repaint = ctx.clone();
    let reader = std::thread::Builder::new()
        .name("model-file-import".to_owned())
        .spawn(move || {
            let read = candidate
                .load_library_file(&source, None)
                .map(|library| (library, Box::new(candidate)));
            if let Ok(mut slot) = posted.lock() {
                *slot = Some(read);
            }
            repaint.request_repaint();
        });
    let state = match reader {
        Ok(_) => FileImport::Reading {
            file,
            path,
            project: cx.model.workspace.content.project.id(),
            outcome,
        },
        Err(error) => FileImport::Refused(format!("{file} was not added: {error}")),
    };
    ctx.data_mut(|data| data.insert_temp(slot(), state));
}

/// Add the file a finished read produced, or say why it was not added.
pub(super) fn finish_read(ctx: &Context, cx: &mut Cx<'_, SimulateFeature>) {
    let Some(FileImport::Reading {
        file,
        path,
        project,
        outcome,
    }) = current(ctx)
    else {
        return;
    };
    let Some(read) = outcome.lock().ok().and_then(|mut slot| slot.take()) else {
        return;
    };
    ctx.data_mut(|data| data.remove::<FileImport>(slot()));
    let added = if project == cx.model.workspace.content.project.id() {
        read.and_then(|(library, candidate)| add(cx, &path, &library, *candidate))
    } else {
        Err("the project changed while it was read".to_owned())
    };
    if let Err(error) = added {
        let message = format!("{file} was not added: {error}");
        ctx.data_mut(|data| data.insert_temp(slot(), FileImport::Refused(message.clone())));
        record_plan_refusal(cx, message);
    }
}

/// Publish `library` from `candidate` to the project, unless the project
/// already holds it from this file, and bind it to the plan.
fn add(
    cx: &mut Cx<'_, SimulateFeature>,
    path: &std::path::Path,
    library: &str,
    candidate: ModelLibraryManager,
) -> Result<(), String> {
    if cx
        .model
        .sim_setup
        .model_bindings
        .iter()
        .any(|binding| binding.library_name.eq_ignore_ascii_case(library))
    {
        return Err(format!("its library {library} is already on this plan"));
    }
    if cx
        .model
        .model_library_manager
        .get_library(library)
        .is_none()
    {
        crate::workbench::app_state::design_history::publish_model_library_candidate(
            cx.model,
            candidate,
            library,
            format!("import model source {}", path.display()),
        )?;
    }
    apply_model_binding_action(cx, ModelBindingAction::Attach(library.to_owned()));
    let bound = cx
        .model
        .sim_setup
        .model_bindings
        .iter()
        .any(|binding| binding.library_name.eq_ignore_ascii_case(library));
    if bound {
        Ok(())
    } else {
        Err(format!(
            "its library {library} was imported, but the plan refused it: {}",
            cx.model.analysis_lifecycle_status.message()
        ))
    }
}

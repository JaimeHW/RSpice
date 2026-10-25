//! Autosave checkpoints — interrupted-session recovery that never touches your file
//! using the standard confirmation dialog.
//!
//! While the schematic is dirty and has a path, a timer writes the same
//! serialization the real save uses to a `.autosave` sibling. A clean save
//! retires only a checkpoint demonstrably owned by this process. Opening a
//! file offers an eligible bound checkpoint before loading either snapshot.

#![cfg(not(target_arch = "wasm32"))]

use std::path::PathBuf;

use egui::Context;
use rspice_ui_kit::theme::{self, FontWeight};
use rspice_ui_kit::tokens::{self, Tokens};
use rspice_ui_kit::widgets::{Dialog, DialogChoice, DialogSize};

use crate::diagnostics::{LogSeverity, LogSource};
use crate::state::SchematicState;
use crate::workbench::app_state::Model;
use crate::workbench::lifecycle::project_lifecycle::PlatformState;

/// Write a checkpoint when the interval has elapsed on a dirty,
/// path-bearing schematic. Called once per frame; cheap when idle. `last` is
/// when the timer was armed or the last checkpoint written.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn autosave_tick(
    platform: &mut PlatformState,
    model: &mut Model,
    last: &mut Option<std::time::Instant>,
    ctx: &Context,
) {
    let minutes = platform.autosave_minutes;
    if minutes == 0 || !model.schematic.session.is_dirty || model.schematic.session.read_only {
        *last = None;
        return;
    }
    let Some(path) = model.schematic.session.current_file.clone() else {
        // Nothing to anchor a checkpoint to — save once and autosave
        // takes over.
        return;
    };

    let now = std::time::Instant::now();
    let interval = std::time::Duration::from_secs(u64::from(minutes) * 60);
    let Some(armed) = *last else {
        // First dirty frame arms the timer; the checkpoint follows one
        // full interval later.
        *last = Some(now);
        ctx.request_repaint_after(interval);
        return;
    };
    let elapsed = now.duration_since(armed);
    if elapsed < interval {
        // Idle windows produce no frames — keep one queued so the
        // checkpoint lands on time without an interaction.
        ctx.request_repaint_after(interval - elapsed);
        return;
    }

    // A checkpoint that was written is not an event: only its failure is,
    // and only when it starts. A failure that persists would otherwise
    // raise an error at every interval, and an error stays until answered;
    // the recovery is recorded instead, so the Console shows the span.
    let was_failing = platform.autosave_failing;
    let written =
        crate::workbench::lifecycle::recovery_checkpoint::write_checkpoint(&path, &model.schematic);
    platform.autosave_failing = written.is_err();
    match written {
        Err(e) if !was_failing => model.report_failure("Autosave failed", e.to_string()),
        Ok(_) if was_failing => {
            model.record(
                LogSeverity::Info,
                LogSource::Studio,
                "Autosave is writing checkpoints again.",
            );
        }
        Err(_) | Ok(_) => {}
    }
    *last = Some(now);
}

/// What the reader chose in the restore dialog, with the checkpoint work the
/// choice needed already done. Loading it, and then dropping the pending
/// restore, is the caller's.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) enum AutosaveRestore {
    /// The verified checkpoint, as unsaved changes over the file's identity,
    /// and the line to record once it is loaded.
    Checkpoint {
        path: PathBuf,
        schematic: Box<SchematicState>,
        note: String,
    },
    /// The saved file, and the line to record once it is opened.
    SavedFile { path: PathBuf, note: Option<String> },
}

/// Show the restore decision an open deferred: Restore reads the checkpoint,
/// Discard deletes it, Esc keeps it. A checkpoint that could not be read or
/// deleted is recorded here and the decision stays pending.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn restore_dialog(
    model: &mut Model,
    platform: &PlatformState,
    ctx: &Context,
) -> Option<AutosaveRestore> {
    let candidate = platform.pending_autosave_restore.clone()?;
    let path = candidate.source.clone();
    let checkpoint = candidate.checkpoint.clone();
    let file_name = path
        .file_name()
        .map(|name| name.to_string_lossy().to_string())
        .unwrap_or_else(|| path.display().to_string());
    let age = checkpoint_age(&checkpoint);
    let can_discard = candidate.can_discard();

    let choice = Dialog::new("Restore autosave?", "Restore")
        .description(
            "Review an interrupted autosave checkpoint and choose whether to restore it, keep it, or discard it when safe.",
        )
        .size(DialogSize::Transaction)
        .ghost(if can_discard {
            "Discard"
        } else {
            "Keep checkpoint"
        })
        .hint("Esc opens the file and keeps the checkpoint")
        .show(ctx, |ui| {
            let c = Tokens::get(ui.ctx()).color;
            ui.label(
                egui::RichText::new(format!(
                    "An interrupted autosave of '{file_name}' from {age} is available. \
                     Restore its exact reviewed bytes, or open the saved file and keep \
                     the checkpoint."
                ))
                .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                .color(c.text_dim),
            );
        });

    match choice {
        DialogChoice::None | DialogChoice::Secondary => None,
        DialogChoice::Primary => {
            let restored = crate::workbench::lifecycle::recovery_checkpoint::read_bound_checkpoint(
                &path,
                &checkpoint,
                &candidate.binding,
            )
            .and_then(|bytes| {
                let text = std::str::from_utf8(&bytes)
                    .map_err(|error| format!("checkpoint is not UTF-8 JSON: {error}"))?;
                crate::io::schematic_io::load_schematic_text(text, Some(&path))
                    .map_err(|error| error.to_string())
            });
            match restored {
                Ok(mut schematic) => {
                    schematic.session.current_file = Some(path.clone());
                    schematic.session.is_dirty = true;
                    Some(AutosaveRestore::Checkpoint {
                        path,
                        schematic: Box::new(schematic),
                        note: format!("Restored the autosave from {age}; save to keep it."),
                    })
                }
                Err(error) => {
                    model.record(
                        LogSeverity::Error,
                        LogSource::Studio,
                        format!(
                            "Autosave restore failed because the checkpoint changed or could not be verified: {error}"
                        ),
                    );
                    None
                }
            }
        }
        DialogChoice::Ghost => {
            if can_discard {
                match crate::workbench::lifecycle::recovery_checkpoint::discard_bound_checkpoint(
                    &path,
                    &checkpoint,
                    &candidate.binding,
                ) {
                    Ok(()) => Some(AutosaveRestore::SavedFile { path, note: None }),
                    Err(error) => {
                        model.record(
                            LogSeverity::Error,
                            LogSource::Studio,
                            format!(
                                "Autosave discard failed; the checkpoint was not deleted: {error}"
                            ),
                        );
                        None
                    }
                }
            } else {
                Some(AutosaveRestore::SavedFile {
                    path,
                    note: Some(format!(
                        "Kept the autosave for review in Recovery: {}",
                        checkpoint.display()
                    )),
                })
            }
        }
        DialogChoice::Cancelled => Some(AutosaveRestore::SavedFile {
            path,
            note: Some(format!("Kept the autosave at {}.", checkpoint.display())),
        }),
    }
}

/// Coarse human age of a checkpoint file ("3 min ago"), from its mtime.
#[cfg(not(target_arch = "wasm32"))]
fn checkpoint_age(checkpoint: &std::path::Path) -> String {
    let elapsed = std::fs::metadata(checkpoint)
        .and_then(|meta| meta.modified())
        .ok()
        .and_then(|modified| modified.elapsed().ok());
    match elapsed {
        None => "an earlier session".to_owned(),
        Some(age) => crate::time_compat::relative_age(age),
    }
}

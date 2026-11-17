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
use crate::workbench::lifecycle::project_lifecycle::PlatformState;
use rspice_app_state::workbench::app_state::model::Model;

/// Write a checkpoint when the interval has elapsed on a dirty,
/// path-bearing schematic. Called once per frame; cheap when idle. `last` is
/// when the timer was armed or the last checkpoint written.
#[cfg(not(target_arch = "wasm32"))]
pub fn autosave_tick(
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
                "Autosave is working again.",
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
pub enum AutosaveRestore {
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
pub fn restore_dialog(
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
    let modified = candidate.binding.identity.modified_unix_ms;
    let age = autosave_age(modified);
    let time = autosave_time(modified);
    let can_discard = candidate.can_discard();

    let choice = Dialog::new("Restore autosave?", "Restore")
        .description(
            "An interrupted session left an autosave of this drawing. Restore it, keep it, or discard it.",
        )
        .size(DialogSize::Transaction)
        .ghost(if can_discard {
            "Discard"
        } else {
            "Keep autosave"
        })
        .hint("Esc opens the saved file and keeps the autosave.")
        .show(ctx, |ui| {
            let c = Tokens::get(ui.ctx()).color;
            ui.label(
                egui::RichText::new(restore_sentence(&file_name, &time, &age))
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
                    .map_err(|error| format!("the autosave is not UTF-8 JSON: {error}"))?;
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
                            "The autosave could not be restored: it changed or could not be read: {error}"
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
                                "The autosave could not be discarded and is still on disk: {error}"
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

/// What the dialog says the autosave is: the drawing, when it was saved on
/// the reader's clock, and how long ago.
fn restore_sentence(file_name: &str, time: &str, age: &str) -> String {
    format!(
        "Restore the autosaved drawing from {time} ({age}), or open the saved \
         '{file_name}' and keep the autosave."
    )
}

/// When the autosave was written, on the reader's clock: an autosave is this
/// computer's own record of its interrupted work.
fn autosave_time(modified_unix_ms: u64) -> String {
    (modified_unix_ms != 0)
        .then(|| crate::time_compat::local_date_time(modified_unix_ms))
        .flatten()
        .map_or_else(
            || "an earlier session".to_owned(),
            |stamp| {
                format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}",
                    stamp.year(),
                    u8::from(stamp.month()),
                    stamp.day(),
                    stamp.hour(),
                    stamp.minute()
                )
            },
        )
}

/// Coarse human age of an autosave ("3 min ago"), from the modification
/// time read once when it was found and offered: the dialog asks the clock,
/// never the file.
#[cfg(not(target_arch = "wasm32"))]
fn autosave_age(modified_unix_ms: u64) -> String {
    let elapsed = crate::time_compat::checked_unix_time_ms()
        .ok()
        .filter(|_| modified_unix_ms != 0)
        .and_then(|now| now.checked_sub(modified_unix_ms));
    match elapsed {
        None => "an earlier session".to_owned(),
        Some(age) => crate::time_compat::relative_age(std::time::Duration::from_millis(age)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The dialog names what it restores and when, in the reader's words:
    /// the autosaved drawing, at a time on the clock.
    #[test]
    fn the_restore_sentence_names_the_autosaved_drawing_and_its_time() {
        let sentence = restore_sentence(
            "amp.rsch",
            &autosave_time(1_760_000_000_000),
            &autosave_age(1_760_000_000_000),
        );
        assert!(
            sentence.starts_with("Restore the autosaved drawing from 20"),
            "{sentence}"
        );
        assert!(sentence.contains("'amp.rsch'"), "{sentence}");
        for internal in ["reviewed bytes", "checkpoint"] {
            assert!(!sentence.contains(internal), "{sentence}");
        }
        assert_eq!(autosave_time(0), "an earlier session");
    }
}

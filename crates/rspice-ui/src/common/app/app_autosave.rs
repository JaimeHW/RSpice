//! Autosave checkpoints — interrupted-session recovery that never touches your file
//! using the standard confirmation dialog.
//!
//! While the schematic is dirty and has a path, a timer writes the same
//! serialization the real save uses to a `.autosave` sibling. A clean save
//! retires only a checkpoint demonstrably owned by this process. Opening a
//! file offers an eligible bound checkpoint before loading either snapshot.

#![cfg(not(target_arch = "wasm32"))]

use egui::Context;

use super::{ConsoleMessage, RSpiceApp};
use crate::common::file_workflow;
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};

impl RSpiceApp {
    /// Write a checkpoint when the interval has elapsed on a dirty,
    /// path-bearing schematic. Called once per frame; cheap when idle.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn autosave_tick(&mut self, ctx: &Context) {
        let minutes = self.state.ui.autosave_minutes;
        if minutes == 0 || !self.state.schematic.is_dirty || self.state.schematic.read_only {
            self.autosave_last = None;
            return;
        }
        let Some(path) = self.state.schematic.current_file.clone() else {
            // Nothing to anchor a checkpoint to — save once and autosave
            // takes over.
            return;
        };

        let now = std::time::Instant::now();
        let interval = std::time::Duration::from_secs(u64::from(minutes) * 60);
        let Some(armed) = self.autosave_last else {
            // First dirty frame arms the timer; the checkpoint follows one
            // full interval later.
            self.autosave_last = Some(now);
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

        match crate::common::recovery_checkpoint::write_checkpoint(&path, &self.state.schematic) {
            Ok(checkpoint) => {
                self.state.log_buffer.log(
                    crate::panels::LogSeverity::Debug,
                    crate::panels::LogSource::System,
                    format!("Autosaved checkpoint: {}", checkpoint.display()),
                    None,
                );
            }
            Err(e) => {
                self.state
                    .push_user_message(ConsoleMessage::warning(format!("Autosave failed: {e}")));
            }
        }
        self.autosave_last = Some(now);
    }

    /// The restore decision an open deferred: Restore loads the checkpoint
    /// as unsaved changes over the file's identity; Discard deletes it and
    /// opens the file; Esc opens the file and keeps the checkpoint.
    #[cfg(not(target_arch = "wasm32"))]
    pub(super) fn process_autosave_restore_dialog(&mut self, ctx: &Context) {
        let Some(candidate) = self.state.dialogs.pending_autosave_restore.clone() else {
            return;
        };
        let path = candidate.source.clone();
        let checkpoint = candidate.checkpoint.clone();
        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        let age = checkpoint_age(&checkpoint);
        let can_discard = candidate.can_discard();

        let choice = Dialog::new("File", "Restore autosave?", "Restore")
            .size(DialogSize::Sm)
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

        let io = self.file_workflow_io.as_ref();
        match choice {
            DialogChoice::None => {}
            DialogChoice::Primary => {
                let restored = crate::common::recovery_checkpoint::read_bound_checkpoint(
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
                        schematic.current_file = Some(path.clone());
                        schematic.is_dirty = true;
                        file_workflow::apply_loaded_schematic(
                            &mut self.state,
                            schematic,
                            file_workflow::SchematicLoadOrigin::PersistentPath(&path),
                        );
                        self.state.push_user_message(ConsoleMessage::info(format!(
                            "Restored exact autosave bytes from {age}; save to keep them"
                        )));
                        self.state.dialogs.pending_autosave_restore = None;
                    }
                    Err(error) => self.state.push_user_message(ConsoleMessage::warning(format!(
                        "Autosave restore was blocked because the checkpoint changed or could not be verified: {error}"
                    ))),
                }
            }
            DialogChoice::Ghost => {
                if can_discard {
                    match crate::common::recovery_checkpoint::discard_bound_checkpoint(
                        &path,
                        &checkpoint,
                        &candidate.binding,
                    ) {
                        Ok(()) => {
                            file_workflow::load_schematic_bypassing_autosave(
                                &mut self.state,
                                &path,
                                io,
                            );
                            self.state.dialogs.pending_autosave_restore = None;
                        }
                        Err(error) => self.state.push_user_message(ConsoleMessage::warning(
                            format!(
                                "Autosave discard was blocked; no checkpoint bytes were deleted: {error}"
                            ),
                        )),
                    }
                } else {
                    file_workflow::load_schematic_bypassing_autosave(&mut self.state, &path, io);
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Legacy autosave kept for non-destructive Recovery review: {}",
                        checkpoint.display()
                    )));
                    self.state.dialogs.pending_autosave_restore = None;
                }
            }
            DialogChoice::Cancelled => {
                file_workflow::load_schematic_bypassing_autosave(&mut self.state, &path, io);
                self.state.push_user_message(ConsoleMessage::info(format!(
                    "Autosave kept: {}",
                    checkpoint.display()
                )));
                self.state.dialogs.pending_autosave_restore = None;
            }
            DialogChoice::Secondary => {}
        }
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
        Some(age) if age.as_secs() < 60 => "moments ago".to_owned(),
        Some(age) if age.as_secs() < 3600 => format!("{} min ago", age.as_secs() / 60),
        Some(age) if age.as_secs() < 86_400 => format!("{} h ago", age.as_secs() / 3600),
        Some(age) => format!("{} days ago", age.as_secs() / 86_400),
    }
}

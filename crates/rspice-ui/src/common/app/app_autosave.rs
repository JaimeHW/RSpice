//! Autosave checkpoints — crash recovery that never touches your file
//! using the standard confirmation dialog.
//!
//! While the schematic is dirty and has a path, a timer writes the same
//! serialization the real save uses to a `.autosave` sibling. A clean save
//! deletes the checkpoint (the file is the truth again). Opening a file
//! shadowed by a newer checkpoint asks before loading either.

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
        let minutes = self.state.shell.autosave_minutes;
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

        let checkpoint = file_workflow::autosave_checkpoint_path(&path);
        match self
            .file_workflow_io
            .save_schematic(&self.state.schematic, &checkpoint)
        {
            Ok(()) => {
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
    pub(super) fn process_autosave_restore_dialog(&mut self, ctx: &Context) {
        let Some((path, checkpoint)) = self.state.dialogs.pending_autosave_restore.clone() else {
            return;
        };

        let file_name = path
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_else(|| path.display().to_string());
        let age = checkpoint_age(&checkpoint);

        let choice = Dialog::new("File", "Restore autosave?", "Restore")
            .size(DialogSize::Sm)
            .ghost("Discard")
            .hint("Esc opens the file and keeps the checkpoint")
            .show(ctx, |ui| {
                let c = Tokens::get(ui.ctx()).color;
                ui.label(
                    egui::RichText::new(format!(
                        "An autosave of '{file_name}' from {age} is newer than the \
                         saved file. Restore the autosaved changes, or discard them \
                         and open the file."
                    ))
                    .font(theme::sans(tokens::FS_1, FontWeight::Regular))
                    .color(c.text_dim),
                );
            });

        let io = self.file_workflow_io.as_ref();
        match choice {
            DialogChoice::None => {}
            DialogChoice::Primary => {
                // The checkpoint's content, the file's identity: restoring
                // leaves the schematic dirty so an explicit save commits it.
                if file_workflow::load_schematic_bypassing_autosave(
                    &mut self.state,
                    &checkpoint,
                    io,
                ) {
                    self.state.schematic.current_file = Some(path.clone());
                    self.state.schematic.is_dirty = true;
                    self.state.push_user_message(ConsoleMessage::info(format!(
                        "Restored autosave from {age} — save to keep it"
                    )));
                }
                self.state.dialogs.pending_autosave_restore = None;
            }
            DialogChoice::Ghost => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let _ = std::fs::remove_file(&checkpoint);
                }
                file_workflow::load_schematic_bypassing_autosave(&mut self.state, &path, io);
                self.state.dialogs.pending_autosave_restore = None;
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

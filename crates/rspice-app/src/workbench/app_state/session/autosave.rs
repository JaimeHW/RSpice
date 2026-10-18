//! Autosave checkpoints — interrupted-session recovery that never touches your file
//! using the standard confirmation dialog.
//!
//! While the schematic is dirty and has a path, a timer writes the same
//! serialization the real save uses to a `.autosave` sibling. A clean save
//! retires only a checkpoint demonstrably owned by this process. Opening a
//! file offers an eligible bound checkpoint before loading either snapshot.

#![cfg(not(target_arch = "wasm32"))]

use egui::Context;

use crate::diagnostics::{LogSeverity, LogSource};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize};
use crate::workbench::app::RSpiceApp;
use crate::workbench::workflows::file_workflow;

impl RSpiceApp {
    /// Write a checkpoint when the interval has elapsed on a dirty,
    /// path-bearing schematic. Called once per frame; cheap when idle.
    #[cfg(not(target_arch = "wasm32"))]
    pub(in crate::workbench) fn autosave_tick(&mut self, ctx: &Context) {
        let minutes = self.state.ui.autosave_minutes;
        if minutes == 0
            || !self.state.schematic.session.is_dirty
            || self.state.schematic.session.read_only
        {
            self.autosave_last = None;
            return;
        }
        let Some(path) = self.state.schematic.session.current_file.clone() else {
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

        // A checkpoint that was written is not an event: only its failure is.
        if let Err(e) = crate::workbench::lifecycle::recovery_checkpoint::write_checkpoint(
            &path,
            &self.state.schematic,
        ) {
            self.state.report_failure("Autosave failed", e.to_string());
        }
        self.autosave_last = Some(now);
    }

    /// The restore decision an open deferred: Restore loads the checkpoint
    /// as unsaved changes over the file's identity; Discard deletes it and
    /// opens the file; Esc opens the file and keeps the checkpoint.
    #[cfg(not(target_arch = "wasm32"))]
    pub(in crate::workbench) fn process_autosave_restore_dialog(&mut self, ctx: &Context) {
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

        let io = self.file_workflow_io.as_ref();
        match choice {
            DialogChoice::None => {}
            DialogChoice::Primary => {
                let restored =
                    crate::workbench::lifecycle::recovery_checkpoint::read_bound_checkpoint(
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
                        file_workflow::apply_loaded_schematic(
                            &mut self.state,
                            schematic,
                            file_workflow::SchematicLoadOrigin::PersistentPath(&path),
                        );
                        self.state.record(
                            LogSeverity::Info,
                            LogSource::Studio,
                            format!("Restored the autosave from {age}; save to keep it."),
                        );
                        self.state.dialogs.pending_autosave_restore = None;
                    }
                    Err(error) => {
                        self.state.record(
                            LogSeverity::Error,
                            LogSource::Studio,
                            format!(
                                "Autosave restore failed because the checkpoint changed or could not be verified: {error}"
                            ),
                        );
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
                        Ok(()) => {
                            file_workflow::load_schematic_bypassing_autosave(
                                &mut self.state,
                                &path,
                                io,
                            );
                            self.state.dialogs.pending_autosave_restore = None;
                        }
                        Err(error) => {
                            self.state.record(
                                LogSeverity::Error,
                                LogSource::Studio,
                                format!(
                                    "Autosave discard failed; the checkpoint was not deleted: {error}"
                                ),
                            );
                        }
                    }
                } else {
                    file_workflow::load_schematic_bypassing_autosave(&mut self.state, &path, io);
                    self.state.record(
                        LogSeverity::Info,
                        LogSource::Studio,
                        format!(
                            "Kept the autosave for review in Recovery: {}",
                            checkpoint.display()
                        ),
                    );
                    self.state.dialogs.pending_autosave_restore = None;
                }
            }
            DialogChoice::Cancelled => {
                file_workflow::load_schematic_bypassing_autosave(&mut self.state, &path, io);
                self.state.record(
                    LogSeverity::Info,
                    LogSource::Studio,
                    format!("Kept the autosave at {}.", checkpoint.display()),
                );
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
        Some(age) => crate::time_compat::relative_age(age),
    }
}

#[cfg(test)]
mod tests {
    use crate::workbench::app::RSpiceApp;

    /// A checkpoint that could not be written is said when it happens: the
    /// user is relying on it, and a bell record is found too late.
    #[test]
    fn a_checkpoint_that_cannot_be_written_is_reported() {
        // A file where the checkpoint's folder has to be.
        let blocker = std::env::temp_dir().join(format!(
            "rspice-autosave-blocker-{}-{}",
            std::process::id(),
            crate::time_compat::unix_epoch().as_nanos()
        ));
        std::fs::write(&blocker, b"not a folder").expect("write the blocking file");

        let mut app = RSpiceApp::test_instance();
        app.state.ui.autosave_minutes = 1;
        app.state.schematic.session.is_dirty = true;
        app.state.schematic.session.current_file = Some(blocker.join("design.rspice"));
        app.autosave_last =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(2 * 60));
        app.autosave_tick(&egui::Context::default());
        let _ = std::fs::remove_file(&blocker);

        let activity = app.state.ui.toasts.activity();
        assert_eq!(activity.len(), 1);
        assert_eq!(activity[0].title(), "Autosave failed");
        assert_eq!(activity[0].kind(), crate::ui::widgets::ToastKind::Error);
    }
}

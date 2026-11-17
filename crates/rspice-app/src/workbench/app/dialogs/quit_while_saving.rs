//! What quitting asks while a save is still copying the results it keeps to another volume:
//! wait for the save, and quit once it has written the project, or quit without saving.

use egui::Context;

use crate::workbench::app::RSpiceApp;
use crate::workbench::app_state::AppState;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::ui::theme::{self, FontWeight};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::ui::tokens::{self, Tokens};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::ui::widgets::{Dialog, DialogChoice, DialogSize};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::workbench::app::dialogs::confirmation_state::ConfirmationAction;

impl AppState {
    /// Whether a save is copying its results to another volume now.
    pub(crate) fn results_copy_running(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.platform
                .results_save
                .as_ref()
                .is_some_and(|save| save.placement.copy_progress().is_some())
        }
        #[cfg(target_arch = "wasm32")]
        false
    }
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_quit_while_saving_dialog(&mut self, ctx: &Context) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // A save that ended while the question was open leaves nothing to wait for.
            let Some(save) = self.state.platform.results_save.as_ref() else {
                self.state.shell.quit_while_saving = false;
                return;
            };
            let transaction = save.transaction;
            let title = format!(
                "Results are still being saved to {}",
                save.placement.volume()
            );
            let sentence = "RSpice quits once the save has written the project. Quitting now \
                            cancels the save: the project file stays as it was, and its unsaved \
                            changes are lost.";
            let choice = Dialog::new(&title, "Wait")
                .description(sentence)
                .size(DialogSize::Confirmation)
                .secondary("Quit without saving")
                .secondary_leading()
                .show(ctx, |ui| {
                    let t = Tokens::get(ui.ctx());
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(sentence)
                                .font(theme::sans(tokens::FS_2, FontWeight::Regular))
                                .color(t.color.text),
                        )
                        .wrap(),
                    );
                });
            match choice {
                DialogChoice::Primary => {
                    self.state.shell.quit_while_saving = false;
                    self.state
                        .workbench
                        .confirmation_dialog
                        .await_canonical_save(
                            transaction,
                            ConfirmationAction::Exit,
                            None,
                            None,
                            None,
                        );
                }
                DialogChoice::Secondary => {
                    self.state.shell.quit_while_saving = false;
                    self.quit_without_saving_results();
                }
                DialogChoice::Ghost | DialogChoice::Cancelled => {
                    self.state.shell.quit_while_saving = false;
                }
                DialogChoice::None => {}
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = ctx;
            self.state.shell.quit_while_saving = false;
        }
    }

    /// Abandon the save copying its results, leaving nothing of the copy and the project file
    /// as it was, then quit as "Don't save" does.
    #[cfg(not(target_arch = "wasm32"))]
    fn quit_without_saving_results(&mut self) {
        if let Some(save) = self.state.platform.results_save.take() {
            // The copy stops between blocks and deletes what it wrote.
            save.placement
                .cancel_and_wait(std::time::Duration::from_secs(5));
            rspice_app_services::workbench::lifecycle::project_lifecycle::cancel_transaction_if(
                &mut self.state.platform,
                save.transaction,
            );
            let _ = self
                .state
                .workbench
                .confirmation_dialog
                .take_canonical_save(save.transaction);
        }
        self.state.models.clear_project_model_editor();
        self.execute_pending_action(ConfirmationAction::Exit, None, None, None);
    }
}

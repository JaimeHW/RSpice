//! Closing keeps the session's results for the next start: the results store segments its runs
//! read go into this instance's session folder off the frame, and the window closes once they
//! are there. A link or clone takes a moment and asks nothing; a copy to another volume shows
//! its progress and lets the reader close without those runs, or stay.

use egui::Context;

use crate::workbench::app::RSpiceApp;
#[cfg(not(target_arch = "wasm32"))]
use crate::workbench::app_state::AppState;
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_services::io::result_segments::{self, SessionResultsAtClose};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::ui::theme::{self, FontWeight};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::ui::tokens::{self, Tokens};
#[cfg(not(target_arch = "wasm32"))]
use rspice_app_workbench::ui::widgets::{Dialog, DialogChoice, DialogSize};

#[cfg(not(target_arch = "wasm32"))]
impl AppState {
    /// Closing was asked for: whether it waits while the session's results go into its folder.
    pub(crate) fn hold_close_for_session_results(&mut self) -> bool {
        let segments = result_segments::file_segments(
            &self
                .platform
                .project_lifecycle
                .result_cache
                .capture(&self.model.simulation),
            &self.model.workspace.content.visualization_documents,
        );
        let held = result_segments::place_session_segments_for_close(segments)
            != SessionResultsAtClose::Kept;
        self.shell.closing_for_session_results |= held;
        held
    }
}

impl RSpiceApp {
    /// Close once the session's results are kept, and keep frames coming until then.
    pub(in crate::workbench) fn close_once_session_results_are_kept(&mut self, ctx: &Context) {
        #[cfg(not(target_arch = "wasm32"))]
        if self.state.shell.closing_for_session_results {
            if result_segments::session_results_at_close() == SessionResultsAtClose::Kept {
                self.state.shell.closing_for_session_results = false;
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                ctx.request_repaint_after(std::time::Duration::from_millis(100));
            }
        }
        #[cfg(target_arch = "wasm32")]
        let _ = ctx;
    }

    pub(in crate::workbench) fn render_close_while_keeping_results_dialog(
        &mut self,
        ctx: &Context,
    ) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let SessionResultsAtClose::Copying {
                volume,
                copied,
                total,
            } = result_segments::session_results_at_close()
            else {
                return;
            };
            let title = format!("Copying this session's results to {volume}");
            let sentence = "RSpice closes once the results of its runs are copied, so the next \
                            start shows them. Closing now starts the next session without these \
                            runs; saved projects keep their own results.";
            let choice = Dialog::new(&title, "Close without these runs")
                .description(sentence)
                .size(DialogSize::Confirmation)
                .ghost("Keep RSpice open")
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
                    ui.add_space(tokens::SP_2);
                    let fraction = if total == 0 {
                        0.0
                    } else {
                        copied as f32 / total as f32
                    };
                    ui.add(egui::ProgressBar::new(fraction).text(format!(
                        "{} of {}",
                        rspice_simulation_contract::run_set::format_bytes(copied),
                        rspice_simulation_contract::run_set::format_bytes(total)
                    )));
                });
            match choice {
                DialogChoice::Primary => {
                    result_segments::close_without_session_results();
                }
                DialogChoice::Ghost | DialogChoice::Cancelled => {
                    self.state.shell.closing_for_session_results = false;
                    result_segments::stop_placing_session_segments();
                }
                DialogChoice::Secondary | DialogChoice::None => {}
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = ctx;
            self.state.shell.closing_for_session_results = false;
        }
    }
}

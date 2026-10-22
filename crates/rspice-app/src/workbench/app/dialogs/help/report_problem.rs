//! Report a problem: what support needs to reproduce one, in one dialog.
//!
//! The build, the platform and the graphics adapter it renders with, the file
//! the session log is written to, the support bundle that carries the log,
//! and the address to send it to. It replaced the Help Center's Diagnostics
//! and Support bundle pages, which each held half of this, said the log file
//! was "not configured" while the product wrote one, and named no address.

use std::sync::OnceLock;

use egui::Context;

use super::{VERSION_LINE, export_support_bundle, fact_grid};
use crate::ui::widgets::{Dialog, DialogChoice, DialogHintTone, DialogSize};
use crate::workbench::app::{HelpCenterPage, RSpiceApp};

/// Where a problem report goes.
pub(crate) const SUPPORT_ADDRESS: &str = "hello@rspice.app";

/// The adapter the window renders with, recorded once when it is created.
static GRAPHICS_ADAPTER: OnceLock<String> = OnceLock::new();

/// Record the graphics adapter the window renders with, when it renders
/// through wgpu, for the dialog to report. The first record wins: the window
/// keeps one adapter for its lifetime.
pub(crate) fn record_graphics_adapter(render_state: Option<&eframe::egui_wgpu::RenderState>) {
    if let Some(render_state) = render_state {
        let info = render_state.adapter.get_info();
        let _ = GRAPHICS_ADAPTER.set(graphics_summary(&info.name, info.backend, info.device_type));
    }
}

fn graphics_summary(
    name: &str,
    backend: eframe::egui_wgpu::wgpu::Backend,
    device_type: eframe::egui_wgpu::wgpu::DeviceType,
) -> String {
    use eframe::egui_wgpu::wgpu::{Backend, DeviceType};

    let backend = match backend {
        Backend::Vulkan => "Vulkan",
        Backend::Metal => "Metal",
        Backend::Dx12 => "Direct3D 12",
        Backend::Gl => "OpenGL",
        Backend::BrowserWebGpu => "WebGPU",
        _ => "no graphics API",
    };
    let mut summary = match name.trim() {
        "" => backend.to_owned(),
        name => format!("{name} \u{b7} {backend}"),
    };
    if device_type == DeviceType::Cpu {
        summary.push_str(" \u{b7} software rendering");
    }
    summary
}

/// Whether the Help Center's page is one of the two this dialog replaced.
/// System diagnostics and Create support bundle both still route here, so
/// either command opens it.
pub(super) const fn is_report_page(page: HelpCenterPage) -> bool {
    matches!(
        page,
        HelpCenterPage::Diagnostics | HelpCenterPage::SupportBundle
    )
}

fn operating_system() -> String {
    if cfg!(target_arch = "wasm32") {
        return "Web browser (WebAssembly)".to_owned();
    }
    let name = match std::env::consts::OS {
        "windows" => "Windows",
        "macos" => "macOS",
        "linux" => "Linux",
        other => other,
    };
    format!("{name} {}", std::env::consts::ARCH)
}

/// The file the session log is written to, where this platform has one.
fn session_log_file() -> Option<String> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        use crate::workbench::lifecycle::recovery;
        if recovery::diagnostics_folder_supported() {
            return recovery::diagnostics_directory().map(|directory| {
                directory
                    .join(recovery::SESSION_LOG_FILE)
                    .display()
                    .to_string()
            });
        }
    }
    None
}

impl RSpiceApp {
    pub(in crate::workbench) fn render_report_problem_dialog(&mut self, ctx: &Context) {
        let mut include_session_log = self.state.dialogs.help_center.include_session_log;
        let mut reviewed = self.state.dialogs.help_center.disclosure_reviewed;
        let entries = self.state.model.log_buffer.len();
        let log_file = session_log_file();
        let mut open_folder = false;

        let mut dialog = Dialog::new("Report a problem", "Create support bundle\u{2026}")
            .description(
                "The build, platform and session log support needs to reproduce a problem, the \
                 support bundle that carries them, and the address to send it to.",
            )
            .size(DialogSize::Transaction)
            // Scroll-free: no gutter down the trailing edge.
            .overlay_scrollbar()
            .ghost("Close")
            .primary_enabled(reviewed);
        if !reviewed {
            dialog = dialog
                .hint("Confirm what the bundle contains to create it")
                .hint_tone(DialogHintTone::Warn);
        }
        let choice = dialog.show(ctx, |ui| {
            fact_grid::show(ui, |rows| {
                rows.mono("Version", VERSION_LINE);
                rows.text("Platform", &operating_system());
                if let Some(graphics) = GRAPHICS_ADAPTER.get() {
                    rows.text("Graphics", graphics);
                }
                if let Some(log_file) = &log_file {
                    rows.mono("Session log", log_file);
                    rows.row("", |ui| open_folder = ui.button("Open folder").clicked());
                }
                rows.mono("Contact", SUPPORT_ADDRESS);
                rows.row("Bundle", |ui| {
                    ui.checkbox(
                        &mut include_session_log,
                        format!("Include the session log: {entries} entries, home folder removed"),
                    );
                });
                rows.row("", |ui| {
                    ui.checkbox(&mut reviewed, "I reviewed what the bundle contains");
                });
                rows.note(
                    "The bundle is a JSON file of the facts above, the open workspace, document \
                     counts and file-format versions. It never holds project files, models, \
                     results, license keys or credentials.",
                );
            });
        });

        self.state.dialogs.help_center.include_session_log = include_session_log;
        self.state.dialogs.help_center.disclosure_reviewed = reviewed;
        if open_folder {
            #[cfg(not(target_arch = "wasm32"))]
            if let Err(error) =
                crate::workbench::lifecycle::recovery::open_diagnostics_folder(&self.state)
            {
                self.state.prompt(format!(
                    "The session log folder could not be opened: {error}"
                ));
            }
        }
        match choice {
            DialogChoice::None => {}
            DialogChoice::Primary => export_support_bundle(self, include_session_log),
            _ => self.close_help_center_route(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eframe::egui_wgpu::wgpu::{Backend, DeviceType};

    #[test]
    fn the_graphics_summary_names_the_adapter_and_its_api() {
        assert_eq!(
            graphics_summary(
                "NVIDIA GeForce RTX 4070",
                Backend::Dx12,
                DeviceType::DiscreteGpu
            ),
            "NVIDIA GeForce RTX 4070 \u{b7} Direct3D 12"
        );
        assert_eq!(
            graphics_summary(
                "Microsoft Basic Render Driver",
                Backend::Dx12,
                DeviceType::Cpu
            ),
            "Microsoft Basic Render Driver \u{b7} Direct3D 12 \u{b7} software rendering"
        );
        assert_eq!(
            graphics_summary("", Backend::BrowserWebGpu, DeviceType::Other),
            "WebGPU"
        );
    }
}

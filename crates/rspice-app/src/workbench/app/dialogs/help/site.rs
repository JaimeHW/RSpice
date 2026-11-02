//! The pages of the RSpice site the Help menu opens in the reader's browser:
//! this version's release notes.
//!
//! The site publishes one release-notes entry per version, under the
//! element id `v<version>`, so the link lands on the entry for the build
//! the reader is running.

use crate::workbench::framework::requests::{OpenWebPage, Outbox};

/// This version's entry in the release notes.
pub(in crate::workbench) const RELEASE_NOTES_URL: &str =
    concat!("https://rspice.app/changelog#v", env!("CARGO_PKG_VERSION"));

/// Ask the frame to open this version's release notes in a new tab.
pub(in crate::workbench) fn open_release_notes(outbox: &mut Outbox) {
    outbox.push(OpenWebPage(RELEASE_NOTES_URL.to_owned()));
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workbench::RSpiceApp;
    use crate::workbench::commands::vocabulary::Command;
    use egui::{Context, OpenUrl, OutputCommand, Rect, pos2, vec2};

    /// One whole application frame; the pages it asked the platform to open.
    /// Generic over the app, so the helper is not a handler that takes the
    /// whole application.
    fn opened_pages(app: &mut impl eframe::App) -> Vec<OpenUrl> {
        let ctx = Context::default();
        crate::ui::Theme::default().apply(&ctx);
        let mut frame = eframe::Frame::_new_kittest();
        let output = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(pos2(0.0, 0.0), vec2(1440.0, 900.0))),
                time: Some(0.1),
                focused: true,
                ..Default::default()
            },
            |ui| eframe::App::ui(app, ui, &mut frame),
        );
        output
            .platform_output
            .commands
            .into_iter()
            .filter_map(|command| match command {
                OutputCommand::OpenUrl(open) => Some(open),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn release_notes_open_this_versions_entry_on_the_site_in_a_new_tab() {
        assert_eq!(
            RELEASE_NOTES_URL,
            format!(
                "https://rspice.app/changelog#v{}",
                env!("CARGO_PKG_VERSION")
            )
        );
        let mut app = RSpiceApp::test_instance();
        assert!(opened_pages(&mut app).is_empty(), "nothing opens unasked");
        Command::ReleaseNotes.execute(&mut app);
        assert_eq!(
            opened_pages(&mut app),
            [OpenUrl::new_tab(RELEASE_NOTES_URL)]
        );
        assert!(
            !app.state.shell.help_center.open,
            "release notes are the site's, not a page of the Help center"
        );
        assert!(opened_pages(&mut app).is_empty(), "the page opens once");
    }
}

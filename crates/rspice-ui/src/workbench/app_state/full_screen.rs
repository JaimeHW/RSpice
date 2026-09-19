//! Full-screen state transitions shared by commands and platform events.

use super::AppState;

impl AppState {
    /// Put the application window into full screen, keeping the panels the reader
    /// has open.
    ///
    /// Hiding the navigator and inspector is its own command — Focus mode — so it
    /// is not asked again here.
    pub(crate) fn enter_full_screen_presentation(&mut self) {
        self.workbench.full_screen_presentation = true;
        self.workbench.full_screen = true;
        self.ui.request_full_screen(true);
    }

    pub(crate) fn exit_full_screen_presentation(&mut self) {
        self.clear_full_screen_presentation(true);
    }

    pub(crate) fn clear_full_screen_presentation(&mut self, request_platform_exit: bool) {
        let platform_full_screen = self.workbench.full_screen;
        self.workbench.full_screen = false;
        self.workbench.full_screen_presentation = false;
        if request_platform_exit && platform_full_screen {
            self.ui.request_full_screen(false);
        }
    }
}

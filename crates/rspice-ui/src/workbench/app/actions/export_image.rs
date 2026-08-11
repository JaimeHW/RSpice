//! Viewer-figure export.
//!
//! The figure goes to the hardcopy publication pipeline — the one that already
//! renders every result sheet and every schematic as PDF/A, PDF, SVG or PNG at
//! a chosen resolution, attaches provenance, and which Print has always used.
//! What was here instead was a second, worse export beside it: it sent
//! `ViewportCommand::Screenshot`, waited a frame, cropped the window capture to
//! the document well and encoded that. A "figure" was therefore the pixels
//! already on screen, at whatever the display happened to be, with the chrome
//! removed by rectangle — no resolution, no vector format, no page size, no
//! colour profile, no provenance.
//!
//! Both targets take that route now. The browser kept the canvas capture for a
//! while on the claim that the pipeline could not write an artifact there, and
//! that claim was wrong. `dialogs::hardcopy::finalize` is native-only because
//! it owns the *spooler* and the durable-file compare-and-exchange; the browser
//! has its own owner for the same stage, and it is complete —
//! `dialogs::hardcopy::worker` renders `RenderTarget::ExportArtifact` off the
//! main thread, packages multi-part output as a deterministic ZIP, and
//! `ExportWorkflowIo::write_bytes_file_observed` hands the bytes to the
//! download manager. One `#[cfg]` on one module is not a statement about the
//! pipeline it belongs to.

use crate::workbench::app::RSpiceApp;

impl RSpiceApp {
    /// Pump a pending figure-export request.
    pub(in crate::workbench) fn handle_image_export(&mut self) {
        if !std::mem::take(&mut self.state.ui.export_figure_requested) {
            return;
        }
        crate::workbench::app::open_hardcopy_workflow(
            self,
            crate::workbench::app::HardcopyWorkflow::Export,
        );
    }
}

#[cfg(test)]
mod tests {
    use crate::workbench::app::{HardcopyWorkflow, RSpiceApp};

    /// The workflow refuses to open without a retained source, so the fixture
    /// has to make the active cell view the active document first — the same
    /// setup the publish dialog's own tests use.
    fn app_with_an_exportable_document() -> RSpiceApp {
        let mut app = RSpiceApp::test_instance();
        let reference = app.state.workspace.active_view.clone();
        app.state.workbench.documents.activate(
            crate::workbench::state::WorkspaceDocumentId::CellView(reference),
        );
        app
    }

    /// Guards the routing, not the artifact: a figure request must reach the
    /// publication pipeline. This runs on the host target only, so it cannot
    /// prove the browser does the same — what protects that is the absence of
    /// a `#[cfg(target_arch)]` fork in `handle_image_export`. Re-introduce one
    /// and this test keeps passing while the browser silently diverges again.
    #[test]
    fn a_figure_request_opens_the_publication_workflow() {
        let mut app = app_with_an_exportable_document();
        app.state.ui.export_figure_requested = true;

        app.handle_image_export();

        assert!(
            !app.state.ui.export_figure_requested,
            "the request must be consumed exactly once"
        );
        assert!(
            app.state.dialogs.hardcopy.open,
            "the figure export must open the hardcopy dialog"
        );
        assert_eq!(
            app.state.dialogs.hardcopy.workflow,
            HardcopyWorkflow::Export
        );
    }

    #[test]
    fn an_absent_request_does_not_open_anything() {
        let mut app = app_with_an_exportable_document();

        app.handle_image_export();

        assert!(!app.state.dialogs.hardcopy.open);
    }
}

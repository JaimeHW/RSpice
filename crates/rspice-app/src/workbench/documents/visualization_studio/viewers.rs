//! Studio viewer composition and immediate source/document action adapters.

use super::*;
use rspice_results_ui::studio::viewers::{self, library, toolbar};

pub(super) fn show(ui: &mut Ui, app: &mut RSpiceApp, compact: bool) {
    viewers::show(ui, &mut Host(app), compact);
}

struct Host<'a>(&'a mut RSpiceApp);

impl viewers::ViewerHost for Host<'_> {
    fn toolbar(&mut self, ui: &mut Ui, compact: bool) {
        toolbar::show(ui, self, compact);
    }
    fn library(&mut self, ui: &mut Ui) {
        let query = library::filter(
            ui,
            &mut self.0.state.workbench.visualization_studio.viewer_query,
        );
        let analysis_ids = available_analysis_ids(&self.0.state);
        let capabilities = ViewerCapabilities {
            analysis_ids: &analysis_ids,
            external_capabilities: &[],
        };
        if let Some((id, viewer)) = library::show(
            ui,
            &query,
            &self
                .0
                .state
                .workbench
                .visualization_studio
                .selected_viewer_document,
            |definition| resolved_viewer_availability(&self.0.state, definition, capabilities),
        ) {
            self.0
                .state
                .workbench
                .visualization_studio
                .selected_viewer_document = id.to_owned();
            add_viewer_pane(self.0, id, viewer);
        }
    }
    fn stage(&mut self, ui: &mut Ui) {
        viewer_stage(ui, self.0);
    }
    fn inspector(&mut self, ui: &mut Ui, compact: bool) {
        viewer_inspector(ui, self.0, compact);
    }
}

impl toolbar::ToolbarHost for Host<'_> {
    fn waveform_coordinates(&self) -> bool {
        self.0.state.ui.results.viewer == ResultViewer::Waves
    }
    fn magnification_available(&self) -> bool {
        magnification_available(&self.0.state)
    }
    fn tool(&self) -> ViewerTool {
        self.0.state.workbench.visualization_studio.tool
    }
    fn fit_block_reason(&self) -> Option<&'static str> {
        fit_block_reason(&self.0.state)
    }
    fn magnification_readout(&mut self, tokens: &Tokens) -> String {
        let status = result_document::active_shared_x_status(tokens, &mut self.0.state);
        magnification_readout(status.as_ref())
    }
    fn request(&mut self, action: toolbar::ToolbarAction) {
        use toolbar::ToolbarAction;
        match action {
            ToolbarAction::SelectTool(tool) => {
                self.0.state.workbench.visualization_studio.tool = tool
            }
            ToolbarAction::TraceManager => open_trace_manager(self.0),
            ToolbarAction::Axes => {
                self.0.state.workbench.visualization_studio.section = VisualizationSection::Axes
            }
            ToolbarAction::AddCursor => add_cursor_at_midpoint(self.0),
            ToolbarAction::AddMarker => add_marker_at_midpoint(self.0),
            ToolbarAction::Measurement => open_dock(self.0, VisualizationDock::Measurement),
            ToolbarAction::Annotation => open_dock(self.0, VisualizationDock::Annotation),
            ToolbarAction::Export => export_document(self.0),
            ToolbarAction::Fit => fit_active_view(self.0),
            ToolbarAction::Zoom(factor) => zoom_active(self.0, factor),
        }
    }
}

//! The results feature observed through a whole application: the commands
//! that reach the Results workspace and its return edges, and the cases that
//! read or write another feature's state on the way.

mod availability;
mod calculator;
mod clear_history;
mod commands;
mod create_document;
mod cross_probe;
mod deck_outputs;
mod document_edit;
mod expression_diagnostics;
#[cfg(not(target_arch = "wasm32"))]
mod full_disk;
pub(super) mod histogram_publication;
mod inspector;
mod keys;
#[cfg(not(target_arch = "wasm32"))]
mod large_data;
mod loop_gain;
mod navigation;
mod navigator_browser;
mod op_inspector;
mod optimization_apply;
mod persistent_document;
mod plot_page;
mod result_import_adapters;
mod result_import_workflow;
mod retained_memo;
mod runs;
mod saved_documents;
mod sheet_print;
mod shown_analysis;
mod sweep_sheet;
mod two_windows;
mod viewer_capabilities;
mod waveform_export;
mod working_view;

use crate::workbench::RSpiceApp;
use rspice_app_results::workbench::documents::result_document::ResultsParts;
use rspice_app_runs::simulation::controller::SimulationController;

/// The data browser's rows as one frame's accessibility tree reads them: the
/// saved documents' rows, and the heads of the runs and their analyses.
mod browser_rows {
    use egui::accesskit::{Node, NodeId, Toggled};

    /// One row: what it reads, whether it reads as what Results shows,
    /// whether it is marked unsaved, and where it stands.
    #[derive(Clone, Debug)]
    pub(super) struct Row {
        pub(super) id: NodeId,
        pub(super) label: String,
        pub(super) shown: bool,
        pub(super) unsaved: bool,
        pub(super) top: f64,
        pub(super) centre: egui::Pos2,
    }

    fn row(id: NodeId, node: &Node) -> Option<Row> {
        let bounds = node.bounds()?;
        Some(Row {
            id,
            label: node.label()?.to_owned(),
            shown: node.toggled() == Some(Toggled::True),
            unsaved: node
                .description()
                .is_some_and(|description| description.ends_with("unsaved changes")),
            top: bounds.y0,
            centre: egui::pos2(
                ((bounds.x0 + bounds.x1) * 0.5) as f32,
                ((bounds.y0 + bounds.y1) * 0.5) as f32,
            ),
        })
    }

    fn top_down(nodes: &[(NodeId, Node)], keep: impl Fn(&Node) -> bool) -> Vec<Row> {
        let mut rows = nodes
            .iter()
            .filter(|(_, node)| keep(node))
            .filter_map(|(id, node)| row(*id, node))
            .collect::<Vec<_>>();
        rows.sort_by(|above, below| above.top.total_cmp(&below.top));
        rows
    }

    /// The saved documents' rows, top to bottom.
    pub(super) fn documents(nodes: &[(NodeId, Node)]) -> Vec<Row> {
        top_down(nodes, |node| {
            node.description()
                .is_some_and(|description| description.starts_with("Saved result document"))
        })
    }

    /// The names the documents' rows read, top to bottom.
    pub(super) fn document_names(nodes: &[(NodeId, Node)]) -> Vec<String> {
        documents(nodes).into_iter().map(|row| row.label).collect()
    }

    /// The heads of runs and analyses, top to bottom: each says how many
    /// quantities it holds.
    fn heads(nodes: &[(NodeId, Node)]) -> Vec<Row> {
        top_down(nodes, |node| {
            node.label()
                .is_some_and(|label| label.contains(" quantities, "))
        })
    }

    /// The head of the run labelled `run`.
    pub(super) fn run_head(nodes: &[(NodeId, Node)], run: &str) -> Row {
        let start = format!("{run}, ");
        heads(nodes)
            .into_iter()
            .find(|head| head.label.starts_with(&start))
            .unwrap_or_else(|| panic!("the browser lists {run}"))
    }

    /// The head of the first analysis under the run labelled `run`, which
    /// must be open.
    pub(super) fn analysis_under(nodes: &[(NodeId, Node)], run: &str) -> Row {
        let start = format!("{run}, ");
        let heads = heads(nodes);
        let at = heads
            .iter()
            .position(|head| head.label.starts_with(&start))
            .unwrap_or_else(|| panic!("the browser lists {run}"));
        heads
            .get(at + 1)
            .cloned()
            .unwrap_or_else(|| panic!("{run} lists an analysis"))
    }

    /// The rows that read as what Results shows.
    pub(super) fn shown(nodes: &[(NodeId, Node)]) -> Vec<String> {
        let mut rows = documents(nodes);
        rows.extend(heads(nodes));
        rows.into_iter()
            .filter(|row| row.shown)
            .map(|row| row.label)
            .collect()
    }
}

/// Lend `app`'s results parts and run controller to `draw`, as the shell
/// lends them to the results surface.
fn with_surface_parts<R>(
    app: &mut RSpiceApp,
    draw: impl FnOnce(&mut ResultsParts<'_>, &mut SimulationController) -> R,
) -> R {
    draw(
        &mut app.state.results_parts(),
        &mut app.simulation_controller,
    )
}

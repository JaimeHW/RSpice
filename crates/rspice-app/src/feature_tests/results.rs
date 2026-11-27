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
/// heads of the runs and their analyses.
mod browser_rows {
    use egui::accesskit::{Node, NodeId, Toggled};

    /// One head: what it reads, whether it reads as what Results shows, and
    /// where it stands.
    #[derive(Clone, Debug)]
    pub(super) struct Row {
        pub(super) id: NodeId,
        pub(super) label: String,
        pub(super) shown: bool,
        pub(super) top: f64,
    }

    /// The heads of runs and analyses, top to bottom: each says how many
    /// quantities it holds.
    fn heads(nodes: &[(NodeId, Node)]) -> Vec<Row> {
        let mut rows = nodes
            .iter()
            .filter(|(_, node)| {
                node.label()
                    .is_some_and(|label| label.contains(" quantities, "))
            })
            .filter_map(|(id, node)| {
                Some(Row {
                    id: *id,
                    label: node.label()?.to_owned(),
                    shown: node.toggled() == Some(Toggled::True),
                    top: node.bounds()?.y0,
                })
            })
            .collect::<Vec<_>>();
        rows.sort_by(|above, below| above.top.total_cmp(&below.top));
        rows
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

    /// The heads that read as what Results shows.
    pub(super) fn shown(nodes: &[(NodeId, Node)]) -> Vec<String> {
        heads(nodes)
            .into_iter()
            .filter(|row| row.shown)
            .map(|row| row.label)
            .collect()
    }
}

/// The document strip's tabs as one frame's accessibility tree reads them.
mod strip_tabs {
    use egui::accesskit::{Node, NodeId, Role};

    /// One tab: what it reads, whether it is in front, whether it is marked
    /// unsaved, whether it closes, and where it stands.
    #[derive(Clone, Debug)]
    pub(super) struct Tab {
        pub(super) id: NodeId,
        pub(super) label: String,
        pub(super) selected: bool,
        pub(super) unsaved: bool,
        pub(super) closable: bool,
        pub(super) centre: egui::Pos2,
    }

    /// The strip's tabs, left to right. They are the tabs that say how they
    /// close or that they never do; a viewer's tabs say neither.
    pub(super) fn tabs(nodes: &[(NodeId, Node)]) -> Vec<Tab> {
        let mut tabs = nodes
            .iter()
            .filter(|(_, node)| node.role() == Role::Tab)
            .filter_map(|(id, node)| {
                let description = node.description()?;
                let closable = description.ends_with("Press Delete to close.");
                let pinned =
                    description == "Pinned primary document." || description == "Unsaved changes.";
                if !closable && !pinned {
                    return None;
                }
                let bounds = node.bounds()?;
                Some(Tab {
                    id: *id,
                    label: node.label()?.to_owned(),
                    selected: node.is_selected() == Some(true),
                    unsaved: description.starts_with("Unsaved changes"),
                    closable,
                    centre: egui::pos2(
                        ((bounds.x0 + bounds.x1) * 0.5) as f32,
                        ((bounds.y0 + bounds.y1) * 0.5) as f32,
                    ),
                })
            })
            .collect::<Vec<_>>();
        tabs.sort_by(|left, right| left.centre.x.total_cmp(&right.centre.x));
        tabs
    }

    /// What the strip's tabs read, left to right.
    pub(super) fn labels(nodes: &[(NodeId, Node)]) -> Vec<String> {
        tabs(nodes).into_iter().map(|tab| tab.label).collect()
    }

    /// Each tab's label and whether it is marked unsaved, left to right.
    pub(super) fn marks(nodes: &[(NodeId, Node)]) -> Vec<(String, bool)> {
        tabs(nodes)
            .into_iter()
            .map(|tab| (tab.label, tab.unsaved))
            .collect()
    }

    /// The tab labelled `label`.
    pub(super) fn tab(nodes: &[(NodeId, Node)], label: &str) -> Tab {
        tabs(nodes)
            .into_iter()
            .find(|tab| tab.label == label)
            .unwrap_or_else(|| panic!("the strip has no tab {label}"))
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

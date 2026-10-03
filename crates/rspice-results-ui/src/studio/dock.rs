//! Studio dock identities and action-sheet presentation.

pub mod comparison;
pub mod entities;
pub mod family;
pub mod layout;
pub mod pane;

use super::widgets::section_heading;
use egui::{ScrollArea, Ui};
use rspice_results::studio_presentation::VisualizationSection;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisualizationDock {
    AddPane,
    TraceManager,
    CursorManager,
    DocumentProperties,
    ReorderPanes,
    LinkGroups,
    PageEditor,
    Measurement,
    Annotation,
    FamilySlice,
    FamilyEncoding,
    FamilyFilter,
    Comparison,
    Export,
}

impl VisualizationDock {
    pub const fn title(self) -> &'static str {
        match self {
            Self::AddPane => "Add visualization pane",
            Self::TraceManager => "Trace and family manager",
            Self::CursorManager => "Cursor and marker manager",
            Self::DocumentProperties => "Document properties",
            Self::ReorderPanes => "Reorder visualization panes",
            Self::LinkGroups => "Axis and cursor link groups",
            Self::PageEditor => "Assign pane to report page",
            Self::Measurement => "Create result measurement",
            Self::Annotation => "Create result annotation",
            Self::FamilySlice => "Family slicing and pivot",
            Self::FamilyEncoding => "Family visual encoding",
            Self::FamilyFilter => "Advanced family filter",
            Self::Comparison => "Plan explicit comparison",
            Self::Export => "Export visualization document",
        }
    }
}

pub fn actions_sheet(
    ui: &mut Ui,
    section: VisualizationSection,
    mut open: impl FnMut(VisualizationDock),
) {
    section_heading(ui, section);
    ScrollArea::vertical()
        .id_salt("visualization.actions-sheet")
        .show(ui, |ui| {
            let actions = [
                ("Add visualization pane", VisualizationDock::AddPane),
                ("Trace manager", VisualizationDock::TraceManager),
                ("Cursor manager", VisualizationDock::CursorManager),
                ("Document properties", VisualizationDock::DocumentProperties),
                ("Export document", VisualizationDock::Export),
            ];
            for (label, dock) in actions {
                if ui
                    .add_sized([ui.available_width(), 44.0], egui::Button::new(label))
                    .clicked()
                {
                    open(dock);
                }
            }
        });
}

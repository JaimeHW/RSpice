//! What the instrument says, what its verbs do, and whether it fits.

use egui::vec2;

use crate::state::{Component, Point};
use crate::workbench::app_state::Model;

/// The centre column the shell leaves this stage at one viewport.
///
/// Read off `LayoutSpec`, which owns every one of these numbers, rather than
/// measured from a rendered frame: the gate is about whether the instrument
/// fits the space the shell gives it, and the shell decides that space from
/// exactly these fields.
pub fn stage_size(width: f32, height: f32) -> egui::Vec2 {
    use crate::workbench::design_system::ACTIVITY_RAIL_W;
    use crate::workbench::layout::LayoutSpec;
    use crate::workbench::state::WorkbenchSession;

    let mut workbench = WorkbenchSession::default();
    workbench.workspace = crate::workbench::state::Workspace::Stimulus;
    let layout = LayoutSpec::resolve(width, height, &workbench);
    let columns = width
        - if layout.show_activity_rail {
            ACTIVITY_RAIL_W
        } else {
            0.0
        }
        - if layout.show_navigator_dock {
            layout.navigator_width
        } else {
            0.0
        }
        - if layout.show_inspector_dock {
            layout.inspector_width
        } else {
            0.0
        };
    let rows = height
        - layout.title_bar_height
        - layout.toolbar_height
        - if layout.show_status_bar {
            layout.status_bar_height
        } else {
            0.0
        }
        // The collapsed console keeps its tab track, which is the one chrome
        // row the layout spec states only as a height it can grow to.
        - if layout.show_console_strip { 31.0 } else { 0.0 };
    vec2(columns.max(320.0), rows.max(240.0))
}

/// One placed source that has adopted a fixture definition.
pub fn adopter(model: &mut Model, name: &str, instance: &str) -> u64 {
    let definition = model
        .workspace
        .content
        .stimulus_library
        .get(name)
        .cloned()
        .expect("fixture definition");
    let id = 7;
    let mut component = Component::new(id, definition.component_type(), Point::new(4, 4));
    component.name = instance.to_owned();
    definition.adopt_onto(&mut component).expect("adopt");
    model
        .schematic
        .document_mut_for_test()
        .components
        .push(component);
    id
}

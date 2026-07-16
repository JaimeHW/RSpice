//! Context docks for the clean-room workbench.

mod console;
mod drawers;
mod inspector;
mod navigator;

use egui::{Context, Frame, Id, SidePanel, TopBottomPanel, containers::PanelState};

use crate::common::RSpiceApp;
use crate::ui::tokens::Tokens;

use super::layout::LayoutSpec;

const NAVIGATOR_PANEL_ID: &str = "workbench.navigator";
const INSPECTOR_PANEL_ID: &str = "workbench.inspector";
const CONSOLE_PANEL_ID: &str = "workbench.console";

/// Keep egui's panel cache subordinate to the persisted workbench contract.
///
/// `Panel` remembers its last rectangle by id, including rectangles produced
/// solely by a viewport or breakpoint change. Responsive defaults and Reset
/// Layout must therefore clear a stale cached rectangle before the panel is
/// constructed. A genuinely dragged splitter is copied back to
/// [`WorkbenchState`](super::state::WorkbenchState) after painting and will
/// match the cache on the next frame.
pub(super) fn synchronize_panel_memory(ctx: &Context, app: &RSpiceApp, layout: LayoutSpec) {
    synchronize_panel_size(
        ctx,
        NAVIGATOR_PANEL_ID,
        layout.navigator_width,
        layout.navigator_resizable && app.state.workbench.navigator_width_custom,
        PanelDimension::Width,
    );
    synchronize_panel_size(
        ctx,
        INSPECTOR_PANEL_ID,
        layout.inspector_width,
        layout.inspector_resizable && app.state.workbench.inspector_width_custom,
        PanelDimension::Width,
    );
    synchronize_panel_size(
        ctx,
        CONSOLE_PANEL_ID,
        layout.console_height,
        layout.console_resizable,
        PanelDimension::Height,
    );
}

#[derive(Clone, Copy)]
enum PanelDimension {
    Width,
    Height,
}

fn synchronize_panel_size(
    ctx: &Context,
    panel_id: &'static str,
    authoritative_size: f32,
    retain_matching_cache: bool,
    dimension: PanelDimension,
) {
    let id = Id::new(panel_id);
    let cached_size = PanelState::load(ctx, id).map(|state| match dimension {
        PanelDimension::Width => state.rect.width(),
        PanelDimension::Height => state.rect.height(),
    });
    if panel_cache_is_stale(cached_size, authoritative_size, retain_matching_cache) {
        ctx.data_mut(|data| data.remove::<PanelState>(id));
    }
}

fn panel_cache_is_stale(
    cached_size: Option<f32>,
    authoritative_size: f32,
    retain_matching_cache: bool,
) -> bool {
    !retain_matching_cache
        || !cached_size.is_some_and(|cached| (cached - authoritative_size).abs() <= 0.5)
}

fn splitter_is_dragged(ctx: &Context, panel_id: &'static str) -> bool {
    ctx.read_response(Id::new(panel_id).with("__resize"))
        .is_some_and(|response| response.dragged())
}

pub fn show_navigator(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let panel = SidePanel::left(NAVIGATOR_PANEL_ID)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true);
    let panel = if layout.navigator_resizable {
        panel
            .default_width(layout.navigator_width)
            .width_range(220.0..=440.0)
            .resizable(true)
    } else {
        panel.exact_width(layout.navigator_width).resizable(false)
    };
    let shown = panel.show(ctx, |ui| navigator::show(ui, app));
    if layout.navigator_resizable && splitter_is_dragged(ctx, NAVIGATOR_PANEL_ID) {
        let actual = shown.response.rect.width();
        app.state.workbench.navigator_width = actual.clamp(220.0, 440.0);
        app.state.workbench.navigator_width_custom = true;
    }
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Complementary);
        node.set_label("Workspace navigator");
    });
}

pub fn show_inspector(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let panel = SidePanel::right(INSPECTOR_PANEL_ID)
        .default_width(layout.inspector_width)
        .width_range(278.0..=440.0)
        .resizable(layout.inspector_resizable)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true);
    let shown = panel.show(ctx, |ui| inspector::show(ui, app));
    if layout.inspector_resizable && splitter_is_dragged(ctx, INSPECTOR_PANEL_ID) {
        let actual = shown.response.rect.width();
        app.state.workbench.inspector_width = actual.clamp(278.0, 440.0);
        app.state.workbench.inspector_width_custom = true;
    }
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Complementary);
        node.set_label("Inspector");
    });
}

pub fn show_console(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    if !layout.show_console_strip {
        return;
    }
    let t = Tokens::get(ctx);
    let panel = TopBottomPanel::bottom(CONSOLE_PANEL_ID)
        .default_height(layout.console_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true);
    let panel = if layout.console_resizable {
        panel
            .height_range(layout.console_min_height..=layout.console_max_height)
            .resizable(true)
    } else {
        // Collapsed, touch, and maximized compositions are exact mockup rows;
        // no previously dragged desktop height may leak into them.
        panel.exact_height(layout.console_height).resizable(false)
    };
    let shown = panel.show(ctx, |ui| console::show(ui, app, layout));
    if layout.console_resizable && splitter_is_dragged(ctx, CONSOLE_PANEL_ID) {
        app.state.workbench.console_height = shown
            .response
            .rect
            .height()
            .clamp(layout.console_min_height, layout.console_max_height);
    }
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Region);
        node.set_label("Console and diagnostics");
    });
}

pub fn show_drawers(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    drawers::show(ctx, app, layout);
}

#[cfg(test)]
mod tests {
    use super::panel_cache_is_stale;

    #[test]
    fn responsive_and_reset_layouts_discard_egui_panel_memory() {
        assert!(panel_cache_is_stale(Some(256.0), 230.4, false));
        assert!(panel_cache_is_stale(Some(256.0), 256.0, false));
    }

    #[test]
    fn only_a_matching_explicit_resize_retains_egui_panel_memory() {
        assert!(!panel_cache_is_stale(Some(340.0), 340.0, true));
        assert!(panel_cache_is_stale(Some(340.0), 312.0, true));
        assert!(panel_cache_is_stale(None, 340.0, true));
    }
}

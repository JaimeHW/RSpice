//! Context docks for the clean-room workbench.

mod console;
mod drawers;
mod inspector;
mod navigator;

use egui::{Context, Frame, Id, Key, Panel, Response, containers::PanelState};

use crate::ui::tokens::Tokens;
use crate::workbench::RSpiceApp;

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SplitterKeyboardAction {
    Decrease,
    Increase,
    Reset,
}

fn splitter_response(ctx: &Context, panel_id: &'static str) -> Option<Response> {
    ctx.read_response(Id::new(panel_id).with("__resize"))
}

fn horizontal_splitter_action(
    ctx: &Context,
    response: &Response,
    increase_key: Key,
    decrease_key: Key,
) -> Option<SplitterKeyboardAction> {
    // egui's built-in panel separator deliberately uses `Sense::drag()`,
    // which is focusable but does not set the response's CLICKED flag. Read
    // the pointer's double-click cadence directly and scope it to the exact
    // separator rectangle so the mockup's reset gesture remains available
    // without replacing egui's mature drag handling.
    if ctx.input(|input| {
        input
            .pointer
            .interact_pos()
            .is_some_and(|position| response.rect.contains(position))
            && input
                .pointer
                .button_double_clicked(egui::PointerButton::Primary)
    }) {
        return Some(SplitterKeyboardAction::Reset);
    }
    if !response.has_focus() {
        return None;
    }
    ctx.input(|input| {
        if input.key_pressed(Key::Home) {
            Some(SplitterKeyboardAction::Reset)
        } else if input.key_pressed(increase_key) {
            Some(SplitterKeyboardAction::Increase)
        } else if input.key_pressed(decrease_key) {
            Some(SplitterKeyboardAction::Decrease)
        } else {
            None
        }
    })
}

fn vertical_splitter_action(ctx: &Context, response: &Response) -> Option<SplitterKeyboardAction> {
    horizontal_splitter_action(ctx, response, Key::ArrowUp, Key::ArrowDown)
}

fn apply_splitter_step(value: f32, action: SplitterKeyboardAction, step: f32) -> Option<f32> {
    match action {
        SplitterKeyboardAction::Decrease => Some(value - step),
        SplitterKeyboardAction::Increase => Some(value + step),
        SplitterKeyboardAction::Reset => None,
    }
}

fn expose_splitter_accessibility(
    ctx: &Context,
    response: &Response,
    label: &'static str,
    value: f32,
    minimum: f32,
    maximum: f32,
    step: f32,
) {
    ctx.accesskit_node_builder(response.id, |node| {
        node.set_role(egui::accesskit::Role::Splitter);
        node.set_label(label);
        node.set_numeric_value(f64::from(value));
        node.set_min_numeric_value(f64::from(minimum));
        node.set_max_numeric_value(f64::from(maximum));
        node.set_numeric_value_step(f64::from(step));
    });
}

pub fn show_navigator(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let panel = Panel::left(NAVIGATOR_PANEL_ID)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true);
    let panel = if layout.navigator_resizable {
        panel
            .default_size(layout.navigator_width)
            .size_range(220.0..=440.0)
            .resizable(true)
    } else {
        panel.exact_size(layout.navigator_width).resizable(false)
    };
    let shown = panel.show(ctx, |ui| navigator::show(ui, app));
    if layout.navigator_resizable && splitter_is_dragged(ctx, NAVIGATOR_PANEL_ID) {
        let actual = shown.response.rect.width();
        app.state.workbench.navigator_width = actual.clamp(220.0, 440.0);
        app.state.workbench.navigator_width_custom = true;
    }
    if layout.navigator_resizable
        && let Some(response) = splitter_response(ctx, NAVIGATOR_PANEL_ID)
    {
        expose_splitter_accessibility(
            ctx,
            &response,
            "Resize workspace navigator",
            app.state.workbench.navigator_width,
            220.0,
            440.0,
            12.0,
        );
        if let Some(action) =
            horizontal_splitter_action(ctx, &response, Key::ArrowRight, Key::ArrowLeft)
        {
            if let Some(width) =
                apply_splitter_step(app.state.workbench.navigator_width, action, 12.0)
            {
                app.state.workbench.navigator_width = width.clamp(220.0, 440.0);
                app.state.workbench.navigator_width_custom = true;
            } else {
                app.state.workbench.navigator_width = 256.0;
                app.state.workbench.navigator_width_custom = false;
            }
            ctx.request_repaint();
        }
    }
    ctx.accesskit_node_builder(shown.response.id, |node| {
        node.set_role(egui::accesskit::Role::Complementary);
        node.set_label("Workspace navigator");
    });
}

pub fn show_inspector(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let panel = Panel::right(INSPECTOR_PANEL_ID)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true);
    let panel = if layout.inspector_resizable {
        panel
            .default_size(layout.inspector_width)
            .size_range(278.0..=440.0)
            .resizable(true)
    } else {
        // Responsive/touch projections own an exact dock width. Letting an
        // inspector child raise egui's non-resizable default would shift the
        // canvas and adjacent controls as selection content changes.
        panel.exact_size(layout.inspector_width).resizable(false)
    };
    let shown = panel.show(ctx, |ui| inspector::show(ui, app));
    if layout.inspector_resizable && splitter_is_dragged(ctx, INSPECTOR_PANEL_ID) {
        let actual = shown.response.rect.width();
        app.state.workbench.inspector_width = actual.clamp(278.0, 440.0);
        app.state.workbench.inspector_width_custom = true;
    }
    if layout.inspector_resizable
        && let Some(response) = splitter_response(ctx, INSPECTOR_PANEL_ID)
    {
        expose_splitter_accessibility(
            ctx,
            &response,
            "Resize inspector",
            app.state.workbench.inspector_width,
            278.0,
            440.0,
            12.0,
        );
        // The inspector is attached to the right edge, so moving its splitter
        // left increases the panel and moving it right decreases it.
        if let Some(action) =
            horizontal_splitter_action(ctx, &response, Key::ArrowLeft, Key::ArrowRight)
        {
            if let Some(width) =
                apply_splitter_step(app.state.workbench.inspector_width, action, 12.0)
            {
                app.state.workbench.inspector_width = width.clamp(278.0, 440.0);
                app.state.workbench.inspector_width_custom = true;
            } else {
                app.state.workbench.inspector_width = 312.0;
                app.state.workbench.inspector_width_custom = false;
            }
            ctx.request_repaint();
        }
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
    let panel = Panel::bottom(CONSOLE_PANEL_ID)
        .default_size(layout.console_height)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true);
    let panel = if layout.console_resizable {
        panel
            .size_range(layout.console_min_height..=layout.console_max_height)
            .resizable(true)
    } else {
        // Collapsed, touch, and maximized compositions are exact mockup rows;
        // no previously dragged desktop height may leak into them.
        panel.exact_size(layout.console_height).resizable(false)
    };
    let shown = panel.show(ctx, |ui| console::show(ui, app, layout));
    if layout.console_resizable && splitter_is_dragged(ctx, CONSOLE_PANEL_ID) {
        app.state.workbench.console_height = shown
            .response
            .rect
            .height()
            .clamp(layout.console_min_height, layout.console_max_height);
    }
    if layout.console_resizable
        && let Some(response) = splitter_response(ctx, CONSOLE_PANEL_ID)
    {
        expose_splitter_accessibility(
            ctx,
            &response,
            "Resize console and diagnostics",
            app.state.workbench.console_height,
            layout.console_min_height,
            layout.console_max_height,
            16.0,
        );
        if let Some(action) = vertical_splitter_action(ctx, &response) {
            app.state.workbench.console_height =
                apply_splitter_step(app.state.workbench.console_height, action, 16.0)
                    .unwrap_or(145.0)
                    .clamp(layout.console_min_height, layout.console_max_height);
            app.state.workbench.console_maximized = false;
            app.state.workbench.console_visible = true;
            ctx.request_repaint();
        }
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
    use super::{SplitterKeyboardAction, apply_splitter_step, panel_cache_is_stale};

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

    #[test]
    fn splitter_keyboard_steps_match_the_mockup_contract() {
        assert_eq!(
            apply_splitter_step(256.0, SplitterKeyboardAction::Increase, 12.0),
            Some(268.0)
        );
        assert_eq!(
            apply_splitter_step(312.0, SplitterKeyboardAction::Decrease, 12.0),
            Some(300.0)
        );
        assert_eq!(
            apply_splitter_step(145.0, SplitterKeyboardAction::Increase, 16.0),
            Some(161.0)
        );
        assert_eq!(
            apply_splitter_step(440.0, SplitterKeyboardAction::Reset, 12.0),
            None
        );
    }
}

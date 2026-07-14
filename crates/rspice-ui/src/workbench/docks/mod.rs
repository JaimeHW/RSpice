//! Context docks for the clean-room workbench.

mod console;
mod drawers;
mod inspector;
mod navigator;

use egui::{Context, Frame, SidePanel, TopBottomPanel};

use crate::common::RSpiceApp;
use crate::ui::tokens::Tokens;

use super::layout::LayoutSpec;

pub fn show_navigator(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let panel = SidePanel::left("workbench.navigator")
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
    panel.show(ctx, |ui| navigator::show(ui, app));
}

pub fn show_inspector(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    let panel = SidePanel::right("workbench.inspector")
        .default_width(layout.inspector_width)
        .width_range(278.0..=440.0)
        .resizable(layout.inspector_resizable)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true);
    panel.show(ctx, |ui| inspector::show(ui, app));
}

pub fn show_console(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    TopBottomPanel::bottom("workbench.console")
        .default_height(layout.console_height)
        .height_range(layout.console_min_height..=layout.console_max_height)
        .resizable(layout.console_resizable)
        .frame(Frame::new().fill(t.color.bg_inset))
        .show_separator_line(true)
        .show(ctx, |ui| console::show(ui, app, layout));
}

pub fn show_drawers(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    drawers::show(ctx, app, layout);
}

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
    SidePanel::left("workbench.navigator")
        .default_width(layout.navigator_width)
        .width_range(236.0..=460.0)
        .resizable(true)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| navigator::show(ui, app));
}

pub fn show_inspector(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    SidePanel::right("workbench.inspector")
        .default_width(layout.inspector_width)
        .width_range(260.0..=520.0)
        .resizable(true)
        .frame(Frame::new().fill(t.color.bg_panel))
        .show_separator_line(true)
        .show(ctx, |ui| inspector::show(ui, app));
}

pub fn show_console(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    let t = Tokens::get(ctx);
    TopBottomPanel::bottom("workbench.console")
        .default_height(layout.console_height)
        .height_range(112.0..=640.0)
        .resizable(!app.state.workbench.console_maximized)
        .frame(Frame::new().fill(t.color.bg_inset))
        .show_separator_line(true)
        .show(ctx, |ui| console::show(ui, app));
}

pub fn show_drawers(ctx: &Context, app: &mut RSpiceApp, layout: LayoutSpec) {
    drawers::show(ctx, app, layout);
}

//! Modal dialogs.
//!
//! One module per dialog, each owning its own state struct and rendering.
//! `DialogState` in [`state`] aggregates the visibility flags so a single
//! read answers whether any modal owns input this frame; the dialogs
//! themselves never inspect each other.

pub(in crate::workbench) mod check_and_save;
pub(in crate::workbench) mod check_and_save_validation;
pub(in crate::workbench) mod configuration_sets;
pub(in crate::workbench) mod confirmation;
pub(in crate::workbench) mod confirmation_state;
pub(in crate::workbench) mod connectivity_manager;
pub(in crate::workbench) mod create_model_bound_symbol;
pub(in crate::workbench) mod design_management;
pub(in crate::workbench) mod design_review_comments;
pub(in crate::workbench) mod drawing_sheet_defaults;
pub(in crate::workbench) mod drawing_sheet_layers;
pub(in crate::workbench) mod drawing_sheet_overflow;
pub(in crate::workbench) mod drawing_sheet_presets;
pub(in crate::workbench) mod drawing_sheet_preview;
pub(in crate::workbench) mod drawing_sheet_setup;
pub(in crate::workbench) mod drawing_sheet_support;
pub(in crate::workbench) mod engineering_table;
pub(in crate::workbench) mod find_replace;
pub(in crate::workbench) mod grid_snap_routing;
pub(in crate::workbench) mod hardcopy;
pub(in crate::workbench) mod help;
pub(in crate::workbench) mod hierarchy;
pub(in crate::workbench) mod library;
pub(in crate::workbench) mod license;
pub(in crate::workbench) mod live_session;
pub(in crate::workbench) mod object_properties;
pub(in crate::workbench) mod operation_primitives;
pub(in crate::workbench) mod pdk_settings;
pub(in crate::workbench) mod pdk_workflow;
pub(in crate::workbench) mod placement;
pub(in crate::workbench) mod plan_removal;
pub(in crate::workbench) mod preferences;
pub(in crate::workbench) mod project;
pub(in crate::workbench) mod project_revision_history;
pub(in crate::workbench) mod property_dialog;
pub(in crate::workbench) mod publish_web;
pub(in crate::workbench) mod replace_instance;
pub(in crate::workbench) mod review_primitives;
pub(in crate::workbench) mod rule_setup;
pub(in crate::workbench) mod schematic_command;
pub(in crate::workbench) mod selection;
pub(in crate::workbench) mod source_document;
pub(in crate::workbench) mod source_find;
pub(in crate::workbench) mod source_language_tools;
pub(in crate::workbench) mod state;
pub(in crate::workbench) mod stimulus_link;
pub(in crate::workbench) mod symbol_definition;
pub(in crate::workbench) mod unpublish_web;
pub(in crate::workbench) mod veriloga_build_settings;
pub(in crate::workbench) mod visibility_options;
pub(in crate::workbench) mod window_session;

use egui::Context;

use crate::workbench::app::RSpiceApp;

/// Render a modal form for review once its fade-in has finished.
///
/// The standard render rasterizes its third pass, while the modal is still
/// drawn below full opacity, and every tone in it comes out washed against
/// the backdrop: the accent primary measures under 2:1 there and 5:1 once
/// the fade is done. This one runs passes until the animation time has
/// passed on egui's frame clock.
#[cfg(all(test, not(target_arch = "wasm32")))]
pub(crate) fn render_settled_form(
    mode: crate::ui::tokens::Mode,
    size: egui::Vec2,
    mut pass: impl FnMut(&Context),
) -> crate::ui::raster::Canvas {
    let ctx = Context::default();
    ctx.set_os(egui::os::OperatingSystem::Nix);
    crate::ui::Theme {
        mode,
        ..crate::ui::Theme::default()
    }
    .apply(&ctx);
    let input = || egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(egui::Pos2::ZERO, size)),
        ..Default::default()
    };
    let _ = ctx.run_ui(input(), |ui| pass(ui.ctx()));
    let visible_at = ctx.input(|input| input.time) + f64::from(ctx.global_style().animation_time);
    for _ in 0..240 {
        let _ = ctx.run_ui(input(), |ui| pass(ui.ctx()));
        if ctx.input(|input| input.time) > visible_at {
            break;
        }
    }
    let output = ctx.run_ui(input(), |ui| pass(ui.ctx()));
    crate::ui::raster::rasterize(&ctx, size, &output)
}

impl RSpiceApp {
    pub(in crate::workbench) fn process_model_browser_dialog(&mut self, ctx: &Context) {
        let _ = crate::properties::model_browser::render_model_browser(
            ctx,
            &mut self.state.model_browser_state,
            &self.state.model_library_manager,
        );
    }
}

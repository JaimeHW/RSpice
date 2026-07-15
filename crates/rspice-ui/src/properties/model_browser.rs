//! Model Browser Dialog Module
//!
//! Commercial-grade PDK model browser for semiconductor property selection.
//! Matches Cadence Spectre's model library browsing capabilities.
//!
//! # Architecture
//!
//! - `ModelBrowserState`: UI state for modal dialog
//! - `ModelBrowserResult`: Selected model or cancelled
//! - `render_model_browser`: egui modal with tree, list, and details panes
//!
//! # Integration
//!
//! Used by the tabbed property dialog to allow users to browse and select
//! device models from loaded PDK libraries.

use crate::state::model_library::{DeviceModel, ModelLibrary, ModelLibraryManager, ModelType};
use crate::ui::theme::{self, FontWeight};
use crate::ui::tokens::{self, Tokens};
use crate::ui::widgets::{Dialog, DialogChoice, DialogSize, TreeRow, chip, kv_row};
use egui::{Context, ScrollArea, Ui};
use std::collections::HashSet;

// =============================================================================
// Model Browser State
// =============================================================================

/// State for the model browser dialog.
#[derive(Debug, Clone, Default)]
pub struct ModelBrowserState {
    /// Whether the dialog is open.
    pub open: bool,
    /// Search filter text.
    pub search_text: String,
    /// Filter by model type (None = all types).
    pub type_filter: Option<ModelType>,
    /// Currently selected library name.
    pub selected_library: Option<String>,
    /// Currently selected model name.
    pub selected_model: Option<String>,
    /// Expanded library nodes in tree view.
    pub expanded_libraries: HashSet<String>,
    /// Target component type (to filter compatible models).
    pub target_type: Option<ModelType>,
    /// Selected corner for the library.
    pub selected_corner: Option<String>,
    /// Browse-only mode (no Apply button, just viewing)
    pub browse_only: bool,
}

impl ModelBrowserState {
    /// Create a new browser state.
    pub fn new() -> Self {
        Self::default()
    }

    /// Open the browser dialog.
    pub fn open(&mut self) {
        self.open = true;
        self.clear_selection();
    }

    /// Open with a target type filter.
    pub fn open_for_type(&mut self, target: ModelType) {
        self.open = true;
        self.target_type = Some(target);
        self.type_filter = Some(target);
        self.clear_selection();
    }

    /// Close the dialog.
    pub fn close(&mut self) {
        self.open = false;
    }

    /// Clear the current selection.
    pub fn clear_selection(&mut self) {
        self.selected_library = None;
        self.selected_model = None;
    }

    /// Select a library.
    pub fn select_library(&mut self, name: &str) {
        self.selected_library = Some(name.to_string());
        self.selected_model = None;
    }

    /// Select a model.
    pub fn select_model(&mut self, library: &str, model: &str) {
        self.selected_library = Some(library.to_string());
        self.selected_model = Some(model.to_string());
    }

    /// Toggle library expansion in tree.
    pub fn toggle_library(&mut self, name: &str) {
        if self.expanded_libraries.contains(name) {
            self.expanded_libraries.remove(name);
        } else {
            self.expanded_libraries.insert(name.to_string());
        }
    }

    /// Check if a library is expanded.
    pub fn is_expanded(&self, name: &str) -> bool {
        self.expanded_libraries.contains(name)
    }

    /// Get the full model reference string (library:model).
    pub fn get_model_reference(&self) -> Option<String> {
        match (&self.selected_library, &self.selected_model) {
            (Some(lib), Some(model)) => Some(format!("{}:{}", lib, model)),
            _ => None,
        }
    }

    /// Filter models based on search text and type filter.
    pub fn matches_filter(&self, model: &DeviceModel) -> bool {
        // Type filter
        if let Some(target) = self.type_filter
            && model.model_type != target
        {
            return false;
        }

        // Search text filter
        if !self.search_text.is_empty() {
            let search_lower = self.search_text.to_lowercase();
            let name_matches = model.name.to_lowercase().contains(&search_lower);
            let desc_matches = model.description.to_lowercase().contains(&search_lower);
            if !name_matches && !desc_matches {
                return false;
            }
        }

        true
    }

    /// Get compatible models from a library.
    pub fn get_compatible_models<'a>(&self, library: &'a ModelLibrary) -> Vec<&'a DeviceModel> {
        library
            .models
            .values()
            .filter(|m| self.matches_filter(m))
            .collect()
    }

    /// Count matching models in a library.
    pub fn count_matching_models(&self, library: &ModelLibrary) -> usize {
        library
            .models
            .values()
            .filter(|m| self.matches_filter(m))
            .count()
    }
}

// =============================================================================
// Model Browser Result
// =============================================================================

/// Result of the model browser dialog.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum ModelBrowserResult {
    /// No action taken (dialog still open or closed without selection).
    #[default]
    None,
    /// User cancelled the dialog.
    Cancelled,
    /// User selected a model.
    Selected {
        /// Library name.
        library: String,
        /// Model name.
        model: String,
        /// Selected corner (if any).
        corner: Option<String>,
    },
}

// =============================================================================
// Model Browser Rendering
// =============================================================================

/// Pane content height inside the dialog body.
const PANE_HEIGHT: f32 = 380.0;

/// Render the model browser dialog.
///
/// Returns `ModelBrowserResult::Selected` when user confirms selection.
pub fn render_model_browser(
    ctx: &Context,
    state: &mut ModelBrowserState,
    manager: &ModelLibraryManager,
) -> ModelBrowserResult {
    let mut result = ModelBrowserResult::None;

    if !state.open {
        return result;
    }

    let hint = format!(
        "{} libraries · {} models",
        manager.library_count(),
        manager.total_model_count()
    );
    let description = if state.browse_only {
        "Search loaded model libraries and inspect device-model details."
    } else {
        "Search loaded model libraries, inspect device-model details, and apply a compatible model to the selected device."
    };

    let mut dialog = Dialog::new(
        "Library",
        "Model browser",
        if state.browse_only { "Close" } else { "Apply" },
    )
    .description(description)
    .size(DialogSize::Manager)
    .hint(&hint);
    if !state.browse_only {
        dialog = dialog
            .ghost("Cancel")
            .primary_enabled(state.selected_model.is_some());
    }

    let choice = dialog.show(ctx, |ui| {
        ui.spacing_mut().item_spacing.y = 4.0;

        // Search + type filter chips.
        ui.add(
            egui::TextEdit::singleline(&mut state.search_text)
                .hint_text("Search models…")
                .desired_width(f32::INFINITY),
        );
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            if chip(ui, "all", state.type_filter.is_none()).clicked() {
                state.type_filter = None;
            }
            for model_type in [
                ModelType::Nmos,
                ModelType::Pmos,
                ModelType::Npn,
                ModelType::Pnp,
                ModelType::Resistor,
                ModelType::Capacitor,
                ModelType::Diode,
            ] {
                if chip(
                    ui,
                    model_type.display_name(),
                    state.type_filter == Some(model_type),
                )
                .clicked()
                {
                    state.type_filter = Some(model_type);
                }
            }
        });
        ui.add_space(4.0);

        // Tree · list · detail panes.
        let t = Tokens::get(ui.ctx());
        let c = t.color;
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;
            let pane = |ui: &mut Ui, width: f32, id: &str, body: &mut dyn FnMut(&mut Ui)| {
                ui.allocate_ui_with_layout(
                    egui::vec2(width, PANE_HEIGHT),
                    egui::Layout::top_down(egui::Align::Min),
                    |ui| {
                        ScrollArea::vertical()
                            .id_salt(id.to_owned())
                            .auto_shrink([false, false])
                            .show(ui, |ui| body(ui));
                    },
                );
            };

            pane(ui, 190.0, "model_browser_libraries", &mut |ui| {
                render_library_tree(ui, state, manager);
            });
            let line_x = ui.cursor().left() + 4.0;
            ui.painter().vline(
                line_x,
                egui::Rangef::new(ui.cursor().top(), ui.cursor().top() + PANE_HEIGHT),
                egui::Stroke::new(1.0, c.border),
            );
            ui.add_space(9.0);
            pane(ui, 220.0, "model_browser_models", &mut |ui| {
                render_model_list(ui, state, manager);
            });
            let line_x = ui.cursor().left() + 4.0;
            ui.painter().vline(
                line_x,
                egui::Rangef::new(ui.cursor().top(), ui.cursor().top() + PANE_HEIGHT),
                egui::Stroke::new(1.0, c.border),
            );
            ui.add_space(9.0);
            let detail_width = ui.available_width().max(180.0);
            pane(ui, detail_width, "model_browser_details", &mut |ui| {
                render_model_details(ui, state, manager);
            });
        });
    });

    match choice {
        DialogChoice::Primary => {
            if state.browse_only {
                state.close();
                result = ModelBrowserResult::Cancelled;
            } else if let (Some(library), Some(model)) =
                (&state.selected_library, &state.selected_model)
            {
                result = ModelBrowserResult::Selected {
                    library: library.clone(),
                    model: model.clone(),
                    corner: state.selected_corner.clone(),
                };
                state.close();
            }
        }
        DialogChoice::Ghost | DialogChoice::Cancelled => {
            state.close();
            result = ModelBrowserResult::Cancelled;
        }
        DialogChoice::Secondary | DialogChoice::None => {}
    }

    result
}

/// Faint mono pane caption.
fn pane_caption(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    let mut job = egui::text::LayoutJob::default();
    job.append(
        text,
        0.0,
        egui::TextFormat {
            font_id: theme::mono(tokens::FS_0, FontWeight::Regular),
            color: t.color.text_faint,
            extra_letter_spacing: 0.08 * tokens::FS_0,
            ..Default::default()
        },
    );
    ui.label(job);
    ui.add_space(2.0);
}

/// Faint placeholder line.
fn pane_note(ui: &mut Ui, text: &str) {
    let t = Tokens::get(ui.ctx());
    ui.label(
        egui::RichText::new(text)
            .font(theme::sans(tokens::FS_0, FontWeight::Regular))
            .color(t.color.text_faint),
    );
}

/// Render the library tree in the left pane.
fn render_library_tree(ui: &mut Ui, state: &mut ModelBrowserState, manager: &ModelLibraryManager) {
    pane_caption(ui, "LIBRARIES");
    if manager.library_count() == 0 {
        pane_note(ui, "No libraries loaded");
        return;
    }

    ui.spacing_mut().item_spacing.y = 0.0;
    for lib in manager.libraries_sorted() {
        let is_selected = state.selected_library.as_deref() == Some(&lib.name);
        let matching_count = state.count_matching_models(lib);
        let row = TreeRow::new(&lib.name)
            .meta(&matching_count.to_string())
            .selected(is_selected)
            .show(ui);
        if row.response.clicked() {
            state.select_library(&lib.name);
        }
    }
}

/// Render the model list in the center pane.
fn render_model_list(ui: &mut Ui, state: &mut ModelBrowserState, manager: &ModelLibraryManager) {
    pane_caption(ui, "MODELS");
    ui.spacing_mut().item_spacing.y = 0.0;

    if let Some(ref lib_name) = state.selected_library.clone() {
        if let Some(lib) = manager.get_library(lib_name) {
            let mut models = state.get_compatible_models(lib);
            if models.is_empty() {
                pane_note(ui, "No matching models");
                return;
            }
            models.sort_by(|a, b| a.name.cmp(&b.name));

            for model in models {
                let is_selected = state.selected_model.as_deref() == Some(&model.name);
                let row = TreeRow::new(&model.name)
                    .meta(model.level.display_name())
                    .selected(is_selected)
                    .show(ui);
                if row.response.clicked() {
                    state.select_model(lib_name, &model.name);
                }
            }
        }
    } else if !state.search_text.is_empty() {
        let results = manager.search_models(&state.search_text);
        if results.is_empty() {
            pane_note(ui, "No results found");
            return;
        }
        let pairs: Vec<(String, String)> = results
            .iter()
            .filter(|(_, model)| state.matches_filter(model))
            .take(50)
            .map(|(lib, model)| (lib.name.clone(), model.name.clone()))
            .collect();
        for (lib_name, model_name) in pairs {
            let is_selected = state.selected_library.as_deref() == Some(lib_name.as_str())
                && state.selected_model.as_deref() == Some(model_name.as_str());
            let row = TreeRow::new(&model_name)
                .meta(&lib_name)
                .selected(is_selected)
                .show(ui);
            if row.response.clicked() {
                state.select_model(&lib_name, &model_name);
            }
        }
    } else {
        pane_note(ui, "Select a library or search");
    }
}

/// Render model details in the right pane.
fn render_model_details(ui: &mut Ui, state: &mut ModelBrowserState, manager: &ModelLibraryManager) {
    let t = Tokens::get(ui.ctx());
    let c = t.color;
    pane_caption(ui, "DETAILS");

    let (Some(lib_name), Some(model_name)) =
        (state.selected_library.clone(), state.selected_model.clone())
    else {
        pane_note(ui, "Select a model to view details");
        return;
    };
    let Some(lib) = manager.get_library(&lib_name) else {
        return;
    };
    let Some(model) = lib.get_model(&model_name) else {
        return;
    };

    ui.label(
        egui::RichText::new(&model.name)
            .font(theme::sans(tokens::FS_2, FontWeight::SemiBold))
            .color(c.text),
    );
    ui.label(
        egui::RichText::new(format!(
            "{} · {}",
            model.model_type.display_name(),
            model.level.display_name()
        ))
        .font(theme::mono(tokens::FS_0, FontWeight::Regular))
        .color(c.text_dim),
    );
    if !model.description.is_empty() {
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(&model.description)
                .font(theme::sans(tokens::FS_0, FontWeight::Regular))
                .color(c.text_dim),
        );
    }

    ui.add_space(6.0);
    ui.spacing_mut().item_spacing.y = 0.0;
    if let (Some(l_min), Some(l_max)) = (model.l_min, model.l_max) {
        kv_row(
            ui,
            "Length",
            &format!("{} – {}", format_si_value(l_min), format_si_value(l_max)),
        );
    }
    if let (Some(w_min), Some(w_max)) = (model.w_min, model.w_max) {
        kv_row(
            ui,
            "Width",
            &format!("{} – {}", format_si_value(w_min), format_si_value(w_max)),
        );
    }
    if let Some(vdd) = model.vdd {
        kv_row(ui, "Vdd", &format!("{vdd:.2} V"));
    }
    if let Some(vth0) = model.vth0 {
        kv_row(ui, "Vth0", &format!("{vth0:.3} V"));
    }
    if !model.parameters.is_empty() {
        kv_row(ui, "Parameters", &model.parameters.len().to_string());
    }

    // Process corners as chips — the selection rides into the result.
    if !lib.corners.is_empty() {
        ui.add_space(8.0);
        pane_caption(ui, "CORNER");
        let mut corners: Vec<&String> = lib.corners.keys().collect();
        corners.sort();
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = egui::vec2(4.0, 4.0);
            for corner in corners {
                let on = state.selected_corner.as_deref() == Some(corner.as_str());
                if chip(ui, corner, on).clicked() {
                    state.selected_corner = if on { None } else { Some(corner.clone()) };
                }
            }
        });
    }

    if let Some(ref path) = model.file_path {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(path.display().to_string())
                .font(theme::mono(tokens::FS_0, FontWeight::Regular))
                .color(c.text_faint),
        );
    }
}

/// Format a value with SI prefixes.
fn format_si_value(value: f64) -> String {
    let abs_value = value.abs();

    if abs_value == 0.0 {
        return "0".to_string();
    }

    let (scaled, suffix) = if abs_value >= 1e-3 {
        (value * 1e3, "mm")
    } else if abs_value >= 1e-6 {
        (value * 1e6, "µm")
    } else if abs_value >= 1e-9 {
        (value * 1e9, "nm")
    } else {
        (value * 1e12, "pm")
    };

    if (scaled.round() - scaled).abs() < 1e-9 {
        format!("{:.0}{}", scaled.round(), suffix)
    } else {
        format!("{:.2}{}", scaled, suffix)
    }
}

// =============================================================================
// Tests
// =============================================================================

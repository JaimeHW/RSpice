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

use crate::state::model_library::{
    DeviceModel, ModelLibrary, ModelLibraryManager, ModelType,
};
use egui::{Context, RichText, ScrollArea, Ui};
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
        if let Some(target) = self.type_filter {
            if model.model_type != target {
                return false;
            }
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
#[derive(Debug, Clone, PartialEq, Eq)]
#[derive(Default)]
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

    // Modal window
    egui::Window::new("Model Browser")
        .collapsible(false)
        .resizable(true)
        .default_size([800.0, 600.0])
        .min_width(700.0)
        .min_height(400.0)
        .show(ctx, |ui| {
            // Top toolbar
            ui.horizontal(|ui| {
                // Search box
                ui.label("🔍");
                let search_response = ui.add(
                    egui::TextEdit::singleline(&mut state.search_text)
                        .desired_width(200.0)
                        .hint_text("Search models..."),
                );
                if search_response.changed() {
                    // Real-time search
                }

                ui.separator();

                // Type filter
                ui.label("Type:");
                egui::ComboBox::from_id_salt("type_filter")
                    .selected_text(match state.type_filter {
                        None => "All",
                        Some(t) => t.display_name(),
                    })
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(state.type_filter.is_none(), "All")
                            .clicked()
                        {
                            state.type_filter = None;
                        }
                        for model_type in &[
                            ModelType::Nmos,
                            ModelType::Pmos,
                            ModelType::Npn,
                            ModelType::Pnp,
                            ModelType::Resistor,
                            ModelType::Capacitor,
                            ModelType::Diode,
                        ] {
                            if ui
                                .selectable_label(
                                    state.type_filter == Some(*model_type),
                                    model_type.display_name(),
                                )
                                .clicked()
                            {
                                state.type_filter = Some(*model_type);
                            }
                        }
                    });

                ui.separator();

                // Summary
                let total_models = manager.total_model_count();
                ui.label(
                    RichText::new(format!(
                        "{} libraries, {} models",
                        manager.library_count(),
                        total_models
                    ))
                    .weak(),
                );
            });

            ui.separator();

            // Three-pane layout using allocate_ui_with_layout pattern (same as Simulation Setup)
            // This ensures proper resizing without feedback loops
            let content_height = (ui.available_height() - 60.0).max(200.0);

            ui.horizontal(|ui| {
                // Left pane: Library tree
                ui.allocate_ui_with_layout(
                    egui::vec2(200.0, content_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.label(RichText::new("Libraries").strong());
                        ui.separator();

                        ScrollArea::vertical()
                            .id_salt("model_browser_libraries")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                render_library_tree(ui, state, manager);
                            });
                    },
                );

                ui.separator();

                // Center pane: Model list
                ui.allocate_ui_with_layout(
                    egui::vec2(280.0, content_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.label(RichText::new("Models").strong());
                        ui.separator();

                        ScrollArea::vertical()
                            .id_salt("model_browser_models")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                render_model_list(ui, state, manager);
                            });
                    },
                );

                ui.separator();

                // Right pane: Model details
                ui.allocate_ui_with_layout(
                    egui::vec2(ui.available_width().max(200.0), content_height),
                    egui::Layout::top_down(egui::Align::LEFT),
                    |ui| {
                        ui.label(RichText::new("Details").strong());
                        ui.separator();

                        ScrollArea::vertical()
                            .id_salt("model_browser_details")
                            .auto_shrink([false, false])
                            .show(ui, |ui| {
                                render_model_details(ui, state, manager);
                            });
                    },
                );
            });

            ui.separator();

            // Bottom buttons
            ui.horizontal(|ui| {
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Close/Cancel button
                    let button_text = if state.browse_only { "Close" } else { "Cancel" };
                    if ui.button(button_text).clicked() {
                        state.close();
                        result = ModelBrowserResult::Cancelled;
                    }

                    // Apply button (only shown if not in browse-only mode)
                    if !state.browse_only {
                        let can_apply = state.selected_model.is_some();
                        if ui
                            .add_enabled(can_apply, egui::Button::new("Apply"))
                            .clicked()
                        {
                            if let (Some(library), Some(model)) =
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
                    }
                });
            });
        });

    result
}

/// Render the library tree in the left pane.
fn render_library_tree(ui: &mut Ui, state: &mut ModelBrowserState, manager: &ModelLibraryManager) {
    if manager.library_count() == 0 {
        ui.label(RichText::new("No libraries loaded").weak().italics());
        return;
    }

    for lib in manager.libraries_sorted() {
        let is_selected = state.selected_library.as_deref() == Some(&lib.name);
        let matching_count = state.count_matching_models(lib);
        let corner_count = lib.corner_count();

        // Professional library display - clean vertical layout
        let response = ui.vertical(|ui| {
            // Library header with selection highlight
            let response = ui.selectable_label(
                is_selected,
                RichText::new(format!("📚 {}", lib.name)).strong(),
            );

            // Subtext: model count • corner count
            let subtext = if corner_count > 0 {
                format!("{} models • {} corners", matching_count, corner_count)
            } else {
                format!("{} models", matching_count)
            };
            ui.label(RichText::new(subtext).small().weak());

            response
        });

        // Click anywhere in the vertical group to select
        if response.inner.clicked() {
            state.select_library(&lib.name);
        }

        ui.add_space(4.0);
    }
}

/// Render the model list in the center pane.
fn render_model_list(ui: &mut Ui, state: &mut ModelBrowserState, manager: &ModelLibraryManager) {
    // If a library is selected, show its models
    if let Some(ref lib_name) = state.selected_library.clone() {
        if let Some(lib) = manager.get_library(lib_name) {
            let models = state.get_compatible_models(lib);

            if models.is_empty() {
                ui.label(RichText::new("No matching models").weak().italics());
                return;
            }

            // Sort models by name
            let mut sorted: Vec<_> = models;
            sorted.sort_by(|a, b| a.name.cmp(&b.name));

            for model in sorted {
                let is_selected = state.selected_model.as_deref() == Some(&model.name);
                let icon = model.model_type.icon();
                let label = format!("{} {}", icon, model.name);

                let response = ui.selectable_label(is_selected, label);
                if response.clicked() {
                    state.select_model(lib_name, &model.name);
                }

                // Show level as subtext
                response.on_hover_text(format!(
                    "{} - {}",
                    model.model_type.display_name(),
                    model.level.display_name()
                ));
            }
        }
    } else {
        // No library selected - show search results across all libraries
        if !state.search_text.is_empty() {
            let results = manager.search_models(&state.search_text);
            if results.is_empty() {
                ui.label(RichText::new("No results found").weak().italics());
            } else {
                for (lib, model) in results.iter().take(50) {
                    if !state.matches_filter(model) {
                        continue;
                    }

                    let is_selected = state.selected_library.as_deref() == Some(&lib.name)
                        && state.selected_model.as_deref() == Some(&model.name);

                    let icon = model.model_type.icon();
                    let label = format!("{} {}", icon, model.name);

                    let response = ui.selectable_label(is_selected, label);
                    if response.clicked() {
                        state.select_model(&lib.name, &model.name);
                    }

                    response.on_hover_text(format!("Library: {}", lib.name));
                }
            }
        } else {
            ui.label(RichText::new("Select a library or search").weak().italics());
        }
    }
}

/// Render model details in the right pane.
fn render_model_details(ui: &mut Ui, state: &ModelBrowserState, manager: &ModelLibraryManager) {
    if let (Some(ref lib_name), Some(ref model_name)) =
        (&state.selected_library, &state.selected_model)
    {
        if let Some(lib) = manager.get_library(lib_name) {
            if let Some(model) = lib.get_model(model_name) {
                // Model name header
                ui.label(RichText::new(&model.name).heading());
                ui.add_space(4.0);

                // Type and level
                ui.horizontal(|ui| {
                    ui.label(
                        RichText::new(format!(
                            "{} {}",
                            model.model_type.icon(),
                            model.model_type.display_name()
                        ))
                        .strong(),
                    );
                    ui.separator();
                    ui.label(format!("Level: {}", model.level.display_name()));
                });

                ui.add_space(8.0);

                // Description
                if !model.description.is_empty() {
                    ui.label(&model.description);
                    ui.add_space(8.0);
                }

                // Geometry limits (for transistors)
                if model.l_min.is_some() || model.w_min.is_some() {
                    ui.separator();
                    ui.label(RichText::new("Geometry Limits").strong());

                    egui::Grid::new("geometry_grid")
                        .num_columns(2)
                        .spacing([20.0, 4.0])
                        .show(ui, |ui| {
                            if let (Some(l_min), Some(l_max)) = (model.l_min, model.l_max) {
                                ui.label("Length:");
                                ui.label(format!(
                                    "{} - {}",
                                    format_si_value(l_min),
                                    format_si_value(l_max)
                                ));
                                ui.end_row();
                            }

                            if let (Some(w_min), Some(w_max)) = (model.w_min, model.w_max) {
                                ui.label("Width:");
                                ui.label(format!(
                                    "{} - {}",
                                    format_si_value(w_min),
                                    format_si_value(w_max)
                                ));
                                ui.end_row();
                            }
                        });
                }

                // Operating voltage
                if let Some(vdd) = model.vdd {
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.label("Vdd:");
                        ui.label(format!("{:.2} V", vdd));
                    });
                }

                // Threshold voltage
                if let Some(vth0) = model.vth0 {
                    ui.horizontal(|ui| {
                        ui.label("Vth0:");
                        ui.label(format!("{:.3} V", vth0));
                    });
                }

                // File path
                if let Some(ref path) = model.file_path {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.label(RichText::new("File").strong());
                    ui.label(RichText::new(path.display().to_string()).small().weak());
                }

                // Parameters summary
                if !model.parameters.is_empty() {
                    ui.add_space(8.0);
                    ui.separator();
                    ui.label(
                        RichText::new(format!("Parameters ({})", model.parameters.len())).strong(),
                    );
                }

                return;
            }
        }
    }

    ui.label(
        RichText::new("Select a model to view details")
            .weak()
            .italics(),
    );
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::model_library::ModelLevel;

    // =========================================================================
    // ModelBrowserState Tests
    // =========================================================================

    #[test]
    fn test_browser_state_default() {
        let state = ModelBrowserState::new();
        assert!(!state.open);
        assert!(state.search_text.is_empty());
        assert!(state.type_filter.is_none());
        assert!(state.selected_library.is_none());
        assert!(state.selected_model.is_none());
    }

    #[test]
    fn test_browser_state_open() {
        let mut state = ModelBrowserState::new();
        state.open();
        assert!(state.open);
    }

    #[test]
    fn test_browser_state_open_for_type() {
        let mut state = ModelBrowserState::new();
        state.open_for_type(ModelType::Nmos);

        assert!(state.open);
        assert_eq!(state.target_type, Some(ModelType::Nmos));
        assert_eq!(state.type_filter, Some(ModelType::Nmos));
    }

    #[test]
    fn test_browser_state_close() {
        let mut state = ModelBrowserState::new();
        state.open();
        state.close();
        assert!(!state.open);
    }

    #[test]
    fn test_browser_state_select_library() {
        let mut state = ModelBrowserState::new();
        state.select_library("tsmc180");

        assert_eq!(state.selected_library, Some("tsmc180".to_string()));
        assert!(state.selected_model.is_none());
    }

    #[test]
    fn test_browser_state_select_model() {
        let mut state = ModelBrowserState::new();
        state.select_model("tsmc180", "nmos_svt");

        assert_eq!(state.selected_library, Some("tsmc180".to_string()));
        assert_eq!(state.selected_model, Some("nmos_svt".to_string()));
    }

    #[test]
    fn test_browser_state_clear_selection() {
        let mut state = ModelBrowserState::new();
        state.select_model("lib", "model");
        state.clear_selection();

        assert!(state.selected_library.is_none());
        assert!(state.selected_model.is_none());
    }

    #[test]
    fn test_browser_state_toggle_library() {
        let mut state = ModelBrowserState::new();

        assert!(!state.is_expanded("lib1"));

        state.toggle_library("lib1");
        assert!(state.is_expanded("lib1"));

        state.toggle_library("lib1");
        assert!(!state.is_expanded("lib1"));
    }

    #[test]
    fn test_browser_state_get_model_reference() {
        let mut state = ModelBrowserState::new();

        // No selection
        assert!(state.get_model_reference().is_none());

        // Only library
        state.select_library("lib");
        assert!(state.get_model_reference().is_none());

        // Both selected
        state.select_model("lib", "model");
        assert_eq!(state.get_model_reference(), Some("lib:model".to_string()));
    }

    // =========================================================================
    // Filter Tests
    // =========================================================================

    #[test]
    fn test_matches_filter_no_filter() {
        let state = ModelBrowserState::new();
        let model = DeviceModel::new("nmos_svt", ModelType::Nmos);

        assert!(state.matches_filter(&model));
    }

    #[test]
    fn test_matches_filter_type_match() {
        let mut state = ModelBrowserState::new();
        state.type_filter = Some(ModelType::Nmos);

        let nmos = DeviceModel::new("nmos_svt", ModelType::Nmos);
        let pmos = DeviceModel::new("pmos_svt", ModelType::Pmos);

        assert!(state.matches_filter(&nmos));
        assert!(!state.matches_filter(&pmos));
    }

    #[test]
    fn test_matches_filter_search_name() {
        let mut state = ModelBrowserState::new();
        state.search_text = "svt".to_string();

        let matching = DeviceModel::new("nmos_svt", ModelType::Nmos);
        let non_matching = DeviceModel::new("nmos_hvt", ModelType::Nmos);

        assert!(state.matches_filter(&matching));
        assert!(!state.matches_filter(&non_matching));
    }

    #[test]
    fn test_matches_filter_search_case_insensitive() {
        let mut state = ModelBrowserState::new();
        state.search_text = "NMOS".to_string();

        let model = DeviceModel::new("nmos_svt", ModelType::Nmos);
        assert!(state.matches_filter(&model));
    }

    #[test]
    fn test_matches_filter_search_description() {
        let mut state = ModelBrowserState::new();
        state.search_text = "standard".to_string();

        let mut model = DeviceModel::new("nmos_svt", ModelType::Nmos);
        model.description = "Standard Vt transistor".to_string();

        assert!(state.matches_filter(&model));
    }

    #[test]
    fn test_matches_filter_combined() {
        let mut state = ModelBrowserState::new();
        state.type_filter = Some(ModelType::Nmos);
        state.search_text = "svt".to_string();

        let matching = DeviceModel::new("nmos_svt", ModelType::Nmos);
        let wrong_type = DeviceModel::new("pmos_svt", ModelType::Pmos);
        let wrong_name = DeviceModel::new("nmos_hvt", ModelType::Nmos);

        assert!(state.matches_filter(&matching));
        assert!(!state.matches_filter(&wrong_type));
        assert!(!state.matches_filter(&wrong_name));
    }

    // =========================================================================
    // Compatible Models Tests
    // =========================================================================

    #[test]
    fn test_get_compatible_models() {
        let mut library = ModelLibrary::new("test");
        library.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
        library.add_model(DeviceModel::new("nmos_hvt", ModelType::Nmos));
        library.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));

        let mut state = ModelBrowserState::new();
        state.type_filter = Some(ModelType::Nmos);

        let compatible = state.get_compatible_models(&library);
        assert_eq!(compatible.len(), 2);
    }

    #[test]
    fn test_count_matching_models() {
        let mut library = ModelLibrary::new("test");
        library.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
        library.add_model(DeviceModel::new("nmos_hvt", ModelType::Nmos));
        library.add_model(DeviceModel::new("pmos_svt", ModelType::Pmos));

        let state = ModelBrowserState::new();
        assert_eq!(state.count_matching_models(&library), 3);
    }

    // =========================================================================
    // ModelBrowserResult Tests
    // =========================================================================

    #[test]
    fn test_browser_result_default() {
        let result = ModelBrowserResult::default();
        assert_eq!(result, ModelBrowserResult::None);
    }

    #[test]
    fn test_browser_result_cancelled() {
        let result = ModelBrowserResult::Cancelled;
        assert_eq!(result, ModelBrowserResult::Cancelled);
    }

    #[test]
    fn test_browser_result_selected() {
        let result = ModelBrowserResult::Selected {
            library: "lib".to_string(),
            model: "model".to_string(),
            corner: Some("tt".to_string()),
        };

        if let ModelBrowserResult::Selected {
            library,
            model,
            corner,
        } = result
        {
            assert_eq!(library, "lib");
            assert_eq!(model, "model");
            assert_eq!(corner, Some("tt".to_string()));
        } else {
            panic!("Expected Selected result");
        }
    }

    // =========================================================================
    // SI Value Format Tests
    // =========================================================================

    #[test]
    fn test_format_si_value_zero() {
        assert_eq!(format_si_value(0.0), "0");
    }

    #[test]
    fn test_format_si_value_nanometers() {
        assert_eq!(format_si_value(60e-9), "60nm");
    }

    #[test]
    fn test_format_si_value_micrometers() {
        assert_eq!(format_si_value(1e-6), "1µm");
    }

    #[test]
    fn test_format_si_value_millimeters() {
        assert_eq!(format_si_value(1e-3), "1mm");
    }

    #[test]
    fn test_format_si_value_fractional() {
        let s = format_si_value(65e-9);
        assert!(s.contains("nm"));
    }

    // =========================================================================
    // Integration Tests
    // =========================================================================

    #[test]
    fn test_browser_workflow() {
        let mut manager = ModelLibraryManager::new();

        // Add a library with models
        let mut lib = ModelLibrary::new("tsmc180").with_technology("TSMC", "180nm");
        lib.add_model(
            DeviceModel::new("nmos_3p3", ModelType::Nmos)
                .with_level(ModelLevel::Bsim4)
                .with_geometry(180e-9, 10e-6, 360e-9, 100e-6),
        );
        lib.add_model(
            DeviceModel::new("pmos_3p3", ModelType::Pmos)
                .with_level(ModelLevel::Bsim4)
                .with_geometry(180e-9, 10e-6, 360e-9, 100e-6),
        );
        manager.add_library(lib);

        // Simulate browser workflow
        let mut state = ModelBrowserState::new();
        state.open_for_type(ModelType::Nmos);

        assert!(state.open);
        assert_eq!(state.type_filter, Some(ModelType::Nmos));

        // Select library
        state.select_library("tsmc180");
        assert_eq!(state.selected_library, Some("tsmc180".to_string()));

        // Get compatible models (only NMOS)
        let lib = manager.get_library("tsmc180").unwrap();
        let compatible = state.get_compatible_models(lib);
        assert_eq!(compatible.len(), 1);
        assert_eq!(compatible[0].name, "nmos_3p3");

        // Select model
        state.select_model("tsmc180", "nmos_3p3");
        assert_eq!(
            state.get_model_reference(),
            Some("tsmc180:nmos_3p3".to_string())
        );
    }

    #[test]
    fn test_browser_search_across_libraries() {
        let mut manager = ModelLibraryManager::new();

        let mut lib1 = ModelLibrary::new("lib1");
        lib1.add_model(DeviceModel::new("nmos_svt", ModelType::Nmos));
        manager.add_library(lib1);

        let mut lib2 = ModelLibrary::new("lib2");
        lib2.add_model(DeviceModel::new("nmos_hvt", ModelType::Nmos));
        lib2.add_model(DeviceModel::new("nmos_lvt", ModelType::Nmos));
        manager.add_library(lib2);

        // Search for "nmos"
        let results = manager.search_models("nmos");
        assert_eq!(results.len(), 3);

        // Search for "hvt"
        let results = manager.search_models("hvt");
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_browser_empty_library() {
        let library = ModelLibrary::new("empty");
        let state = ModelBrowserState::new();

        assert_eq!(state.count_matching_models(&library), 0);
        assert!(state.get_compatible_models(&library).is_empty());
    }

    #[test]
    fn test_browser_multiple_libraries() {
        let mut manager = ModelLibraryManager::new();

        manager.add_library(ModelLibrary::new("lib1"));
        manager.add_library(ModelLibrary::new("lib2"));
        manager.add_library(ModelLibrary::new("lib3"));

        assert_eq!(manager.library_count(), 3);
        assert_eq!(manager.libraries_sorted().len(), 3);
    }
}

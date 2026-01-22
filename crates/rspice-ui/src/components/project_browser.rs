//! Project Browser Component - Virtuoso Library Manager Style
//!
//! A professional tree-view component for browsing the Library/Cell/View
//! hierarchy. Matches the Cadence Virtuoso Library Manager paradigm.

use dioxus::prelude::*;
use std::collections::HashSet;

use crate::state::hierarchy::{HierarchyManager, ViewType};
use crate::theme::Theme;

//=============================================================================
// Tree State
//=============================================================================

/// Tracks which tree nodes are expanded
#[derive(Clone, Default)]
pub struct TreeExpansionState {
    expanded: HashSet<String>,
}

impl TreeExpansionState {
    pub fn is_expanded(&self, path: &str) -> bool {
        self.expanded.contains(path)
    }

    pub fn toggle(&mut self, path: &str) {
        if self.expanded.contains(path) {
            self.expanded.remove(path);
        } else {
            self.expanded.insert(path.to_string());
        }
    }

    pub fn expand_all(&mut self, manager: &HierarchyManager) {
        for (lib_name, library) in &manager.libraries {
            self.expanded.insert(lib_name.clone());
            for cell_name in library.cells.keys() {
                self.expanded.insert(format!("{}/{}", lib_name, cell_name));
            }
        }
    }

    pub fn collapse_all(&mut self) {
        self.expanded.clear();
    }
}

//=============================================================================
// Project Browser Component
//=============================================================================

#[component]
pub fn ProjectBrowser() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let mut tree_state = use_signal(TreeExpansionState::default);
    let hierarchy: Signal<HierarchyManager> = use_context();
    let mut selected_path = use_signal(|| String::new());
    let mut search_filter = use_signal(|| String::new());

    let mgr = hierarchy.read();

    // Compute styles
    let header_style = format!(
        "display: flex; align-items: center; justify-content: space-between; \
         padding: 8px 12px; border-bottom: 1px solid {}; background: {};",
        th.border(),
        th.surface()
    );

    let search_style = format!(
        "width: 100%; padding: 6px 8px; background: {}; border: 1px solid {}; \
         border-radius: 4px; color: {}; font-size: 12px; outline: none;",
        th.bg_primary(),
        th.border(),
        th.text_primary()
    );

    let footer_style = format!(
        "padding: 6px 12px; border-top: 1px solid {}; background: {}; \
         font-size: 11px; color: {};",
        th.border(),
        th.surface(),
        th.text_muted()
    );

    let status = format!(
        "{} libraries, {} cells",
        mgr.libraries.len(),
        mgr.libraries.values().map(|l| l.cells.len()).sum::<usize>()
    );

    rsx! {
        div {
            class: "project-browser",
            style: "display: flex; flex-direction: column; height: 100%; user-select: none;",

            // Header
            div {
                style: "{header_style}",

                span {
                    style: "font-weight: 600; font-size: 12px;",
                    "📁 Libraries"
                }

                div {
                    style: "display: flex; gap: 4px;",

                    button {
                        style: "padding: 2px 6px; background: transparent; border: none; cursor: pointer; font-size: 11px;",
                        title: "Expand All",
                        onclick: move |_| {
                            let mgr = hierarchy.read();
                            tree_state.write().expand_all(&mgr);
                        },
                        "⊞"
                    }

                    button {
                        style: "padding: 2px 6px; background: transparent; border: none; cursor: pointer; font-size: 11px;",
                        title: "Collapse All",
                        onclick: move |_| {
                            tree_state.write().collapse_all();
                        },
                        "⊟"
                    }
                }
            }

            // Search box
            div {
                style: "padding: 8px; border-bottom: 1px solid {th.border()};",

                input {
                    r#type: "text",
                    placeholder: "🔍 Search cells...",
                    value: "{search_filter}",
                    oninput: move |evt| search_filter.set(evt.value().clone()),
                    style: "{search_style}",
                }
            }

            // Tree view - render library content inline (avoiding complex type passing)
            div {
                style: "flex: 1; overflow-y: auto; padding: 4px 0;",

                // Pre-filter libraries based on search (filter is applied inside)
                for (lib_name, library) in mgr.libraries.iter() {
                    {
                        let lib_path = lib_name.clone();
                        let is_lib_expanded = tree_state.read().is_expanded(&lib_path);
                        let is_lib_selected = *selected_path.read() == lib_path;
                        let lib_cell_count = library.cells.len();

                        let filter = search_filter.read().clone();
                        let visible_cells: Vec<_> = library.cells.iter()
                            .filter(|(name, _)| {
                                filter.is_empty() ||
                                name.to_lowercase().contains(&filter.to_lowercase())
                            })
                            .collect();

                        // Skip rendering if no cells match search (use empty element)
                        let should_render = filter.is_empty() || !visible_cells.is_empty();

                        let lib_bg = if is_lib_selected { th.surface_hover().to_string() } else { "transparent".to_string() };
                        let lib_row_style = format!(
                            "display: flex; align-items: center; padding: 4px 8px; cursor: pointer; background: {};",
                            lib_bg
                        );

                        rsx! {
                            if should_render {
                                div {
                                    key: "{lib_name}",
                                    class: "library-node",

                                    // Library row
                                    div {
                                        style: "{lib_row_style}",
                                    onmousedown: {
                                        let path = lib_path.clone();
                                        move |_| selected_path.set(path.clone())
                                    },
                                    ondoubleclick: {
                                        let path = lib_path.clone();
                                        move |_| tree_state.write().toggle(&path)
                                    },

                                    span {
                                        style: "width: 16px; font-size: 10px; text-align: center; cursor: pointer;",
                                        onclick: {
                                            let path = lib_path.clone();
                                            move |evt| {
                                                evt.stop_propagation();
                                                tree_state.write().toggle(&path);
                                            }
                                        },
                                        if is_lib_expanded { "▼" } else { "▶" }
                                    }

                                    span { style: "margin-right: 6px; font-size: 14px;", "📚" }

                                    span {
                                        style: "flex: 1; font-size: 12px; font-weight: 500;",
                                        "{lib_name}"
                                    }

                                    span {
                                        style: "padding: 1px 6px; background: {th.surface()}; border-radius: 8px; font-size: 10px;",
                                        "{lib_cell_count}"
                                    }
                                }

                                // Cells (if expanded)
                                if is_lib_expanded {
                                    div {
                                        style: "margin-left: 12px;",

                                        for (cell_name, cell) in visible_cells.iter() {
                                            {
                                                let cell_path = format!("{}/{}", lib_name, cell_name);
                                                let is_cell_expanded = tree_state.read().is_expanded(&cell_path);
                                                let is_cell_selected = *selected_path.read() == cell_path;
                                                let has_views = !cell.views.is_empty();

                                                let cell_bg = if is_cell_selected { th.surface_hover().to_string() } else { "transparent".to_string() };
                                                let cell_row_style = format!(
                                                    "display: flex; align-items: center; padding: 3px 8px; cursor: pointer; background: {}; border-radius: 3px;",
                                                    cell_bg
                                                );

                                                let arrow_vis = if has_views { "visible" } else { "hidden" };
                                                let arrow_style = format!("width: 14px; font-size: 9px; text-align: center; visibility: {};", arrow_vis);

                                                // Cell icon based on views
                                                let cell_icon = if cell.views.contains_key("schematic") { "📄" }
                                                    else if cell.views.contains_key("symbol") { "🔷" }
                                                    else if cell.views.contains_key("layout") { "📐" }
                                                    else { "📦" };

                                                rsx! {
                                                    div {
                                                        key: "{cell_path}",
                                                        class: "cell-node",

                                                        div {
                                                            style: "{cell_row_style}",
                                                            onmousedown: {
                                                                let path = cell_path.clone();
                                                                move |_| selected_path.set(path.clone())
                                                            },
                                                            ondoubleclick: {
                                                                let path = cell_path.clone();
                                                                let has = has_views;
                                                                move |_| {
                                                                    if has {
                                                                        tree_state.write().toggle(&path);
                                                                    }
                                                                }
                                                            },

                                                            span {
                                                                style: "{arrow_style}",
                                                                onclick: {
                                                                    let path = cell_path.clone();
                                                                    move |evt| {
                                                                        evt.stop_propagation();
                                                                        tree_state.write().toggle(&path);
                                                                    }
                                                                },
                                                                if is_cell_expanded { "▼" } else { "▶" }
                                                            }

                                                            span { style: "margin-right: 5px; font-size: 12px;", "{cell_icon}" }

                                                            span {
                                                                style: "flex: 1; font-size: 12px;",
                                                                "{cell_name}"
                                                            }
                                                        }

                                                        // Views (if expanded)
                                                        if is_cell_expanded {
                                                            div {
                                                                style: "margin-left: 20px;",

                                                                for (view_name, view) in cell.views.iter() {
                                                                    {
                                                                        let view_path = format!("{}/{}", cell_path, view_name);
                                                                        let is_view_selected = *selected_path.read() == view_path;

                                                                        let view_bg = if is_view_selected { th.surface_hover().to_string() } else { "transparent".to_string() };
                                                                        let view_row_style = format!(
                                                                            "display: flex; align-items: center; padding: 2px 8px; cursor: pointer; background: {}; border-radius: 3px;",
                                                                            view_bg
                                                                        );

                                                                        let view_icon = match view.view_type {
                                                                            ViewType::Schematic => "📄",
                                                                            ViewType::Symbol => "🔷",
                                                                            ViewType::Layout => "📐",
                                                                            ViewType::Netlist => "📝",
                                                                            ViewType::Documentation => "📖",
                                                                        };

                                                                        rsx! {
                                                                            div {
                                                                                key: "{view_path}",
                                                                                style: "{view_row_style}",
                                                                                onmousedown: {
                                                                                    let path = view_path.clone();
                                                                                    move |_| selected_path.set(path.clone())
                                                                                },
                                                                                ondoubleclick: {
                                                                                    let path = view_path.clone();
                                                                                    move |_| log::info!("Open view: {}", path)
                                                                                },

                                                                                span { style: "margin-right: 5px; font-size: 11px;", "{view_icon}" }
                                                                                span { style: "font-size: 11px; color: {th.text_secondary()};", "{view_name}" }
                                                                            }
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                }
                            }
                        }
                    }
                }

                if mgr.libraries.is_empty() {
                    div {
                        style: "padding: 20px; text-align: center; font-size: 12px;",
                        "No libraries loaded"

                        div {
                            style: "margin-top: 12px;",
                            button {
                                style: "padding: 6px 12px; background: {th.accent_primary()}; color: white; border: none; border-radius: 4px; cursor: pointer; font-size: 12px;",
                                "Create Library"
                            }
                        }
                    }
                }
            }

            // Footer
            div {
                style: "{footer_style}",
                "{status}"
            }
        }
    }
}

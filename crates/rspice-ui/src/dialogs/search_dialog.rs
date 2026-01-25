//! Search Dialog
//!
//! Find and search dialog for locating components, nets, and other schematic elements.
//! Essential for navigating large designs efficiently.

use dioxus::prelude::*;

use crate::theme::Theme;

//=============================================================================
// Search Types
//=============================================================================

/// Type of item to search for
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SearchType {
    /// Search everything
    #[default]
    All,
    /// Search component names (R1, C1, etc.)
    Components,
    /// Search net names
    Nets,
    /// Search parameter values
    Values,
}

impl SearchType {
    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::All => "All",
            Self::Components => "Components",
            Self::Nets => "Nets",
            Self::Values => "Values",
        }
    }

    /// Get all search types
    pub fn all() -> &'static [SearchType] {
        &[Self::All, Self::Components, Self::Nets, Self::Values]
    }
}

/// A search result item
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    /// Unique identifier
    pub id: usize,
    /// Display name
    pub name: String,
    /// Type of result
    pub result_type: SearchResultType,
    /// Additional context (e.g., net name, value)
    pub context: String,
    /// Location in schematic
    pub x: f64,
    pub y: f64,
}

/// Type of search result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SearchResultType {
    Component,
    Net,
    NetLabel,
    Wire,
}

impl SearchResultType {
    /// Get icon for UI display
    pub fn icon(&self) -> &'static str {
        match self {
            Self::Component => "🔲",
            Self::Net => "🔗",
            Self::NetLabel => "🏷️",
            Self::Wire => "➖",
        }
    }

    /// Get display name
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Component => "Component",
            Self::Net => "Net",
            Self::NetLabel => "Net Label",
            Self::Wire => "Wire",
        }
    }
}

//=============================================================================
// Search Dialog Component
//=============================================================================

/// Props for the search dialog
#[derive(Props, Clone, PartialEq)]
pub struct SearchDialogProps {
    /// Whether the dialog is visible
    pub visible: bool,
    /// Handler for closing the dialog
    pub on_close: EventHandler<()>,
    /// Handler for selecting a result (navigates to it)
    pub on_select: EventHandler<SearchResult>,
    /// Search handler - called with query to get results
    pub on_search: EventHandler<String>,
    /// Search results
    #[props(default)]
    pub results: Vec<SearchResult>,
}

/// Search/find dialog component
#[component]
pub fn SearchDialog(props: SearchDialogProps) -> Element {
    if !props.visible {
        return rsx! {};
    }

    let theme = use_context::<Signal<Theme>>();
    let mut query = use_signal(String::new);
    let mut search_type = use_signal(|| SearchType::All);
    let mut selected_index = use_signal(|| 0usize);

    // Filter results based on search type - clone to avoid lifetime issues
    let current_search_type = search_type();
    let filtered_results: Vec<SearchResult> = props
        .results
        .iter()
        .filter(|r| match current_search_type {
            SearchType::All => true,
            SearchType::Components => r.result_type == SearchResultType::Component,
            SearchType::Nets => {
                r.result_type == SearchResultType::Net
                    || r.result_type == SearchResultType::NetLabel
            }
            SearchType::Values => false, // Would need value field
        })
        .cloned()
        .collect();

    rsx! {
        // Backdrop
        div {
            class: "search-dialog-backdrop",
            style: "
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.5);
                display: flex;
                align-items: flex-start;
                justify-content: center;
                padding-top: 100px;
                z-index: 10000;
            ",
            onclick: move |_| props.on_close.call(()),

            // Dialog
            div {
                class: "search-dialog",
                style: format!(
                    "
                    background: {};
                    border: 1px solid {};
                    border-radius: 12px;
                    width: 560px;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
                    overflow: hidden;
                    ",
                    theme.read().bg_secondary(),
                    theme.read().border()
                ),
                onclick: move |e| e.stop_propagation(),

                // Search input
                div {
                    style: format!(
                        "
                        padding: 16px;
                        border-bottom: 1px solid {};
                        ",
                        theme.read().border_subtle()
                    ),

                    div {
                        style: "display: flex; gap: 12px;",

                        // Search icon + input
                        div {
                            style: format!(
                                "
                                flex: 1;
                                display: flex;
                                align-items: center;
                                gap: 12px;
                                background: {};
                                border-radius: 8px;
                                padding: 8px 16px;
                                ",
                                theme.read().bg_tertiary()
                            ),

                            span { style: "font-size: 18px;", "🔍" }

                            input {
                                r#type: "text",
                                placeholder: "Search components, nets...",
                                value: "{query}",
                                autofocus: true,
                                style: format!(
                                    "
                                    flex: 1;
                                    background: none;
                                    border: none;
                                    outline: none;
                                    color: {};
                                    font-size: 16px;
                                    ",
                                    theme.read().text_primary()
                                ),
                                oninput: move |e| {
                                    query.set(e.value().clone());
                                    props.on_search.call(e.value());
                                    selected_index.set(0);
                                },
                                onkeydown: move |e| {
                                    use dioxus::prelude::Key;
                                    match e.key() {
                                        Key::Escape => props.on_close.call(()),
                                        Key::ArrowDown => {
                                            let max = filtered_results.len().saturating_sub(1);
                                            selected_index.set((selected_index() + 1).min(max));
                                        }
                                        Key::ArrowUp => {
                                            selected_index.set(selected_index().saturating_sub(1));
                                        }
                                        Key::Enter => {
                                            if let Some(result) = filtered_results.get(selected_index()) {
                                                props.on_select.call(result.clone());
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                        }
                    }

                    // Filter tabs
                    div {
                        style: "display: flex; gap: 8px; margin-top: 12px;",

                        for stype in SearchType::all() {
                            button {
                                style: format!(
                                    "
                                    padding: 6px 12px;
                                    border-radius: 6px;
                                    border: none;
                                    font-size: 12px;
                                    cursor: pointer;
                                    transition: all 0.15s ease;
                                    background: {};
                                    color: {};
                                    ",
                                    if search_type() == *stype {
                                        theme.read().accent_primary()
                                    } else {
                                        theme.read().surface()
                                    },
                                    if search_type() == *stype {
                                        "white"
                                    } else {
                                        theme.read().text_secondary()
                                    }
                                ),
                                onclick: {
                                    let stype = *stype;
                                    move |_| search_type.set(stype)
                                },
                                "{stype.display_name()}"
                            }
                        }
                    }
                }

                // Results list
                div {
                    style: format!(
                        "
                        max-height: 400px;
                        overflow-y: auto;
                        ",
                    ),

                    if filtered_results.is_empty() {
                        if query().is_empty() {
                            div {
                                style: format!(
                                    "
                                    padding: 40px;
                                    text-align: center;
                                    color: {};
                                    ",
                                    theme.read().text_muted()
                                ),
                                "Type to search..."
                            }
                        } else {
                            div {
                                style: format!(
                                    "
                                    padding: 40px;
                                    text-align: center;
                                    color: {};
                                    ",
                                    theme.read().text_muted()
                                ),
                                "No results found"
                            }
                        }
                    } else {
                        for (idx, result) in filtered_results.iter().enumerate() {
                            SearchResultRow {
                                result: result.clone(),
                                selected: idx == selected_index(),
                                on_click: {
                                    let result = result.clone();
                                    move |_| props.on_select.call(result.clone())
                                }
                            }
                        }
                    }
                }

                // Footer with hint
                div {
                    style: format!(
                        "
                        padding: 12px 16px;
                        border-top: 1px solid {};
                        font-size: 11px;
                        color: {};
                        display: flex;
                        gap: 16px;
                        ",
                        theme.read().border_subtle(),
                        theme.read().text_muted()
                    ),

                    span { "↑↓ Navigate" }
                    span { "↵ Select" }
                    span { "Esc Close" }
                }
            }
        }
    }
}

//=============================================================================
// Helper Components
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct SearchResultRowProps {
    result: SearchResult,
    selected: bool,
    on_click: EventHandler<MouseEvent>,
}

#[component]
fn SearchResultRow(props: SearchResultRowProps) -> Element {
    let theme = use_context::<Signal<Theme>>();

    rsx! {
        div {
            style: format!(
                "
                display: flex;
                align-items: center;
                gap: 12px;
                padding: 12px 16px;
                cursor: pointer;
                transition: background 0.1s ease;
                background: {};
                ",
                if props.selected {
                    theme.read().surface_hover()
                } else {
                    "transparent"
                }
            ),
            onclick: move |e| props.on_click.call(e),

            // Icon
            span {
                style: "font-size: 16px;",
                "{props.result.result_type.icon()}"
            }

            // Name and context
            div {
                style: "flex: 1;",

                div {
                    style: format!(
                        "
                        font-size: 14px;
                        font-weight: 500;
                        color: {};
                        ",
                        theme.read().text_primary()
                    ),
                    "{props.result.name}"
                }

                if !props.result.context.is_empty() {
                    div {
                        style: format!(
                            "
                            font-size: 12px;
                            color: {};
                            margin-top: 2px;
                            ",
                            theme.read().text_muted()
                        ),
                        "{props.result.context}"
                    }
                }
            }

            // Type badge
            span {
                style: format!(
                    "
                    font-size: 11px;
                    padding: 2px 8px;
                    border-radius: 4px;
                    background: {};
                    color: {};
                    ",
                    theme.read().surface(),
                    theme.read().text_secondary()
                ),
                "{props.result.result_type.display_name()}"
            }
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search_type_all() {
        let all = SearchType::all();
        assert_eq!(all.len(), 4);
    }

    #[test]
    fn test_search_type_display_names() {
        for stype in SearchType::all() {
            assert!(!stype.display_name().is_empty());
        }
    }

    #[test]
    fn test_search_result_type_icons() {
        assert!(!SearchResultType::Component.icon().is_empty());
        assert!(!SearchResultType::Net.icon().is_empty());
        assert!(!SearchResultType::NetLabel.icon().is_empty());
        assert!(!SearchResultType::Wire.icon().is_empty());
    }

    #[test]
    fn test_search_result_creation() {
        let result = SearchResult {
            id: 1,
            name: "R1".to_string(),
            result_type: SearchResultType::Component,
            context: "1kΩ".to_string(),
            x: 100.0,
            y: 200.0,
        };

        assert_eq!(result.name, "R1");
        assert_eq!(result.result_type, SearchResultType::Component);
    }
}

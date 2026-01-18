//! Tab Bar Component for Multi-Document Interface
//!
//! Professional tab bar with document tabs, close buttons, and new document button.
//! Provides visual feedback for dirty state and active tab.

use dioxus::prelude::*;

use crate::state::DocumentManager;
use crate::theme::Theme;

/// Document tab bar component for switching between open documents
#[component]
pub fn DocumentTabBar(
    doc_manager: Signal<DocumentManager>,
    on_tab_change: EventHandler<usize>,
    on_tab_close: EventHandler<usize>,
    on_new_document: EventHandler<()>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let docs = doc_manager.read();

    rsx! {
        div {
            class: "tab-bar",
            style: "
                display: flex;
                align-items: center;
                background: {th.bg_secondary()};
                border-bottom: 1px solid {th.border()};
                height: 32px;
                padding: 0 4px;
                gap: 2px;
                overflow-x: auto;
                flex-shrink: 0;
            ",

            // Document tabs
            for (idx, doc) in docs.documents.iter().enumerate() {
                Tab {
                    key: "{doc.id}",
                    title: doc.display_title(),
                    is_active: idx == docs.active_index,
                    is_dirty: doc.is_dirty,
                    on_click: move |_| on_tab_change.call(idx),
                    on_close: move |_| on_tab_close.call(idx),
                }
            }

            // New document button
            button {
                class: "new-tab-btn",
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    width: 24px;
                    height: 24px;
                    margin-left: 4px;
                    background: transparent;
                    border: 1px solid transparent;
                    border-radius: 4px;
                    color: {th.text_muted()};
                    font-size: 16px;
                    cursor: pointer;
                    transition: all 0.15s ease;
                ",
                title: "New Document (Ctrl+N)",
                onclick: move |_| on_new_document.call(()),
                "+"
            }
        }
    }
}

/// Individual tab component
#[component]
fn Tab(
    title: String,
    is_active: bool,
    is_dirty: bool,
    on_click: EventHandler<MouseEvent>,
    on_close: EventHandler<MouseEvent>,
) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let bg: String = if is_active {
        th.bg_primary().to_string()
    } else if *hovered.read() {
        th.bg_tertiary().to_string()
    } else {
        "transparent".to_string()
    };

    let text_color = if is_active {
        th.text_primary()
    } else {
        th.text_muted()
    };

    let font_weight = if is_active { "500" } else { "400" };

    let border_bottom = if is_active {
        format!("2px solid {}", th.accent_primary())
    } else {
        "2px solid transparent".to_string()
    };

    rsx! {
        div {
            class: "tab",
            style: "
                display: flex;
                align-items: center;
                gap: 6px;
                padding: 0 8px;
                height: 28px;
                background: {bg};
                border-bottom: {border_bottom};
                border-radius: 4px 4px 0 0;
                cursor: pointer;
                user-select: none;
                transition: background 0.15s ease;
                max-width: 200px;
            ",
            onclick: on_click,
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),

            // Document title
            span {
                style: "
                    color: {text_color};
                    font-size: {Theme::FONT_SIZE_SM};
                    font-weight: {font_weight};
                    white-space: nowrap;
                    overflow: hidden;
                    text-overflow: ellipsis;
                ",
                "{title}"
            }

            // Close button (visible on hover or if dirty)
            if *hovered.read() || is_dirty {
                button {
                    class: "tab-close",
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        width: 16px;
                        height: 16px;
                        background: transparent;
                        border: none;
                        border-radius: 3px;
                        color: {th.text_muted()};
                        font-size: 12px;
                        cursor: pointer;
                        padding: 0;
                        line-height: 1;
                        transition: all 0.1s ease;
                    ",
                    title: "Close",
                    onclick: move |evt| {
                        evt.stop_propagation();
                        on_close.call(evt);
                    },
                    if is_dirty && !*hovered.read() {
                        // Show dot indicator for dirty state when not hovered
                        "●"
                    } else {
                        "×"
                    }
                }
            }
        }
    }
}

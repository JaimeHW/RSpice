//! Context Menu Component
//!
//! Right-click context menus for schematic editor.

use crate::theme::Theme;
use dioxus::prelude::*;

/// Context menu item
#[derive(Clone, PartialEq)]
pub struct MenuItem {
    pub label: String,
    pub shortcut: Option<String>,
    pub enabled: bool,
    pub action: MenuAction,
}

/// Menu action types
#[derive(Clone, Copy, PartialEq)]
pub enum MenuAction {
    Undo,
    Redo,
    Cut,
    Copy,
    Paste,
    Delete,
    Rotate,
    EditValue,
    AddJunction,
    SelectAll,
    DeselectAll,
    ZoomIn,
    ZoomOut,
    ZoomFit,
}

impl MenuItem {
    pub fn new(label: impl Into<String>, action: MenuAction) -> Self {
        Self {
            label: label.into(),
            shortcut: None,
            enabled: true,
            action,
        }
    }

    pub fn with_shortcut(mut self, shortcut: impl Into<String>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.enabled = false;
        self
    }

    pub fn enabled_if(mut self, condition: bool) -> Self {
        self.enabled = condition;
        self
    }
}

/// Separator between menu groups
pub struct MenuSeparator;

/// Context menu props
#[derive(Props, Clone, PartialEq)]
pub struct ContextMenuProps {
    /// Screen position (x, y)
    pub position: (f64, f64),
    /// Menu items grouped by separator
    pub items: Vec<MenuItem>,
    /// Called when an action is selected
    pub on_action: EventHandler<MenuAction>,
    /// Called when menu should close
    pub on_close: EventHandler<()>,
}

/// Context menu component
#[component]
pub fn ContextMenu(props: ContextMenuProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let (x, y) = props.position;

    rsx! {
        // Backdrop to catch clicks outside
        div {
            style: "position: fixed; inset: 0; z-index: 999;",
            onclick: move |_| props.on_close.call(()),
            // Prevent browser context menu on backdrop
            oncontextmenu: move |evt| {
                evt.prevent_default();
                props.on_close.call(());
            },
        }

        // Menu container
        div {
            style: "
                position: fixed;
                left: {x}px;
                top: {y}px;
                z-index: 1000;
                min-width: 180px;
                background: {th.bg_secondary()};
                border: 1px solid {th.border()};
                border-radius: 6px;
                padding: 4px 0;
                box-shadow: 0 4px 12px rgba(0,0,0,0.3);
            ",
            // Prevent browser context menu on menu itself
            oncontextmenu: move |evt| {
                evt.prevent_default();
            },

            for item in props.items.iter() {
                ContextMenuItem {
                    item: item.clone(),
                    on_click: move |action| {
                        props.on_action.call(action);
                        props.on_close.call(());
                    },
                }
            }
        }
    }
}

#[component]
fn ContextMenuItem(item: MenuItem, on_click: EventHandler<MenuAction>) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    let (text_color, cursor) = if item.enabled {
        (th.text_primary(), "pointer")
    } else {
        (th.text_muted(), "default")
    };

    rsx! {
        div {
            style: "
                display: flex;
                justify-content: space-between;
                align-items: center;
                padding: 6px 12px;
                font-size: 13px;
                color: {text_color};
                cursor: {cursor};
            ",
            onmouseenter: move |_| {},
            onclick: move |_| {
                if item.enabled {
                    on_click.call(item.action);
                }
            },

            span { "{item.label}" }

            if let Some(ref shortcut) = item.shortcut {
                span {
                    style: "font-size: 11px; color: {th.text_muted()}; margin-left: 20px;",
                    "{shortcut}"
                }
            }
        }
    }
}

/// Separator line
#[component]
pub fn ContextMenuSeparator() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "height: 1px; background: {th.border()}; margin: 4px 8px;",
        }
    }
}

/// Build standard schematic context menu
pub fn schematic_context_menu(
    has_selection: bool,
    has_clipboard: bool,
    can_undo: bool,
    can_redo: bool,
) -> Vec<MenuItem> {
    vec![
        MenuItem::new("Undo", MenuAction::Undo)
            .with_shortcut("Ctrl+Z")
            .enabled_if(can_undo),
        MenuItem::new("Redo", MenuAction::Redo)
            .with_shortcut("Ctrl+Y")
            .enabled_if(can_redo),
        // Separator would go here
        MenuItem::new("Cut", MenuAction::Cut)
            .with_shortcut("Ctrl+X")
            .enabled_if(has_selection),
        MenuItem::new("Copy", MenuAction::Copy)
            .with_shortcut("Ctrl+C")
            .enabled_if(has_selection),
        MenuItem::new("Paste", MenuAction::Paste)
            .with_shortcut("Ctrl+V")
            .enabled_if(has_clipboard),
        MenuItem::new("Delete", MenuAction::Delete)
            .with_shortcut("Del")
            .enabled_if(has_selection),
        // Separator
        MenuItem::new("Rotate", MenuAction::Rotate)
            .with_shortcut("R")
            .enabled_if(has_selection),
        MenuItem::new("Edit Value...", MenuAction::EditValue).enabled_if(has_selection),
    ]
}

/// Build wire-specific context menu
pub fn wire_context_menu(can_undo: bool, can_redo: bool) -> Vec<MenuItem> {
    vec![
        MenuItem::new("Undo", MenuAction::Undo)
            .with_shortcut("Ctrl+Z")
            .enabled_if(can_undo),
        MenuItem::new("Redo", MenuAction::Redo)
            .with_shortcut("Ctrl+Y")
            .enabled_if(can_redo),
        MenuItem::new("Delete Wire", MenuAction::Delete).with_shortcut("Del"),
        MenuItem::new("Add Junction", MenuAction::AddJunction),
    ]
}

/// Build canvas (empty space) context menu
pub fn canvas_context_menu(has_clipboard: bool, can_undo: bool, can_redo: bool) -> Vec<MenuItem> {
    vec![
        MenuItem::new("Undo", MenuAction::Undo)
            .with_shortcut("Ctrl+Z")
            .enabled_if(can_undo),
        MenuItem::new("Redo", MenuAction::Redo)
            .with_shortcut("Ctrl+Y")
            .enabled_if(can_redo),
        MenuItem::new("Paste", MenuAction::Paste)
            .with_shortcut("Ctrl+V")
            .enabled_if(has_clipboard),
        MenuItem::new("Select All", MenuAction::SelectAll).with_shortcut("Ctrl+A"),
        MenuItem::new("Zoom In", MenuAction::ZoomIn).with_shortcut("Ctrl++"),
        MenuItem::new("Zoom Out", MenuAction::ZoomOut).with_shortcut("Ctrl+-"),
        MenuItem::new("Zoom to Fit", MenuAction::ZoomFit),
    ]
}

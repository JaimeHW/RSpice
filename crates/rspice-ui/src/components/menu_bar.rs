//! Menu Bar Component
//!
//! Professional-grade menu bar implementation following commercial EDA patterns.
//! Provides dropdown menus with keyboard navigation, shortcuts display, and
//! submenu support.
//!
//! # Architecture
//!
//! The menu system uses a data-driven approach:
//! - `MenuAction` - Enum of all possible menu actions
//! - `MenuItem` - Individual menu item with label, shortcut, action
//! - `Menu` - Top-level menu containing items
//! - `MenuBar` - The visual component rendering the menu bar
//!
//! This design allows menus to be defined declaratively and enables
//! easy testing, serialization, and customization.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::theme::Theme;

//=============================================================================
// Menu Actions
//=============================================================================

/// All possible menu actions in the application.
///
/// Each variant represents a command that can be triggered from a menu item.
/// This enum is the single source of truth for menu actions, enabling
/// consistent handling across menus, toolbars, and keyboard shortcuts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuAction {
    //-------------------------------------------------------------------------
    // File Menu
    //-------------------------------------------------------------------------
    /// Create a new schematic
    FileNew,
    /// Open an existing schematic file
    FileOpen,
    /// Save the current schematic
    FileSave,
    /// Save the current schematic with a new name
    FileSaveAs,
    /// Export schematic as SVG
    FileExportSvg,
    /// Export schematic as PDF
    FileExportPdf,
    /// Export waveforms as CSV
    FileExportCsv,
    /// Open preferences dialog
    FilePreferences,
    /// Exit the application
    FileExit,

    //-------------------------------------------------------------------------
    // Edit Menu
    //-------------------------------------------------------------------------
    /// Undo the last action
    EditUndo,
    /// Redo the last undone action
    EditRedo,
    /// Cut selected items to clipboard
    EditCut,
    /// Copy selected items to clipboard
    EditCopy,
    /// Paste items from clipboard
    EditPaste,
    /// Delete selected items
    EditDelete,
    /// Select all items
    EditSelectAll,
    /// Duplicate selected items
    EditDuplicate,
    /// Find/search in schematic
    EditFind,

    //-------------------------------------------------------------------------
    // View Menu
    //-------------------------------------------------------------------------
    /// Zoom in
    ViewZoomIn,
    /// Zoom out
    ViewZoomOut,
    /// Fit schematic to view
    ViewZoomFit,
    /// Reset zoom to 100%
    ViewZoom100,
    /// Toggle grid visibility
    ViewGrid,
    /// Toggle console panel
    ViewConsole,
    /// Toggle waveform viewer
    ViewWaveforms,
    /// Toggle library browser
    ViewLibrary,

    //-------------------------------------------------------------------------
    // Simulate Menu
    //-------------------------------------------------------------------------
    /// Run simulation
    SimulateRun,
    /// Stop running simulation
    SimulateStop,
    /// Open simulation setup dialog
    SimulateSetup,
    /// Open simulation options (advanced)
    SimulateOptions,

    //-------------------------------------------------------------------------
    // Tools Menu
    //-------------------------------------------------------------------------
    /// Import Verilog-A model
    ToolsVerilogA,
    /// Open model browser
    ToolsModelBrowser,
    /// Run DRC check
    ToolsDrc,

    //-------------------------------------------------------------------------
    // Help Menu
    //-------------------------------------------------------------------------
    /// Show keyboard shortcuts
    HelpShortcuts,
    /// Show about dialog
    HelpAbout,
}

impl MenuAction {
    /// Get the display label for this action
    pub fn label(&self) -> &'static str {
        match self {
            // File
            Self::FileNew => "New",
            Self::FileOpen => "Open...",
            Self::FileSave => "Save",
            Self::FileSaveAs => "Save As...",
            Self::FileExportSvg => "SVG...",
            Self::FileExportPdf => "PDF...",
            Self::FileExportCsv => "CSV (Waveforms)...",
            Self::FilePreferences => "Preferences...",
            Self::FileExit => "Exit",
            // Edit
            Self::EditUndo => "Undo",
            Self::EditRedo => "Redo",
            Self::EditCut => "Cut",
            Self::EditCopy => "Copy",
            Self::EditPaste => "Paste",
            Self::EditDelete => "Delete",
            Self::EditSelectAll => "Select All",
            Self::EditDuplicate => "Duplicate",
            Self::EditFind => "Find...",
            // View
            Self::ViewZoomIn => "Zoom In",
            Self::ViewZoomOut => "Zoom Out",
            Self::ViewZoomFit => "Fit to Window",
            Self::ViewZoom100 => "Actual Size (100%)",
            Self::ViewGrid => "Grid",
            Self::ViewConsole => "Console",
            Self::ViewWaveforms => "Waveforms",
            Self::ViewLibrary => "Library Browser",
            // Simulate
            Self::SimulateRun => "Run",
            Self::SimulateStop => "Stop",
            Self::SimulateSetup => "Setup...",
            Self::SimulateOptions => "Options...",
            // Tools
            Self::ToolsVerilogA => "Import Verilog-A...",
            Self::ToolsModelBrowser => "Model Browser",
            Self::ToolsDrc => "Design Rule Check",
            // Help
            Self::HelpShortcuts => "Keyboard Shortcuts",
            Self::HelpAbout => "About RSpice",
        }
    }

    /// Get the keyboard shortcut display string for this action
    pub fn shortcut_display(&self) -> Option<&'static str> {
        match self {
            // File
            Self::FileNew => Some("Ctrl+N"),
            Self::FileOpen => Some("Ctrl+O"),
            Self::FileSave => Some("Ctrl+S"),
            Self::FileSaveAs => Some("Ctrl+Shift+S"),
            Self::FilePreferences => Some("Ctrl+,"),
            // Edit
            Self::EditUndo => Some("Ctrl+Z"),
            Self::EditRedo => Some("Ctrl+Y"),
            Self::EditCut => Some("Ctrl+X"),
            Self::EditCopy => Some("Ctrl+C"),
            Self::EditPaste => Some("Ctrl+V"),
            Self::EditDelete => Some("Del"),
            Self::EditSelectAll => Some("Ctrl+A"),
            Self::EditDuplicate => Some("Ctrl+D"),
            Self::EditFind => Some("Ctrl+F"),
            // View
            Self::ViewZoomIn => Some("Ctrl++"),
            Self::ViewZoomOut => Some("Ctrl+-"),
            Self::ViewZoomFit => Some("Ctrl+0"),
            Self::ViewZoom100 => Some("Ctrl+1"),
            Self::ViewGrid => Some("G"),
            Self::ViewConsole => Some("Ctrl+`"),
            Self::ViewLibrary => Some("Ctrl+L"),
            // Simulate
            Self::SimulateRun => Some("F5"),
            Self::SimulateSetup => Some("F2"),
            // Help
            Self::HelpShortcuts => Some("Ctrl+/"),
            Self::HelpAbout => Some("F1"),
            // No shortcut
            _ => None,
        }
    }

    /// Get the menu category for this action
    pub fn category(&self) -> MenuCategory {
        match self {
            Self::FileNew
            | Self::FileOpen
            | Self::FileSave
            | Self::FileSaveAs
            | Self::FileExportSvg
            | Self::FileExportPdf
            | Self::FileExportCsv
            | Self::FilePreferences
            | Self::FileExit => MenuCategory::File,

            Self::EditUndo
            | Self::EditRedo
            | Self::EditCut
            | Self::EditCopy
            | Self::EditPaste
            | Self::EditDelete
            | Self::EditSelectAll
            | Self::EditDuplicate
            | Self::EditFind => MenuCategory::Edit,

            Self::ViewZoomIn
            | Self::ViewZoomOut
            | Self::ViewZoomFit
            | Self::ViewZoom100
            | Self::ViewGrid
            | Self::ViewConsole
            | Self::ViewWaveforms
            | Self::ViewLibrary => MenuCategory::View,

            Self::SimulateRun
            | Self::SimulateStop
            | Self::SimulateSetup
            | Self::SimulateOptions => MenuCategory::Simulate,

            Self::ToolsVerilogA | Self::ToolsModelBrowser | Self::ToolsDrc => MenuCategory::Tools,

            Self::HelpShortcuts | Self::HelpAbout => MenuCategory::Help,
        }
    }

    /// Get all actions in a category
    pub fn actions_in_category(category: MenuCategory) -> Vec<MenuAction> {
        use MenuAction::*;
        match category {
            MenuCategory::File => vec![
                FileNew,
                FileOpen,
                FileSave,
                FileSaveAs,
                FileExportSvg,
                FileExportPdf,
                FileExportCsv,
                FilePreferences,
                FileExit,
            ],
            MenuCategory::Edit => vec![
                EditUndo,
                EditRedo,
                EditCut,
                EditCopy,
                EditPaste,
                EditDelete,
                EditSelectAll,
                EditDuplicate,
                EditFind,
            ],
            MenuCategory::View => vec![
                ViewZoomIn,
                ViewZoomOut,
                ViewZoomFit,
                ViewZoom100,
                ViewGrid,
                ViewConsole,
                ViewWaveforms,
                ViewLibrary,
            ],
            MenuCategory::Simulate => {
                vec![SimulateRun, SimulateStop, SimulateSetup, SimulateOptions]
            }
            MenuCategory::Tools => vec![ToolsVerilogA, ToolsModelBrowser, ToolsDrc],
            MenuCategory::Help => vec![HelpShortcuts, HelpAbout],
        }
    }
}

/// Menu categories for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MenuCategory {
    File,
    Edit,
    View,
    Simulate,
    Tools,
    Help,
}

impl MenuCategory {
    /// Get display label
    pub fn label(&self) -> &'static str {
        match self {
            Self::File => "File",
            Self::Edit => "Edit",
            Self::View => "View",
            Self::Simulate => "Simulate",
            Self::Tools => "Tools",
            Self::Help => "Help",
        }
    }

    /// Get all categories in display order
    pub fn all() -> &'static [MenuCategory] {
        &[
            MenuCategory::File,
            MenuCategory::Edit,
            MenuCategory::View,
            MenuCategory::Simulate,
            MenuCategory::Tools,
            MenuCategory::Help,
        ]
    }
}

//=============================================================================
// Menu Item
//=============================================================================

/// A menu item that can be a command, submenu, separator, or toggle.
#[derive(Debug, Clone, PartialEq)]
pub enum MenuItem {
    /// A command item with action
    Action { action: MenuAction, enabled: bool },
    /// A submenu containing more items
    SubMenu {
        label: String,
        items: Vec<MenuItem>,
        enabled: bool,
    },
    /// A toggle item with checked state
    Toggle {
        action: MenuAction,
        checked: bool,
        enabled: bool,
    },
    /// A visual separator line
    Separator,
    /// A recent file entry
    RecentFile { path: String, display_name: String },
}

impl MenuItem {
    /// Create an action item
    pub fn action(action: MenuAction) -> Self {
        Self::Action {
            action,
            enabled: true,
        }
    }

    /// Create an action item with enabled state
    pub fn action_enabled(action: MenuAction, enabled: bool) -> Self {
        Self::Action { action, enabled }
    }

    /// Create a toggle item
    pub fn toggle(action: MenuAction, checked: bool) -> Self {
        Self::Toggle {
            action,
            checked,
            enabled: true,
        }
    }

    /// Create a submenu
    pub fn submenu(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self::SubMenu {
            label: label.into(),
            items,
            enabled: true,
        }
    }

    /// Create a separator
    pub fn separator() -> Self {
        Self::Separator
    }

    /// Create a recent file entry
    pub fn recent_file(path: impl Into<String>, display_name: impl Into<String>) -> Self {
        Self::RecentFile {
            path: path.into(),
            display_name: display_name.into(),
        }
    }

    /// Check if this item is enabled
    pub fn is_enabled(&self) -> bool {
        match self {
            Self::Action { enabled, .. } => *enabled,
            Self::SubMenu { enabled, .. } => *enabled,
            Self::Toggle { enabled, .. } => *enabled,
            Self::Separator => true,
            Self::RecentFile { .. } => true,
        }
    }

    /// Get the action if this is an action item
    pub fn get_action(&self) -> Option<MenuAction> {
        match self {
            Self::Action { action, .. } | Self::Toggle { action, .. } => Some(*action),
            _ => None,
        }
    }
}

//=============================================================================
// Menu Definition
//=============================================================================

/// A top-level menu with items
#[derive(Debug, Clone)]
pub struct Menu {
    pub label: String,
    pub items: Vec<MenuItem>,
    pub enabled: bool,
}

impl Menu {
    /// Create a new menu
    pub fn new(label: impl Into<String>, items: Vec<MenuItem>) -> Self {
        Self {
            label: label.into(),
            items,
            enabled: true,
        }
    }

    /// Create the default File menu
    pub fn file_menu(recent_files: &[String], is_dirty: bool) -> Self {
        let mut items = vec![
            MenuItem::action(MenuAction::FileNew),
            MenuItem::action(MenuAction::FileOpen),
        ];

        // Add Recent Files submenu
        if !recent_files.is_empty() {
            let recent_items: Vec<MenuItem> = recent_files
                .iter()
                .take(10)
                .map(|path| {
                    let display = std::path::Path::new(path)
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_else(|| path.clone());
                    MenuItem::recent_file(path.clone(), display)
                })
                .collect();
            items.push(MenuItem::submenu("Recent Files", recent_items));
        } else {
            items.push(MenuItem::SubMenu {
                label: "Recent Files".to_string(),
                items: vec![],
                enabled: false,
            });
        }

        items.push(MenuItem::separator());
        items.push(MenuItem::action_enabled(MenuAction::FileSave, is_dirty));
        items.push(MenuItem::action(MenuAction::FileSaveAs));
        items.push(MenuItem::separator());

        // Export submenu
        items.push(MenuItem::submenu(
            "Export",
            vec![
                MenuItem::action(MenuAction::FileExportSvg),
                MenuItem::action(MenuAction::FileExportPdf),
                MenuItem::action(MenuAction::FileExportCsv),
            ],
        ));

        items.push(MenuItem::separator());
        items.push(MenuItem::action(MenuAction::FilePreferences));
        items.push(MenuItem::separator());
        items.push(MenuItem::action(MenuAction::FileExit));

        Self::new("File", items)
    }

    /// Create the default Edit menu
    pub fn edit_menu(can_undo: bool, can_redo: bool, has_selection: bool) -> Self {
        Self::new(
            "Edit",
            vec![
                MenuItem::action_enabled(MenuAction::EditUndo, can_undo),
                MenuItem::action_enabled(MenuAction::EditRedo, can_redo),
                MenuItem::separator(),
                MenuItem::action_enabled(MenuAction::EditCut, has_selection),
                MenuItem::action_enabled(MenuAction::EditCopy, has_selection),
                MenuItem::action(MenuAction::EditPaste),
                MenuItem::action_enabled(MenuAction::EditDelete, has_selection),
                MenuItem::separator(),
                MenuItem::action(MenuAction::EditSelectAll),
                MenuItem::action_enabled(MenuAction::EditDuplicate, has_selection),
                MenuItem::separator(),
                MenuItem::action(MenuAction::EditFind),
            ],
        )
    }

    /// Create the default View menu
    pub fn view_menu(
        grid_visible: bool,
        console_visible: bool,
        waveforms_visible: bool,
        library_visible: bool,
    ) -> Self {
        Self::new(
            "View",
            vec![
                MenuItem::action(MenuAction::ViewZoomIn),
                MenuItem::action(MenuAction::ViewZoomOut),
                MenuItem::action(MenuAction::ViewZoomFit),
                MenuItem::action(MenuAction::ViewZoom100),
                MenuItem::separator(),
                MenuItem::toggle(MenuAction::ViewGrid, grid_visible),
                MenuItem::toggle(MenuAction::ViewConsole, console_visible),
                MenuItem::toggle(MenuAction::ViewWaveforms, waveforms_visible),
                MenuItem::toggle(MenuAction::ViewLibrary, library_visible),
            ],
        )
    }

    /// Create the default Simulate menu
    pub fn simulate_menu(is_running: bool) -> Self {
        Self::new(
            "Simulate",
            vec![
                MenuItem::action_enabled(MenuAction::SimulateRun, !is_running),
                MenuItem::action_enabled(MenuAction::SimulateStop, is_running),
                MenuItem::separator(),
                MenuItem::action(MenuAction::SimulateSetup),
                MenuItem::action(MenuAction::SimulateOptions),
            ],
        )
    }

    /// Create the default Tools menu
    pub fn tools_menu() -> Self {
        Self::new(
            "Tools",
            vec![
                MenuItem::action(MenuAction::ToolsVerilogA),
                MenuItem::separator(),
                MenuItem::action(MenuAction::ToolsModelBrowser),
                MenuItem::action(MenuAction::ToolsDrc),
            ],
        )
    }

    /// Create the default Help menu
    pub fn help_menu() -> Self {
        Self::new(
            "Help",
            vec![
                MenuItem::action(MenuAction::HelpShortcuts),
                MenuItem::separator(),
                MenuItem::action(MenuAction::HelpAbout),
            ],
        )
    }
}

//=============================================================================
// Menu Bar State
//=============================================================================

/// State for tracking which menu is open
#[derive(Debug, Clone, Default)]
pub struct MenuBarState {
    /// Currently open menu index (None if all closed)
    pub open_menu: Option<usize>,
    /// Currently open submenu path (for nested submenus)
    pub submenu_path: Vec<usize>,
    /// Highlighted item index within current menu
    pub highlighted_item: Option<usize>,
}

impl MenuBarState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_open(&self) -> bool {
        self.open_menu.is_some()
    }

    pub fn open(&mut self, menu_idx: usize) {
        self.open_menu = Some(menu_idx);
        self.submenu_path.clear();
        self.highlighted_item = None;
    }

    pub fn close(&mut self) {
        self.open_menu = None;
        self.submenu_path.clear();
        self.highlighted_item = None;
    }

    pub fn toggle(&mut self, menu_idx: usize) {
        if self.open_menu == Some(menu_idx) {
            self.close();
        } else {
            self.open(menu_idx);
        }
    }
}

//=============================================================================
// Menu Bar Component
//=============================================================================

/// Props for the MenuBar component
#[derive(Props, Clone, PartialEq)]
pub struct MenuBarProps {
    /// Callback when a menu action is triggered
    pub on_action: EventHandler<MenuAction>,
    /// Callback when a recent file is selected
    #[props(default)]
    pub on_recent_file: Option<EventHandler<String>>,
    /// Recent files list
    #[props(default)]
    pub recent_files: Vec<String>,
    /// Whether schematic is dirty (has unsaved changes)
    #[props(default)]
    pub is_dirty: bool,
    /// Whether simulation is currently running
    #[props(default)]
    pub is_running: bool,
    /// Whether there's a selection
    #[props(default)]
    pub has_selection: bool,
    /// Whether grid is visible
    #[props(default = true)]
    pub grid_visible: bool,
    /// Whether console is visible
    #[props(default = true)]
    pub console_visible: bool,
    /// Whether waveforms are visible
    #[props(default)]
    pub waveforms_visible: bool,
    /// Whether library is visible
    #[props(default)]
    pub library_visible: bool,
}

/// Main menu bar component
#[component]
pub fn MenuBar(props: MenuBarProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    // Menu state
    let mut state = use_signal(MenuBarState::new);

    // Build menus based on current state
    let menus = vec![
        Menu::file_menu(&props.recent_files, props.is_dirty),
        Menu::edit_menu(false, false, props.has_selection), // TODO: Wire up undo/redo state
        Menu::view_menu(
            props.grid_visible,
            props.console_visible,
            props.waveforms_visible,
            props.library_visible,
        ),
        Menu::simulate_menu(props.is_running),
        Menu::tools_menu(),
        Menu::help_menu(),
    ];

    // Handler for menu item clicks
    let on_action = props.on_action.clone();
    let on_recent = props.on_recent_file.clone();

    rsx! {
        div {
            class: "menu-bar",
            style: "
                display: flex;
                align-items: center;
                height: 32px;
                background: {th.bg_secondary()};
                border-bottom: 1px solid {th.border()};
                padding: 0 {Theme::SPACING_SM};
                user-select: none;
            ",

            // Logo
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: {Theme::SPACING_SM};
                    padding: 0 {Theme::SPACING_MD};
                    margin-right: {Theme::SPACING_SM};
                ",

                // Inline SVG logo (compact)
                svg {
                    width: "20",
                    height: "20",
                    view_box: "0 0 512 512",
                    fill: "none",

                    path {
                        d: "M256 32 L464 140 V372 L256 480 L48 372 V140 Z",
                        fill: "#2a2a3a"
                    }
                    path {
                        d: "M256 32 L464 140 V372 L256 480 L48 372 V140 Z",
                        stroke: "#FF5F15",
                        stroke_opacity: "0.3",
                        stroke_width: "12"
                    }
                    path {
                        d: "M100 256 L160 256 L200 150 L256 360 L312 150 L352 256 L412 256",
                        stroke: "#FF5F15",
                        stroke_width: "28",
                        stroke_linecap: "round",
                        stroke_linejoin: "round"
                    }
                }

                span {
                    style: "
                        font-size: 14px;
                        font-weight: 600;
                        color: {th.text_primary()};
                    ",
                    "RSpice"
                }
            }

            // Menu items
            for (idx, menu) in menus.iter().enumerate() {
                {
                    let menu_label = menu.label.clone();
                    let menu_items = menu.items.clone();
                    let is_open = state.read().open_menu == Some(idx);

                    rsx! {
                        MenuDropdown {
                            key: "{idx}",
                            label: menu_label,
                            items: menu_items,
                            is_open: is_open,
                            on_toggle: move |_| {
                                state.write().toggle(idx);
                            },
                            on_hover: move |_| {
                                // If any menu is open and we hover another, switch to it
                                if state.read().is_open() && state.read().open_menu != Some(idx) {
                                    state.write().open(idx);
                                }
                            },
                            on_action: move |action: MenuAction| {
                                state.write().close();
                                on_action.call(action);
                            },
                            on_recent_file: move |path: String| {
                                state.write().close();
                                if let Some(ref handler) = on_recent {
                                    handler.call(path);
                                }
                            },
                        }
                    }
                }
            }

            // Click outside to close
            if state.read().is_open() {
                div {
                    style: "
                        position: fixed;
                        top: 0;
                        left: 0;
                        right: 0;
                        bottom: 0;
                        z-index: 999;
                    ",
                    onclick: move |_| {
                        state.write().close();
                    },
                }
            }
        }
    }
}

//=============================================================================
// Menu Dropdown Component
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct MenuDropdownProps {
    label: String,
    items: Vec<MenuItem>,
    is_open: bool,
    on_toggle: EventHandler<()>,
    on_hover: EventHandler<()>,
    on_action: EventHandler<MenuAction>,
    on_recent_file: EventHandler<String>,
}

#[component]
fn MenuDropdown(props: MenuDropdownProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let bg_color = if props.is_open || *hovered.read() {
        th.surface_hover().to_string()
    } else {
        "transparent".to_string()
    };

    rsx! {
        div {
            style: "position: relative;",

            // Menu button
            button {
                style: "
                    display: flex;
                    align-items: center;
                    height: 28px;
                    padding: 0 10px;
                    background: {bg_color};
                    border: none;
                    border-radius: 4px;
                    color: {th.text_primary()};
                    font-size: 13px;
                    font-weight: 500;
                    cursor: pointer;
                    outline: none;
                ",
                onclick: move |_| props.on_toggle.call(()),
                onmouseenter: move |_| {
                    hovered.set(true);
                    props.on_hover.call(());
                },
                onmouseleave: move |_| {
                    hovered.set(false);
                },
                "{props.label}"
            }

            // Dropdown panel
            if props.is_open {
                div {
                    style: "
                        position: absolute;
                        top: 100%;
                        left: 0;
                        min-width: 200px;
                        max-width: 300px;
                        background: {th.surface()};
                        border: 1px solid {th.border()};
                        border-radius: 6px;
                        box-shadow: 0 4px 16px rgba(0,0,0,0.25);
                        z-index: 1000;
                        padding: 4px 0;
                    ",

                    for item in props.items.iter() {
                        {render_menu_item(item.clone(), props.on_action.clone(), props.on_recent_file.clone())}
                    }
                }
            }
        }
    }
}

//=============================================================================
// Menu Item Rendering
//=============================================================================

fn render_menu_item(
    item: MenuItem,
    on_action: EventHandler<MenuAction>,
    on_recent_file: EventHandler<String>,
) -> Element {
    match item {
        MenuItem::Action { action, enabled } => {
            rsx! {
                MenuItemRow {
                    label: action.label().to_string(),
                    shortcut: action.shortcut_display().map(|s| s.to_string()),
                    enabled: enabled,
                    checked: false,
                    is_submenu: false,
                    onclick: move |_| {
                        if enabled {
                            on_action.call(action);
                        }
                    },
                }
            }
        }
        MenuItem::Toggle {
            action,
            checked,
            enabled,
        } => {
            rsx! {
                MenuItemRow {
                    label: action.label().to_string(),
                    shortcut: action.shortcut_display().map(|s| s.to_string()),
                    enabled: enabled,
                    checked: checked,
                    is_submenu: false,
                    onclick: move |_| {
                        if enabled {
                            on_action.call(action);
                        }
                    },
                }
            }
        }
        MenuItem::Separator => {
            rsx! {
                MenuSeparator {}
            }
        }
        MenuItem::SubMenu {
            label,
            items,
            enabled,
        } => {
            rsx! {
                SubMenuRow {
                    label: label,
                    items: items,
                    enabled: enabled,
                    on_action: on_action,
                    on_recent_file: on_recent_file,
                }
            }
        }
        MenuItem::RecentFile { path, display_name } => {
            let path_clone = path.clone();
            rsx! {
                MenuItemRow {
                    label: display_name,
                    shortcut: None,
                    enabled: true,
                    checked: false,
                    is_submenu: false,
                    onclick: move |_| {
                        on_recent_file.call(path_clone.clone());
                    },
                }
            }
        }
    }
}

//=============================================================================
// Menu Item Row Component
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct MenuItemRowProps {
    label: String,
    shortcut: Option<String>,
    enabled: bool,
    checked: bool,
    is_submenu: bool,
    onclick: EventHandler<()>,
}

#[component]
fn MenuItemRow(props: MenuItemRowProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);

    let bg_color = if *hovered.read() && props.enabled {
        th.accent_primary().to_string()
    } else {
        "transparent".to_string()
    };

    let text_color = if props.enabled {
        if *hovered.read() {
            "#ffffff".to_string()
        } else {
            th.text_primary().to_string()
        }
    } else {
        th.text_muted().to_string()
    };

    let cursor = if props.enabled { "pointer" } else { "default" };

    rsx! {
        div {
            style: "
                display: flex;
                align-items: center;
                justify-content: space-between;
                padding: 6px 12px;
                background: {bg_color};
                color: {text_color};
                cursor: {cursor};
                font-size: 13px;
            ",
            onmouseenter: move |_| hovered.set(true),
            onmouseleave: move |_| hovered.set(false),
            onclick: move |_| {
                if props.enabled {
                    props.onclick.call(());
                }
            },

            // Left side: checkmark + label
            div {
                style: "display: flex; align-items: center; gap: 8px;",

                // Checkmark for toggle items
                span {
                    style: "width: 16px; text-align: center;",
                    if props.checked { "✓" } else { "" }
                }

                span { "{props.label}" }
            }

            // Right side: shortcut or submenu arrow
            if props.is_submenu {
                span {
                    style: "margin-left: 12px; opacity: 0.7;",
                    "▸"
                }
            } else if let Some(shortcut) = &props.shortcut {
                span {
                    style: "
                        margin-left: 24px;
                        font-size: 11px;
                        opacity: 0.6;
                    ",
                    "{shortcut}"
                }
            }
        }
    }
}

//=============================================================================
// Submenu Row Component
//=============================================================================

#[derive(Props, Clone, PartialEq)]
struct SubMenuRowProps {
    label: String,
    items: Vec<MenuItem>,
    enabled: bool,
    on_action: EventHandler<MenuAction>,
    on_recent_file: EventHandler<String>,
}

#[component]
fn SubMenuRow(props: SubMenuRowProps) -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();
    let mut hovered = use_signal(|| false);
    let mut submenu_open = use_signal(|| false);

    let bg_color = if *hovered.read() && props.enabled {
        th.accent_primary().to_string()
    } else {
        "transparent".to_string()
    };

    let text_color = if props.enabled {
        if *hovered.read() {
            "#ffffff".to_string()
        } else {
            th.text_primary().to_string()
        }
    } else {
        th.text_muted().to_string()
    };

    let cursor = if props.enabled { "pointer" } else { "default" };

    rsx! {
        div {
            style: "position: relative;",
            onmouseenter: move |_| {
                hovered.set(true);
                if props.enabled {
                    submenu_open.set(true);
                }
            },
            onmouseleave: move |_| {
                hovered.set(false);
                submenu_open.set(false);
            },

            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 6px 12px;
                    background: {bg_color};
                    color: {text_color};
                    cursor: {cursor};
                    font-size: 13px;
                ",

                div {
                    style: "display: flex; align-items: center; gap: 8px;",
                    span { style: "width: 16px;", "" }
                    span { "{props.label}" }
                }

                span {
                    style: "margin-left: 12px; opacity: 0.7;",
                    "▸"
                }
            }

            // Submenu panel
            if *submenu_open.read() && props.enabled && !props.items.is_empty() {
                div {
                    style: "
                        position: absolute;
                        top: 0;
                        left: 100%;
                        min-width: 180px;
                        background: {th.surface()};
                        border: 1px solid {th.border()};
                        border-radius: 6px;
                        box-shadow: 0 4px 16px rgba(0,0,0,0.25);
                        z-index: 1001;
                        padding: 4px 0;
                    ",

                    for item in props.items.iter() {
                        {render_menu_item(item.clone(), props.on_action.clone(), props.on_recent_file.clone())}
                    }
                }
            }
        }
    }
}

//=============================================================================
// Menu Separator Component
//=============================================================================

#[component]
fn MenuSeparator() -> Element {
    let theme: Signal<Theme> = use_context();
    let th = theme.read();

    rsx! {
        div {
            style: "
                height: 1px;
                background: {th.border()};
                margin: 4px 8px;
            "
        }
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    //-------------------------------------------------------------------------
    // MenuAction Tests
    //-------------------------------------------------------------------------

    #[test]
    fn test_all_actions_have_labels() {
        for category in MenuCategory::all() {
            for action in MenuAction::actions_in_category(*category) {
                let label = action.label();
                assert!(!label.is_empty(), "Action {:?} has empty label", action);
            }
        }
    }

    #[test]
    fn test_action_categories_cover_all_actions() {
        use MenuAction::*;
        let all_actions = vec![
            // File
            FileNew,
            FileOpen,
            FileSave,
            FileSaveAs,
            FileExportSvg,
            FileExportPdf,
            FileExportCsv,
            FilePreferences,
            FileExit,
            // Edit
            EditUndo,
            EditRedo,
            EditCut,
            EditCopy,
            EditPaste,
            EditDelete,
            EditSelectAll,
            EditDuplicate,
            EditFind,
            // View
            ViewZoomIn,
            ViewZoomOut,
            ViewZoomFit,
            ViewZoom100,
            ViewGrid,
            ViewConsole,
            ViewWaveforms,
            ViewLibrary,
            // Simulate
            SimulateRun,
            SimulateStop,
            SimulateSetup,
            SimulateOptions,
            // Tools
            ToolsVerilogA,
            ToolsModelBrowser,
            ToolsDrc,
            // Help
            HelpShortcuts,
            HelpAbout,
        ];

        for action in all_actions {
            let category = action.category();
            let actions_in_cat = MenuAction::actions_in_category(category);
            assert!(
                actions_in_cat.contains(&action),
                "Action {:?} not found in category {:?}",
                action,
                category
            );
        }
    }

    #[test]
    fn test_shortcut_display_formats() {
        assert_eq!(MenuAction::FileNew.shortcut_display(), Some("Ctrl+N"));
        assert_eq!(
            MenuAction::FileSaveAs.shortcut_display(),
            Some("Ctrl+Shift+S")
        );
        assert_eq!(MenuAction::EditDelete.shortcut_display(), Some("Del"));
        assert_eq!(MenuAction::SimulateRun.shortcut_display(), Some("F5"));
        assert_eq!(MenuAction::ViewGrid.shortcut_display(), Some("G"));
    }

    #[test]
    fn test_some_actions_have_no_shortcut() {
        // These actions don't have shortcuts
        assert_eq!(MenuAction::FileExit.shortcut_display(), None);
        assert_eq!(MenuAction::SimulateStop.shortcut_display(), None);
        assert_eq!(MenuAction::ToolsModelBrowser.shortcut_display(), None);
    }

    //-------------------------------------------------------------------------
    // MenuItem Tests
    //-------------------------------------------------------------------------

    #[test]
    fn test_menu_item_action_creation() {
        let item = MenuItem::action(MenuAction::FileNew);
        assert!(item.is_enabled());
        assert_eq!(item.get_action(), Some(MenuAction::FileNew));
    }

    #[test]
    fn test_menu_item_disabled() {
        let item = MenuItem::action_enabled(MenuAction::FileSave, false);
        assert!(!item.is_enabled());
    }

    #[test]
    fn test_menu_item_toggle() {
        let item = MenuItem::toggle(MenuAction::ViewGrid, true);
        assert!(item.is_enabled());
        if let MenuItem::Toggle { checked, .. } = item {
            assert!(checked);
        } else {
            panic!("Expected Toggle variant");
        }
    }

    #[test]
    fn test_menu_item_submenu() {
        let submenu = MenuItem::submenu(
            "Export",
            vec![
                MenuItem::action(MenuAction::FileExportSvg),
                MenuItem::action(MenuAction::FileExportPdf),
            ],
        );

        if let MenuItem::SubMenu { label, items, .. } = submenu {
            assert_eq!(label, "Export");
            assert_eq!(items.len(), 2);
        } else {
            panic!("Expected SubMenu variant");
        }
    }

    #[test]
    fn test_menu_item_separator() {
        let sep = MenuItem::separator();
        assert!(sep.is_enabled());
        assert_eq!(sep.get_action(), None);
    }

    #[test]
    fn test_menu_item_recent_file() {
        let item = MenuItem::recent_file("/path/to/file.sp", "file.sp");
        if let MenuItem::RecentFile { path, display_name } = item {
            assert_eq!(path, "/path/to/file.sp");
            assert_eq!(display_name, "file.sp");
        } else {
            panic!("Expected RecentFile variant");
        }
    }

    //-------------------------------------------------------------------------
    // Menu Tests
    //-------------------------------------------------------------------------

    #[test]
    fn test_file_menu_structure() {
        let menu = Menu::file_menu(&[], false);
        assert_eq!(menu.label, "File");
        assert!(menu.enabled);
        // Check for required items
        assert!(menu.items.iter().any(|i| matches!(
            i,
            MenuItem::Action {
                action: MenuAction::FileNew,
                ..
            }
        )));
        assert!(menu.items.iter().any(|i| matches!(
            i,
            MenuItem::Action {
                action: MenuAction::FileOpen,
                ..
            }
        )));
        assert!(menu.items.iter().any(|i| matches!(
            i,
            MenuItem::Action {
                action: MenuAction::FileExit,
                ..
            }
        )));
    }

    #[test]
    fn test_file_menu_with_recent_files() {
        let recent = vec![
            "/path/to/file1.sp".to_string(),
            "/path/to/file2.sp".to_string(),
        ];
        let menu = Menu::file_menu(&recent, false);

        // Find the Recent Files submenu
        let recent_submenu = menu.items.iter().find(|i| {
            if let MenuItem::SubMenu { label, .. } = i {
                label == "Recent Files"
            } else {
                false
            }
        });

        assert!(recent_submenu.is_some());
        if let Some(MenuItem::SubMenu { items, enabled, .. }) = recent_submenu {
            assert!(enabled);
            assert_eq!(items.len(), 2);
        }
    }

    #[test]
    fn test_file_menu_save_enabled_when_dirty() {
        let menu_dirty = Menu::file_menu(&[], true);
        let menu_clean = Menu::file_menu(&[], false);

        // Find Save item in both menus
        let find_save = |m: &Menu| {
            m.items.iter().find_map(|i| match i {
                MenuItem::Action {
                    action: MenuAction::FileSave,
                    enabled,
                } => Some(*enabled),
                _ => None,
            })
        };

        assert_eq!(find_save(&menu_dirty), Some(true));
        assert_eq!(find_save(&menu_clean), Some(false));
    }

    #[test]
    fn test_edit_menu_structure() {
        let menu = Menu::edit_menu(true, false, true);
        assert_eq!(menu.label, "Edit");

        // Check undo enabled, redo disabled
        let undo = menu.items.iter().find_map(|i| match i {
            MenuItem::Action {
                action: MenuAction::EditUndo,
                enabled,
            } => Some(*enabled),
            _ => None,
        });
        let redo = menu.items.iter().find_map(|i| match i {
            MenuItem::Action {
                action: MenuAction::EditRedo,
                enabled,
            } => Some(*enabled),
            _ => None,
        });

        assert_eq!(undo, Some(true));
        assert_eq!(redo, Some(false));
    }

    #[test]
    fn test_edit_menu_selection_dependent_items() {
        let menu_with_selection = Menu::edit_menu(false, false, true);
        let menu_no_selection = Menu::edit_menu(false, false, false);

        // Cut/Copy/Delete should be enabled only with selection
        let find_cut = |m: &Menu| {
            m.items.iter().find_map(|i| match i {
                MenuItem::Action {
                    action: MenuAction::EditCut,
                    enabled,
                } => Some(*enabled),
                _ => None,
            })
        };

        assert_eq!(find_cut(&menu_with_selection), Some(true));
        assert_eq!(find_cut(&menu_no_selection), Some(false));
    }

    #[test]
    fn test_view_menu_toggles() {
        let menu = Menu::view_menu(true, false, true, false);

        // Find toggle items and check their state
        for item in &menu.items {
            if let MenuItem::Toggle {
                action, checked, ..
            } = item
            {
                match action {
                    MenuAction::ViewGrid => assert!(checked, "Grid should be checked"),
                    MenuAction::ViewConsole => assert!(!checked, "Console should be unchecked"),
                    MenuAction::ViewWaveforms => assert!(checked, "Waveforms should be checked"),
                    MenuAction::ViewLibrary => assert!(!checked, "Library should be unchecked"),
                    _ => {}
                }
            }
        }
    }

    #[test]
    fn test_simulate_menu_running_state() {
        let menu_running = Menu::simulate_menu(true);
        let menu_stopped = Menu::simulate_menu(false);

        let find_run_enabled = |m: &Menu| {
            m.items.iter().find_map(|i| match i {
                MenuItem::Action {
                    action: MenuAction::SimulateRun,
                    enabled,
                } => Some(*enabled),
                _ => None,
            })
        };
        let find_stop_enabled = |m: &Menu| {
            m.items.iter().find_map(|i| match i {
                MenuItem::Action {
                    action: MenuAction::SimulateStop,
                    enabled,
                } => Some(*enabled),
                _ => None,
            })
        };

        // Running: Run disabled, Stop enabled
        assert_eq!(find_run_enabled(&menu_running), Some(false));
        assert_eq!(find_stop_enabled(&menu_running), Some(true));

        // Stopped: Run enabled, Stop disabled
        assert_eq!(find_run_enabled(&menu_stopped), Some(true));
        assert_eq!(find_stop_enabled(&menu_stopped), Some(false));
    }

    #[test]
    fn test_tools_menu_structure() {
        let menu = Menu::tools_menu();
        assert_eq!(menu.label, "Tools");
        assert!(menu.items.iter().any(|i| matches!(
            i,
            MenuItem::Action {
                action: MenuAction::ToolsVerilogA,
                ..
            }
        )));
    }

    #[test]
    fn test_help_menu_structure() {
        let menu = Menu::help_menu();
        assert_eq!(menu.label, "Help");
        assert!(menu.items.iter().any(|i| matches!(
            i,
            MenuItem::Action {
                action: MenuAction::HelpAbout,
                ..
            }
        )));
        assert!(menu.items.iter().any(|i| matches!(
            i,
            MenuItem::Action {
                action: MenuAction::HelpShortcuts,
                ..
            }
        )));
    }

    //-------------------------------------------------------------------------
    // MenuBarState Tests
    //-------------------------------------------------------------------------

    #[test]
    fn test_menu_bar_state_default() {
        let state = MenuBarState::new();
        assert!(!state.is_open());
        assert_eq!(state.open_menu, None);
    }

    #[test]
    fn test_menu_bar_state_open() {
        let mut state = MenuBarState::new();
        state.open(2);
        assert!(state.is_open());
        assert_eq!(state.open_menu, Some(2));
    }

    #[test]
    fn test_menu_bar_state_close() {
        let mut state = MenuBarState::new();
        state.open(1);
        state.close();
        assert!(!state.is_open());
        assert_eq!(state.open_menu, None);
    }

    #[test]
    fn test_menu_bar_state_toggle() {
        let mut state = MenuBarState::new();

        state.toggle(0);
        assert_eq!(state.open_menu, Some(0));

        state.toggle(0);
        assert_eq!(state.open_menu, None);

        state.toggle(1);
        assert_eq!(state.open_menu, Some(1));
    }

    //-------------------------------------------------------------------------
    // MenuCategory Tests
    //-------------------------------------------------------------------------

    #[test]
    fn test_menu_category_labels() {
        assert_eq!(MenuCategory::File.label(), "File");
        assert_eq!(MenuCategory::Edit.label(), "Edit");
        assert_eq!(MenuCategory::View.label(), "View");
        assert_eq!(MenuCategory::Simulate.label(), "Simulate");
        assert_eq!(MenuCategory::Tools.label(), "Tools");
        assert_eq!(MenuCategory::Help.label(), "Help");
    }

    #[test]
    fn test_menu_category_all() {
        let all = MenuCategory::all();
        assert_eq!(all.len(), 6);
        assert_eq!(all[0], MenuCategory::File);
        assert_eq!(all[5], MenuCategory::Help);
    }

    //-------------------------------------------------------------------------
    // Shortcut Consistency Tests
    //-------------------------------------------------------------------------

    #[test]
    fn test_common_shortcuts_follow_conventions() {
        // These are industry-standard shortcuts that must not change
        assert_eq!(MenuAction::FileNew.shortcut_display(), Some("Ctrl+N"));
        assert_eq!(MenuAction::FileOpen.shortcut_display(), Some("Ctrl+O"));
        assert_eq!(MenuAction::FileSave.shortcut_display(), Some("Ctrl+S"));
        assert_eq!(MenuAction::EditUndo.shortcut_display(), Some("Ctrl+Z"));
        assert_eq!(MenuAction::EditRedo.shortcut_display(), Some("Ctrl+Y"));
        assert_eq!(MenuAction::EditCut.shortcut_display(), Some("Ctrl+X"));
        assert_eq!(MenuAction::EditCopy.shortcut_display(), Some("Ctrl+C"));
        assert_eq!(MenuAction::EditPaste.shortcut_display(), Some("Ctrl+V"));
    }

    #[test]
    fn test_file_menu_has_expected_structure() {
        let menu = Menu::file_menu(&[], false);

        // Count items by type
        let action_count = menu
            .items
            .iter()
            .filter(|i| matches!(i, MenuItem::Action { .. }))
            .count();
        let separator_count = menu
            .items
            .iter()
            .filter(|i| matches!(i, MenuItem::Separator))
            .count();
        let submenu_count = menu
            .items
            .iter()
            .filter(|i| matches!(i, MenuItem::SubMenu { .. }))
            .count();

        // File menu should have substantial structure
        assert!(
            action_count >= 5,
            "File menu should have at least 5 action items"
        );
        assert!(
            separator_count >= 3,
            "File menu should have separators for grouping"
        );
        assert!(
            submenu_count >= 2,
            "File menu should have Recent Files and Export submenus"
        );
    }
}

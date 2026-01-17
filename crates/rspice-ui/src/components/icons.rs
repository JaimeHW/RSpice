//! Icon Component
//!
//! SVG-based icon system with a curated set of circuit simulator icons.

use dioxus::prelude::*;

/// Available icon types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IconType {
    // Simulation controls
    Play,
    Stop,
    Pause,
    
    // File operations
    File,
    FolderOpen,
    Save,
    
    // Edit operations
    Undo,
    Redo,
    
    // View controls
    ZoomIn,
    ZoomOut,
    FitToScreen,
    
    // Components
    Resistor,
    Capacitor,
    Inductor,
    Ground,
    
    // UI
    Settings,
    ChevronRight,
    ChevronDown,
    Close,
    Menu,
}

/// Icon component with consistent sizing
#[component]
pub fn Icon(
    /// The icon to display
    icon: IconType,
    /// Size in pixels (default: 16)
    #[props(default = 16)]
    size: u32,
    /// Color (defaults to currentColor)
    #[props(default = "currentColor".to_string())]
    color: String,
) -> Element {
    let size_str = format!("{}px", size);

    let path = match icon {
        IconType::Play => "M8 5v14l11-7z",
        IconType::Stop => "M6 6h12v12H6z",
        IconType::Pause => "M6 4h4v16H6zm8 0h4v16h-4z",
        
        IconType::File => "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z M14 2v6h6",
        IconType::FolderOpen => "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z",
        IconType::Save => "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z M17 21v-8H7v8 M7 3v5h8",
        
        IconType::Undo => "M3 7v6h6 M3 13A9 9 0 1 0 6 6",
        IconType::Redo => "M21 7v6h-6 M21 13a9 9 0 1 1-3-7",
        
        IconType::ZoomIn => "M11 3a8 8 0 1 0 0 16 8 8 0 0 0 0-16zm0 4v8m-4-4h8 M21 21l-4.35-4.35",
        IconType::ZoomOut => "M11 3a8 8 0 1 0 0 16 8 8 0 0 0 0-16zm-4 8h8 M21 21l-4.35-4.35",
        IconType::FitToScreen => "M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3",
        
        IconType::Resistor => "M2 12h4l1-3 2 6 2-6 2 6 2-6 2 6 1-3h4",
        IconType::Capacitor => "M9 4v16M15 4v16M4 12h5M15 12h5",
        IconType::Inductor => "M2 12h3c0-2 1.5-3 3-3s3 1 3 3 1.5 3 3 3 3-1 3-3h3",
        IconType::Ground => "M12 8v8 M8 16h8 M10 19h4 M11 22h2",
        
        IconType::Settings => "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 1 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 1 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 1 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 1 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z",
        IconType::ChevronRight => "M9 18l6-6-6-6",
        IconType::ChevronDown => "M6 9l6 6 6-6",
        IconType::Close => "M18 6L6 18M6 6l12 12",
        IconType::Menu => "M3 12h18M3 6h18M3 18h18",
    };

    rsx! {
        svg {
            width: "{size_str}",
            height: "{size_str}",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "{color}",
            stroke_width: "2",
            stroke_linecap: "round",
            stroke_linejoin: "round",

            path { d: "{path}" }
        }
    }
}

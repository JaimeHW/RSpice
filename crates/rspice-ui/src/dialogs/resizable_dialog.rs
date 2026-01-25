//! Resizable Dialog Component
//!
//! A commercial-grade, reusable dialog wrapper that provides consistent
//! resize behavior across all RSpice dialogs. Follows patterns from
//! professional EDA tools like Cadence Virtuoso.
//!
//! ## Features
//! - 8-direction resize handles (edges and corners)
//! - Configurable min/max size constraints
//! - Smooth drag-to-resize with visual feedback
//! - Backdrop overlay with click-to-dismiss
//! - Themeable styling via RSpice theme system
//!
//! ## Usage
//! ```rust,ignore
//! ResizableDialog {
//!     title: "My Dialog",
//!     is_open: is_open(),
//!     on_close: move |_| set_open(false),
//!     default_width: 800.0,
//!     default_height: 600.0,
//!     // Dialog content goes here as children
//!     div { "Content" }
//! }
//! ```

use dioxus::prelude::*;

use crate::theme::Theme;

// =============================================================================
// Resize Edge Enum
// =============================================================================

/// Represents the edge or corner being resized.
///
/// Used for both hit detection and cursor style selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ResizeEdge {
    /// No edge selected (default state)
    #[default]
    None,
    /// Top edge (vertical resize)
    Top,
    /// Bottom edge (vertical resize)
    Bottom,
    /// Left edge (horizontal resize)
    Left,
    /// Right edge (horizontal resize)
    Right,
    /// Top-left corner (diagonal resize)
    TopLeft,
    /// Top-right corner (diagonal resize)
    TopRight,
    /// Bottom-left corner (diagonal resize)
    BottomLeft,
    /// Bottom-right corner (diagonal resize)
    BottomRight,
}

impl ResizeEdge {
    /// Returns the CSS cursor style for this edge.
    ///
    /// Follows standard windowing conventions:
    /// - Vertical edges: n-resize, s-resize
    /// - Horizontal edges: w-resize, e-resize
    /// - Corners: nw-resize, ne-resize, sw-resize, se-resize
    pub fn cursor_style(&self) -> &'static str {
        match self {
            Self::None => "default",
            Self::Top => "n-resize",
            Self::Bottom => "s-resize",
            Self::Left => "w-resize",
            Self::Right => "e-resize",
            Self::TopLeft => "nw-resize",
            Self::TopRight => "ne-resize",
            Self::BottomLeft => "sw-resize",
            Self::BottomRight => "se-resize",
        }
    }

    /// Returns all non-None edge variants.
    pub fn all() -> &'static [ResizeEdge] {
        &[
            Self::Top,
            Self::Bottom,
            Self::Left,
            Self::Right,
            Self::TopLeft,
            Self::TopRight,
            Self::BottomLeft,
            Self::BottomRight,
        ]
    }

    /// Detect which edge/corner a point is near, given dialog dimensions.
    ///
    /// # Arguments
    /// - `x`, `y`: Mouse position relative to dialog top-left
    /// - `width`, `height`: Dialog dimensions
    /// - `edge_size`: Size of edge detection zone (typically 8px)
    /// - `corner_size`: Size of corner detection zone (typically 12px)
    ///
    /// # Returns
    /// The edge/corner the point is nearest to, or `None` if inside the dialog.
    pub fn from_position(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        edge_size: f64,
        corner_size: f64,
    ) -> Self {
        let near_left = x < edge_size;
        let near_right = x > width - edge_size;
        let near_top = y < edge_size;
        let near_bottom = y > height - edge_size;

        let in_corner_x_left = x < corner_size;
        let in_corner_x_right = x > width - corner_size;
        let in_corner_y_top = y < corner_size;
        let in_corner_y_bottom = y > height - corner_size;

        // Check corners first (they take priority)
        if in_corner_y_top && in_corner_x_left {
            Self::TopLeft
        } else if in_corner_y_top && in_corner_x_right {
            Self::TopRight
        } else if in_corner_y_bottom && in_corner_x_left {
            Self::BottomLeft
        } else if in_corner_y_bottom && in_corner_x_right {
            Self::BottomRight
        }
        // Then check edges
        else if near_top {
            Self::Top
        } else if near_bottom {
            Self::Bottom
        } else if near_left {
            Self::Left
        } else if near_right {
            Self::Right
        } else {
            Self::None
        }
    }

    /// Returns true if this edge affects horizontal size.
    pub fn affects_width(&self) -> bool {
        matches!(
            self,
            Self::Left
                | Self::Right
                | Self::TopLeft
                | Self::TopRight
                | Self::BottomLeft
                | Self::BottomRight
        )
    }

    /// Returns true if this edge affects vertical size.
    pub fn affects_height(&self) -> bool {
        matches!(
            self,
            Self::Top
                | Self::Bottom
                | Self::TopLeft
                | Self::TopRight
                | Self::BottomLeft
                | Self::BottomRight
        )
    }

    /// Returns true if resizing from this edge moves the left edge.
    pub fn moves_left(&self) -> bool {
        matches!(self, Self::Left | Self::TopLeft | Self::BottomLeft)
    }

    /// Returns true if resizing from this edge moves the top edge.
    pub fn moves_top(&self) -> bool {
        matches!(self, Self::Top | Self::TopLeft | Self::TopRight)
    }
}

// =============================================================================
// Dialog Size Constraints
// =============================================================================

/// Size constraints for resizable dialogs.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DialogSizeConstraints {
    /// Minimum width in pixels
    pub min_width: f64,
    /// Minimum height in pixels
    pub min_height: f64,
    /// Maximum width in pixels (use large values for "unlimited")
    pub max_width: f64,
    /// Maximum height in pixels (use large values for "unlimited")
    pub max_height: f64,
}

impl Default for DialogSizeConstraints {
    fn default() -> Self {
        Self {
            min_width: 400.0,
            min_height: 300.0,
            max_width: 2000.0,
            max_height: 1500.0,
        }
    }
}

impl DialogSizeConstraints {
    /// Clamp a width value to the constraints.
    pub fn clamp_width(&self, width: f64) -> f64 {
        width.clamp(self.min_width, self.max_width)
    }

    /// Clamp a height value to the constraints.
    pub fn clamp_height(&self, height: f64) -> f64 {
        height.clamp(self.min_height, self.max_height)
    }
}

// =============================================================================
// Resizable Dialog Props
// =============================================================================

/// Props for the ResizableDialog component.
#[derive(Props, Clone, PartialEq)]
pub struct ResizableDialogProps {
    /// Dialog title displayed in the header
    pub title: String,

    /// Whether the dialog is currently open/visible
    pub is_open: bool,

    /// Handler called when the dialog should close
    pub on_close: EventHandler<()>,

    /// Default width when dialog opens (pixels)
    #[props(default = 750.0)]
    pub default_width: f64,

    /// Default height when dialog opens (pixels)
    #[props(default = 550.0)]
    pub default_height: f64,

    /// Minimum width constraint (pixels)
    #[props(default = 400.0)]
    pub min_width: f64,

    /// Minimum height constraint (pixels)
    #[props(default = 300.0)]
    pub min_height: f64,

    /// Maximum width constraint (pixels)
    #[props(default = 2000.0)]
    pub max_width: f64,

    /// Maximum height constraint (pixels)
    #[props(default = 1500.0)]
    pub max_height: f64,

    /// Dialog body content
    pub children: Element,
}

// =============================================================================
// Component Constants
// =============================================================================

/// Edge detection zone size in pixels
const EDGE_SIZE: f64 = 8.0;

/// Corner detection zone size in pixels
const CORNER_SIZE: f64 = 14.0;

// =============================================================================
// Resizable Dialog Component
// =============================================================================

/// A resizable dialog wrapper component.
///
/// Provides consistent resize behavior across all RSpice dialogs with
/// professional-grade interactions matching commercial EDA tools.
#[component]
pub fn ResizableDialog(props: ResizableDialogProps) -> Element {
    // Early return if not open
    if !props.is_open {
        return rsx! {};
    }

    let theme = use_context::<Signal<Theme>>();

    // Size constraints
    let constraints = DialogSizeConstraints {
        min_width: props.min_width,
        min_height: props.min_height,
        max_width: props.max_width,
        max_height: props.max_height,
    };

    // Dialog size state (initialized from props defaults)
    let mut width = use_signal(|| constraints.clamp_width(props.default_width));
    let mut height = use_signal(|| constraints.clamp_height(props.default_height));

    // Resize state
    let mut resize_edge: Signal<ResizeEdge> = use_signal(|| ResizeEdge::None);
    let mut resize_start_mouse = use_signal(|| (0.0_f64, 0.0_f64));
    let mut resize_start_size = use_signal(|| (0.0_f64, 0.0_f64));
    let mut resize_start_pos = use_signal(|| (0.0_f64, 0.0_f64));

    // Dialog position state (for edge resizing that moves the dialog)
    let mut dialog_offset_x = use_signal(|| 0.0_f64);
    let mut dialog_offset_y = use_signal(|| 0.0_f64);

    // Drag state (for title bar drag-to-move)
    let mut is_dragging = use_signal(|| false);
    let mut drag_start_mouse = use_signal(|| (0.0_f64, 0.0_f64));
    let mut drag_start_pos = use_signal(|| (0.0_f64, 0.0_f64));

    // Track if we were just interacting (to prevent close on mouse up)
    let mut was_interacting = use_signal(|| false);

    // Determine current cursor based on active resize or drag
    let current_edge = *resize_edge.read();
    let dragging = *is_dragging.read();
    let cursor = if dragging {
        "move"
    } else {
        current_edge.cursor_style()
    };

    // Mouse move handler for resize and drag tracking
    let on_backdrop_mousemove = move |evt: Event<MouseData>| {
        let edge = *resize_edge.read();
        let dragging = *is_dragging.read();

        if edge != ResizeEdge::None {
            // Handle resize
            let (start_mx, start_my) = *resize_start_mouse.read();
            let (start_w, start_h) = *resize_start_size.read();
            let (start_ox, start_oy) = *resize_start_pos.read();

            let current_x = evt.page_coordinates().x;
            let current_y = evt.page_coordinates().y;
            let delta_x = current_x - start_mx;
            let delta_y = current_y - start_my;

            // Calculate new size based on edge being dragged
            let mut new_width = start_w;
            let mut new_height = start_h;
            let mut new_offset_x = start_ox;
            let mut new_offset_y = start_oy;

            match edge {
                ResizeEdge::Right => {
                    new_width = start_w + delta_x;
                }
                ResizeEdge::Left => {
                    new_width = start_w - delta_x;
                    new_offset_x = start_ox + delta_x;
                }
                ResizeEdge::Bottom => {
                    new_height = start_h + delta_y;
                }
                ResizeEdge::Top => {
                    new_height = start_h - delta_y;
                    new_offset_y = start_oy + delta_y;
                }
                ResizeEdge::BottomRight => {
                    new_width = start_w + delta_x;
                    new_height = start_h + delta_y;
                }
                ResizeEdge::BottomLeft => {
                    new_width = start_w - delta_x;
                    new_height = start_h + delta_y;
                    new_offset_x = start_ox + delta_x;
                }
                ResizeEdge::TopRight => {
                    new_width = start_w + delta_x;
                    new_height = start_h - delta_y;
                    new_offset_y = start_oy + delta_y;
                }
                ResizeEdge::TopLeft => {
                    new_width = start_w - delta_x;
                    new_height = start_h - delta_y;
                    new_offset_x = start_ox + delta_x;
                    new_offset_y = start_oy + delta_y;
                }
                ResizeEdge::None => {}
            }

            // Apply constraints
            let clamped_width = constraints.clamp_width(new_width);
            let clamped_height = constraints.clamp_height(new_height);

            // Adjust offset if we hit constraints
            if edge.moves_left() && clamped_width != new_width {
                new_offset_x = start_ox + (start_w - clamped_width);
            }
            if edge.moves_top() && clamped_height != new_height {
                new_offset_y = start_oy + (start_h - clamped_height);
            }

            width.set(clamped_width);
            height.set(clamped_height);
            dialog_offset_x.set(new_offset_x);
            dialog_offset_y.set(new_offset_y);
        } else if dragging {
            // Handle drag (move)
            let (start_mx, start_my) = *drag_start_mouse.read();
            let (start_ox, start_oy) = *drag_start_pos.read();

            let current_x = evt.page_coordinates().x;
            let current_y = evt.page_coordinates().y;
            let delta_x = current_x - start_mx;
            let delta_y = current_y - start_my;

            dialog_offset_x.set(start_ox + delta_x);
            dialog_offset_y.set(start_oy + delta_y);
        }
    };

    // Mouse up handler to end resize or drag
    let on_backdrop_mouseup = move |_: Event<MouseData>| {
        if *resize_edge.read() != ResizeEdge::None || *is_dragging.read() {
            was_interacting.set(true);
        }
        resize_edge.set(ResizeEdge::None);
        is_dragging.set(false);
    };

    // Title for close button
    let title_clone = props.title.clone();

    // Read theme values
    let th = theme.read();
    let bg_secondary = th.bg_secondary();
    let border = th.border();
    let border_subtle = th.border_subtle();
    let text_primary = th.text_primary();
    let text_muted = th.text_muted();
    drop(th);

    // Dialog offset for transform
    let offset_x = *dialog_offset_x.read();
    let offset_y = *dialog_offset_y.read();
    let w = *width.read();
    let h = *height.read();

    // Helper to create resize handle mousedown handler
    let make_mousedown = move |edge: ResizeEdge| {
        move |evt: Event<MouseData>| {
            evt.stop_propagation();
            resize_edge.set(edge);
            resize_start_mouse.set((evt.page_coordinates().x, evt.page_coordinates().y));
            resize_start_size.set((w, h));
            resize_start_pos.set((offset_x, offset_y));
        }
    };

    rsx! {
        // Backdrop - captures all mouse events during resize/drag
        div {
            class: "resizable-dialog-backdrop",
            style: "
                position: fixed;
                top: 0;
                left: 0;
                right: 0;
                bottom: 0;
                background: rgba(0, 0, 0, 0.6);
                display: flex;
                align-items: center;
                justify-content: center;
                z-index: 10000;
                cursor: {cursor};
            ",
            onclick: {
                let on_close = props.on_close.clone();
                move |_| {
                    // Only close if not resizing/dragging AND we weren't just interacting
                    if *resize_edge.read() == ResizeEdge::None &&
                       !*is_dragging.read() &&
                       !*was_interacting.read()
                    {
                        on_close.call(());
                    }
                    // Reset the was_interacting flag after checking
                    was_interacting.set(false);
                }
            },
            onmousemove: on_backdrop_mousemove,
            onmouseup: on_backdrop_mouseup,

            // Dialog container
            div {
                class: "resizable-dialog",
                style: "
                    position: relative;
                    width: {w}px;
                    height: {h}px;
                    transform: translate({offset_x}px, {offset_y}px);
                    display: flex;
                    flex-direction: column;
                    background: {bg_secondary};
                    border: 1px solid {border};
                    border-radius: 8px;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.4);
                    overflow: visible;
                ",
                // Prevent clicks inside from closing
                onclick: move |e| e.stop_propagation(),

                // =========================================================
                // RESIZE HANDLES - explicit invisible divs at edges/corners
                // =========================================================

                // Top edge
                div {
                    style: "
                        position: absolute;
                        top: -4px;
                        left: 14px;
                        right: 14px;
                        height: 8px;
                        cursor: n-resize;
                        z-index: 10;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::Top),
                }

                // Bottom edge
                div {
                    style: "
                        position: absolute;
                        bottom: -4px;
                        left: 14px;
                        right: 14px;
                        height: 8px;
                        cursor: s-resize;
                        z-index: 10;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::Bottom),
                }

                // Left edge
                div {
                    style: "
                        position: absolute;
                        left: -4px;
                        top: 14px;
                        bottom: 14px;
                        width: 8px;
                        cursor: w-resize;
                        z-index: 10;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::Left),
                }

                // Right edge
                div {
                    style: "
                        position: absolute;
                        right: -4px;
                        top: 14px;
                        bottom: 14px;
                        width: 8px;
                        cursor: e-resize;
                        z-index: 10;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::Right),
                }

                // Top-left corner
                div {
                    style: "
                        position: absolute;
                        top: -4px;
                        left: -4px;
                        width: 18px;
                        height: 18px;
                        cursor: nw-resize;
                        z-index: 11;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::TopLeft),
                }

                // Top-right corner
                div {
                    style: "
                        position: absolute;
                        top: -4px;
                        right: -4px;
                        width: 18px;
                        height: 18px;
                        cursor: ne-resize;
                        z-index: 11;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::TopRight),
                }

                // Bottom-left corner
                div {
                    style: "
                        position: absolute;
                        bottom: -4px;
                        left: -4px;
                        width: 18px;
                        height: 18px;
                        cursor: sw-resize;
                        z-index: 11;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::BottomLeft),
                }

                // Bottom-right corner
                div {
                    style: "
                        position: absolute;
                        bottom: -4px;
                        right: -4px;
                        width: 18px;
                        height: 18px;
                        cursor: se-resize;
                        z-index: 11;
                    ",
                    onmousedown: make_mousedown(ResizeEdge::BottomRight),
                }

                // =========================================================
                // DIALOG CONTENT
                // =========================================================

                // Dialog header - DRAGGABLE for moving the dialog
                div {
                    style: "
                        display: flex;
                        justify-content: space-between;
                        align-items: center;
                        padding: 14px 20px;
                        border-bottom: 1px solid {border_subtle};
                        flex-shrink: 0;
                        cursor: move;
                        user-select: none;
                    ",
                    // Start drag on mouse down
                    onmousedown: move |evt: Event<MouseData>| {
                        evt.stop_propagation();
                        is_dragging.set(true);
                        drag_start_mouse.set((evt.page_coordinates().x, evt.page_coordinates().y));
                        drag_start_pos.set((offset_x, offset_y));
                    },

                    h2 {
                        style: "
                            margin: 0;
                            font-size: 16px;
                            font-weight: 600;
                            color: {text_primary};
                            pointer-events: none;
                        ",
                        "{title_clone}"
                    }

                    button {
                        style: "
                            background: none;
                            border: none;
                            color: {text_muted};
                            font-size: 20px;
                            cursor: pointer;
                            padding: 4px 8px;
                            border-radius: 4px;
                            line-height: 1;
                        ",
                        onclick: move |evt| {
                            evt.stop_propagation();
                            props.on_close.call(());
                        },
                        // Prevent drag from starting on close button
                        onmousedown: move |evt: Event<MouseData>| {
                            evt.stop_propagation();
                        },
                        title: "Close",
                        "×"
                    }
                }

                // Dialog body - scrollable content area
                div {
                    style: "
                        flex: 1;
                        overflow: auto;
                        display: flex;
                        flex-direction: column;
                    ",
                    {props.children}
                }
            }
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // -------------------------------------------------------------------------
    // ResizeEdge Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_resize_edge_default() {
        let edge = ResizeEdge::default();
        assert_eq!(edge, ResizeEdge::None);
    }

    #[test]
    fn test_resize_edge_all_variants() {
        let all = ResizeEdge::all();
        assert_eq!(all.len(), 8);
        assert!(!all.contains(&ResizeEdge::None));
    }

    #[test]
    fn test_resize_edge_cursor_styles() {
        assert_eq!(ResizeEdge::None.cursor_style(), "default");
        assert_eq!(ResizeEdge::Top.cursor_style(), "n-resize");
        assert_eq!(ResizeEdge::Bottom.cursor_style(), "s-resize");
        assert_eq!(ResizeEdge::Left.cursor_style(), "w-resize");
        assert_eq!(ResizeEdge::Right.cursor_style(), "e-resize");
        assert_eq!(ResizeEdge::TopLeft.cursor_style(), "nw-resize");
        assert_eq!(ResizeEdge::TopRight.cursor_style(), "ne-resize");
        assert_eq!(ResizeEdge::BottomLeft.cursor_style(), "sw-resize");
        assert_eq!(ResizeEdge::BottomRight.cursor_style(), "se-resize");
    }

    #[test]
    fn test_resize_edge_from_position_center() {
        // Point in center should return None
        let edge = ResizeEdge::from_position(100.0, 100.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::None);
    }

    #[test]
    fn test_resize_edge_from_position_top() {
        // Point at top edge (not corner)
        let edge = ResizeEdge::from_position(100.0, 3.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::Top);
    }

    #[test]
    fn test_resize_edge_from_position_bottom() {
        let edge = ResizeEdge::from_position(100.0, 197.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::Bottom);
    }

    #[test]
    fn test_resize_edge_from_position_left() {
        let edge = ResizeEdge::from_position(3.0, 100.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::Left);
    }

    #[test]
    fn test_resize_edge_from_position_right() {
        let edge = ResizeEdge::from_position(197.0, 100.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::Right);
    }

    #[test]
    fn test_resize_edge_from_position_top_left_corner() {
        let edge = ResizeEdge::from_position(5.0, 5.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::TopLeft);
    }

    #[test]
    fn test_resize_edge_from_position_top_right_corner() {
        let edge = ResizeEdge::from_position(195.0, 5.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::TopRight);
    }

    #[test]
    fn test_resize_edge_from_position_bottom_left_corner() {
        let edge = ResizeEdge::from_position(5.0, 195.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::BottomLeft);
    }

    #[test]
    fn test_resize_edge_from_position_bottom_right_corner() {
        let edge = ResizeEdge::from_position(195.0, 195.0, 200.0, 200.0, 8.0, 14.0);
        assert_eq!(edge, ResizeEdge::BottomRight);
    }

    #[test]
    fn test_resize_edge_affects_width() {
        assert!(!ResizeEdge::None.affects_width());
        assert!(!ResizeEdge::Top.affects_width());
        assert!(!ResizeEdge::Bottom.affects_width());
        assert!(ResizeEdge::Left.affects_width());
        assert!(ResizeEdge::Right.affects_width());
        assert!(ResizeEdge::TopLeft.affects_width());
        assert!(ResizeEdge::TopRight.affects_width());
        assert!(ResizeEdge::BottomLeft.affects_width());
        assert!(ResizeEdge::BottomRight.affects_width());
    }

    #[test]
    fn test_resize_edge_affects_height() {
        assert!(!ResizeEdge::None.affects_height());
        assert!(ResizeEdge::Top.affects_height());
        assert!(ResizeEdge::Bottom.affects_height());
        assert!(!ResizeEdge::Left.affects_height());
        assert!(!ResizeEdge::Right.affects_height());
        assert!(ResizeEdge::TopLeft.affects_height());
        assert!(ResizeEdge::TopRight.affects_height());
        assert!(ResizeEdge::BottomLeft.affects_height());
        assert!(ResizeEdge::BottomRight.affects_height());
    }

    #[test]
    fn test_resize_edge_moves_left() {
        assert!(ResizeEdge::Left.moves_left());
        assert!(ResizeEdge::TopLeft.moves_left());
        assert!(ResizeEdge::BottomLeft.moves_left());
        assert!(!ResizeEdge::Right.moves_left());
        assert!(!ResizeEdge::Top.moves_left());
        assert!(!ResizeEdge::Bottom.moves_left());
    }

    #[test]
    fn test_resize_edge_moves_top() {
        assert!(ResizeEdge::Top.moves_top());
        assert!(ResizeEdge::TopLeft.moves_top());
        assert!(ResizeEdge::TopRight.moves_top());
        assert!(!ResizeEdge::Bottom.moves_top());
        assert!(!ResizeEdge::Left.moves_top());
        assert!(!ResizeEdge::Right.moves_top());
    }

    // -------------------------------------------------------------------------
    // DialogSizeConstraints Tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_dialog_size_constraints_default() {
        let constraints = DialogSizeConstraints::default();
        assert_eq!(constraints.min_width, 400.0);
        assert_eq!(constraints.min_height, 300.0);
        assert_eq!(constraints.max_width, 2000.0);
        assert_eq!(constraints.max_height, 1500.0);
    }

    #[test]
    fn test_dialog_size_constraints_clamp_width() {
        let constraints = DialogSizeConstraints {
            min_width: 200.0,
            min_height: 150.0,
            max_width: 800.0,
            max_height: 600.0,
        };

        assert_eq!(constraints.clamp_width(100.0), 200.0); // Below min
        assert_eq!(constraints.clamp_width(500.0), 500.0); // Within range
        assert_eq!(constraints.clamp_width(1000.0), 800.0); // Above max
    }

    #[test]
    fn test_dialog_size_constraints_clamp_height() {
        let constraints = DialogSizeConstraints {
            min_width: 200.0,
            min_height: 150.0,
            max_width: 800.0,
            max_height: 600.0,
        };

        assert_eq!(constraints.clamp_height(100.0), 150.0); // Below min
        assert_eq!(constraints.clamp_height(400.0), 400.0); // Within range
        assert_eq!(constraints.clamp_height(800.0), 600.0); // Above max
    }

    #[test]
    fn test_dialog_size_constraints_clamp_at_boundaries() {
        let constraints = DialogSizeConstraints {
            min_width: 100.0,
            min_height: 100.0,
            max_width: 500.0,
            max_height: 500.0,
        };

        // Exact min/max values should pass through unchanged
        assert_eq!(constraints.clamp_width(100.0), 100.0);
        assert_eq!(constraints.clamp_width(500.0), 500.0);
        assert_eq!(constraints.clamp_height(100.0), 100.0);
        assert_eq!(constraints.clamp_height(500.0), 500.0);
    }
}

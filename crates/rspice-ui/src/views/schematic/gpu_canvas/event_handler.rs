//! Schematic Event Handler
//!
//! Commercial-grade event handling for the GPU schematic canvas.
//! Follows professional EDA patterns with separated, testable logic.
//!
//! # Architecture
//!
//! Event handling is separated from rendering for:
//! - **Testability**: All event logic can be unit tested without GPU
//! - **Maintainability**: Clear responsibility boundaries
//! - **Performance**: State changes computed once per event
//!
//! # Event Flow
//!
//! ```text
//! DOM Event → EventHandler.process() → EventResult → Canvas updates state
//! ```

use crate::gpu::hit_test::{HitResult, HitTester};
use crate::gpu::integration::screen_to_world;
use crate::state::{Component, ComponentType, Point, Rotation, SchematicState, Tool, Wire};

// =============================================================================
// Event Types (Input)
// =============================================================================

/// Mouse button enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
}

/// Keyboard modifier flags
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Modifiers {
    pub ctrl: bool,
    pub shift: bool,
    pub alt: bool,
}

/// Mouse event data
#[derive(Debug, Clone, Copy)]
pub struct MouseEvent {
    /// Screen coordinates (pixels from top-left)
    pub screen_x: f64,
    pub screen_y: f64,
    /// World coordinates (schematic units)
    pub world_x: f32,
    pub world_y: f32,
    /// Grid-snapped coordinates
    pub grid: Point,
    /// Button that triggered this event (for click/down/up)
    pub button: Option<MouseButton>,
    /// Wheel delta (for scroll events)
    pub wheel_delta: f64,
    /// Modifier keys held
    pub modifiers: Modifiers,
}

impl MouseEvent {
    /// Create a new mouse event from screen coordinates
    pub fn new(
        screen_x: f64,
        screen_y: f64,
        pan: (f64, f64),
        zoom: f64,
        grid_size: i32,
    ) -> Self {
        let (world_x, world_y) = screen_to_world(screen_x, screen_y, pan.0, pan.1, zoom);
        let grid = Point::new(
            ((world_x / grid_size as f32).round() as i32) * grid_size,
            ((world_y / grid_size as f32).round() as i32) * grid_size,
        );
        Self {
            screen_x,
            screen_y,
            world_x,
            world_y,
            grid,
            button: None,
            wheel_delta: 0.0,
            modifiers: Modifiers::default(),
        }
    }

    /// Set mouse button
    pub fn with_button(mut self, button: MouseButton) -> Self {
        self.button = Some(button);
        self
    }

    /// Set wheel delta
    pub fn with_wheel(mut self, delta: f64) -> Self {
        self.wheel_delta = delta;
        self
    }

    /// Set modifiers
    pub fn with_modifiers(mut self, modifiers: Modifiers) -> Self {
        self.modifiers = modifiers;
        self
    }
}

// =============================================================================
// Interaction State
// =============================================================================

/// Current drag operation
#[derive(Debug, Clone, Default)]
pub struct DragOperation {
    /// Whether a drag is active
    pub active: bool,
    /// Starting grid position
    pub start_grid: Point,
    /// Current grid position
    pub current_grid: Point,
    /// Component being dragged (if any)
    pub component_id: Option<u64>,
    /// Wire being dragged (if any)
    pub wire_id: Option<u64>,
    /// Junction point being dragged
    pub junction_point: Option<Point>,
    /// Whether dragging multiple selected items
    pub multi_selection: bool,
}

impl DragOperation {
    /// Calculate the delta from start to current
    pub fn delta(&self) -> Point {
        Point::new(
            self.current_grid.x - self.start_grid.x,
            self.current_grid.y - self.start_grid.y,
        )
    }

    /// Check if this is a significant drag (not just a click)
    pub fn is_significant(&self) -> bool {
        self.active && (self.start_grid != self.current_grid)
    }
}

/// Box selection state
#[derive(Debug, Clone, Default)]
pub struct BoxSelection {
    /// Whether box selection is active
    pub active: bool,
    /// Starting grid position
    pub start_grid: Point,
    /// Ending grid position
    pub end_grid: Point,
    /// Starting screen position (for visual rendering)
    pub start_screen: (f64, f64),
    /// Ending screen position
    pub end_screen: (f64, f64),
}

impl BoxSelection {
    /// Get the bounding box in grid coordinates
    pub fn bounds(&self) -> (Point, Point) {
        let min = Point::new(
            self.start_grid.x.min(self.end_grid.x),
            self.start_grid.y.min(self.end_grid.y),
        );
        let max = Point::new(
            self.start_grid.x.max(self.end_grid.x),
            self.start_grid.y.max(self.end_grid.y),
        );
        (min, max)
    }
}

/// Pan operation state
#[derive(Debug, Clone, Default)]
pub struct PanOperation {
    /// Whether panning is active
    pub active: bool,
    /// Last mouse position for delta calculation
    pub last_screen: (f64, f64),
}

/// Complete interaction state for the canvas
#[derive(Debug, Clone, Default)]
pub struct InteractionState {
    /// Current drag operation
    pub drag: DragOperation,
    /// Current box selection
    pub box_selection: BoxSelection,
    /// Current pan operation
    pub pan: PanOperation,
    /// Last known mouse grid position
    pub mouse_grid: Point,
    /// Hovered wire segment (for highlight)
    pub hovered_wire: Option<u64>,
    /// Hovered component (for highlight)
    pub hovered_component: Option<u64>,
    /// Hovered junction (for corner drag)
    pub hovered_junction: Option<Point>,
}

impl InteractionState {
    /// Check if any operation is active
    pub fn is_busy(&self) -> bool {
        self.drag.active || self.box_selection.active || self.pan.active
    }

    /// Reset all interaction state
    pub fn reset(&mut self) {
        self.drag = DragOperation::default();
        self.box_selection = BoxSelection::default();
        self.pan = PanOperation::default();
    }
}

// =============================================================================
// Event Results (Output)
// =============================================================================

/// Actions that can result from event processing
#[derive(Debug, Clone)]
pub enum EventAction {
    /// No action needed
    None,
    
    /// Update pan position
    Pan { delta_x: f64, delta_y: f64 },
    
    /// Update zoom at a point
    Zoom { center_x: f64, center_y: f64, delta: f64 },
    
    /// Select a component
    SelectComponent { id: u64, add_to_selection: bool },
    
    /// Select a wire
    SelectWire { id: u64, add_to_selection: bool },
    
    /// Select multiple items by box
    SelectBox { component_ids: Vec<u64>, wire_ids: Vec<u64> },
    
    /// Clear selection
    ClearSelection,
    
    /// Start dragging a component
    StartDragComponent { id: u64, start_grid: Point },
    
    /// Start dragging a wire
    StartDragWire { id: u64, start_grid: Point },
    
    /// Start dragging selected items
    StartDragSelection { start_grid: Point },
    
    /// Start dragging a junction point
    StartDragJunction { point: Point },
    
    /// Update drag position
    UpdateDrag { current_grid: Point },
    
    /// Complete drag operation
    CompleteDrag { delta: Point },
    
    /// Cancel drag operation
    CancelDrag,
    
    /// Place a component at position
    PlaceComponent { kind: ComponentType, pos: Point, rotation: Rotation },
    
    /// Start wire drawing
    StartWire { pos: Point },
    
    /// Add wire segment
    AddWireSegment { pos: Point },
    
    /// Complete wire drawing
    CompleteWire,
    
    /// Cancel wire drawing
    CancelWire,
    
    /// Show context menu
    ShowContextMenu { screen_x: f64, screen_y: f64, world: Point },
    
    /// Delete selected items
    DeleteSelection,
    
    /// Rotate preview (for placement)
    RotatePreview,
    
    /// Update wire preview position
    UpdateWirePreview { pos: Point },
    
    /// Open component edit dialog
    EditComponent { id: u64 },
    
    /// Multiple actions
    Multiple(Vec<EventAction>),
}

impl EventAction {
    /// Check if this is the None action
    pub fn is_none(&self) -> bool {
        matches!(self, EventAction::None)
    }

    /// Combine two actions
    pub fn combine(self, other: EventAction) -> EventAction {
        match (self, other) {
            (EventAction::None, action) | (action, EventAction::None) => action,
            (EventAction::Multiple(mut actions), EventAction::Multiple(others)) => {
                actions.extend(others);
                EventAction::Multiple(actions)
            }
            (EventAction::Multiple(mut actions), action) => {
                actions.push(action);
                EventAction::Multiple(actions)
            }
            (action, EventAction::Multiple(mut actions)) => {
                actions.insert(0, action);
                EventAction::Multiple(actions)
            }
            (action1, action2) => EventAction::Multiple(vec![action1, action2]),
        }
    }
}

// =============================================================================
// Event Handler
// =============================================================================

/// Configuration for event handling behavior
#[derive(Debug, Clone)]
pub struct EventHandlerConfig {
    /// Minimum drag distance (in grid units) before drag activates  
    pub drag_threshold: i32,
    /// Double-click timeout in milliseconds
    pub double_click_ms: u64,
    /// Zoom sensitivity (multiplier per wheel unit)
    pub zoom_sensitivity: f64,
    /// Whether to prefer terminal hits over component body
    pub prefer_terminals: bool,
}

impl Default for EventHandlerConfig {
    fn default() -> Self {
        Self {
            drag_threshold: 0,
            double_click_ms: 300,
            zoom_sensitivity: 0.1,
            prefer_terminals: true,
        }
    }
}

/// Schematic event handler
///
/// Processes input events and produces actions for the canvas to execute.
/// This separation allows complete unit testing of event logic.
pub struct SchematicEventHandler {
    /// Hit tester for picking
    hit_tester: HitTester,
    /// Configuration
    config: EventHandlerConfig,
    /// Current interaction state
    pub state: InteractionState,
}

impl Default for SchematicEventHandler {
    fn default() -> Self {
        Self::new(EventHandlerConfig::default())
    }
}

impl SchematicEventHandler {
    /// Create a new event handler
    pub fn new(config: EventHandlerConfig) -> Self {
        Self {
            hit_tester: HitTester::default(),
            config,
            state: InteractionState::default(),
        }
    }

    /// Process mouse down event
    pub fn on_mouse_down(
        &mut self,
        event: &MouseEvent,
        schematic: &SchematicState,
    ) -> EventAction {
        match event.button {
            Some(MouseButton::Left) => self.on_left_down(event, schematic),
            Some(MouseButton::Middle) => self.on_middle_down(event),
            Some(MouseButton::Right) => self.on_right_down(event, schematic),
            None => EventAction::None,
        }
    }

    /// Process mouse up event
    pub fn on_mouse_up(
        &mut self,
        event: &MouseEvent,
        schematic: &SchematicState,
    ) -> EventAction {
        match event.button {
            Some(MouseButton::Left) => self.on_left_up(event, schematic),
            Some(MouseButton::Middle) => self.on_middle_up(event),
            Some(MouseButton::Right) => EventAction::None,
            None => EventAction::None,
        }
    }

    /// Process mouse move event
    pub fn on_mouse_move(
        &mut self,
        event: &MouseEvent,
        schematic: &SchematicState,
    ) -> EventAction {
        self.state.mouse_grid = event.grid;

        if self.state.pan.active {
            return self.handle_pan_move(event);
        }

        if self.state.drag.active {
            return self.handle_drag_move(event);
        }

        if self.state.box_selection.active {
            return self.handle_box_selection_move(event, schematic);
        }

        // Hover detection
        self.update_hover(event, schematic);

        // Wire preview update
        if schematic.wire_drawing.active {
            return EventAction::UpdateWirePreview { pos: event.grid };
        }

        EventAction::None
    }

    /// Process mouse wheel event
    pub fn on_wheel(&mut self, event: &MouseEvent) -> EventAction {
        let zoom_delta = -event.wheel_delta * self.config.zoom_sensitivity;
        EventAction::Zoom {
            center_x: event.screen_x,
            center_y: event.screen_y,
            delta: zoom_delta,
        }
    }

    /// Process key down event
    pub fn on_key_down(
        &mut self,
        key: &str,
        modifiers: Modifiers,
        schematic: &SchematicState,
    ) -> EventAction {
        match key {
            "Escape" => self.on_escape(schematic),
            "Delete" | "Backspace" => EventAction::DeleteSelection,
            "r" | "R" => EventAction::RotatePreview,
            _ => EventAction::None,
        }
    }

    // =========================================================================
    // Left Mouse Button
    // =========================================================================

    fn on_left_down(&mut self, event: &MouseEvent, schematic: &SchematicState) -> EventAction {
        match schematic.tool {
            Tool::Select => self.select_tool_down(event, schematic),
            Tool::Wire => self.wire_tool_down(event, schematic),
            Tool::Place(kind) => self.place_tool_down(event, kind, schematic.preview_rotation),
            Tool::Probe => EventAction::None, // Probe uses hover, not click
            Tool::Label => EventAction::None, // Label tool handled separately
        }
    }

    fn on_left_up(&mut self, event: &MouseEvent, schematic: &SchematicState) -> EventAction {
        if self.state.drag.active {
            return self.complete_drag();
        }

        if self.state.box_selection.active {
            return self.complete_box_selection(schematic);
        }

        EventAction::None
    }

    fn select_tool_down(&mut self, event: &MouseEvent, schematic: &SchematicState) -> EventAction {
        // Hit test to see what was clicked
        let hit = self.hit_tester.test_all(
            &schematic.components,
            &schematic.wires,
            event.world_x,
            event.world_y,
        );

        match hit {
            HitResult::Component { id, .. } => {
                let add_to_selection = event.modifiers.shift || event.modifiers.ctrl;
                let is_selected = schematic.selection.has_component(id);

                if is_selected && !add_to_selection {
                    // Start dragging selected items
                    self.start_selection_drag(event, schematic)
                } else {
                    // Select this component and maybe start dragging
                    self.state.drag = DragOperation {
                        active: true,
                        start_grid: event.grid,
                        current_grid: event.grid,
                        component_id: Some(id),
                        wire_id: None,
                        junction_point: None,
                        multi_selection: false,
                    };
                    EventAction::SelectComponent { id, add_to_selection }
                }
            }
            HitResult::Wire { id, .. } => {
                let add_to_selection = event.modifiers.shift || event.modifiers.ctrl;
                EventAction::SelectWire { id, add_to_selection }
            }
            HitResult::Terminal { component_id, .. } => {
                // Terminal click - could start wire or select component
                EventAction::SelectComponent {
                    id: component_id,
                    add_to_selection: event.modifiers.shift,
                }
            }
            HitResult::Junction { position, .. } => {
                // Junction drag
                self.state.drag = DragOperation {
                    active: true,
                    start_grid: event.grid,
                    current_grid: event.grid,
                    component_id: None,
                    wire_id: None,
                    junction_point: Some(position),
                    multi_selection: false,
                };
                EventAction::StartDragJunction { point: position }
            }
            HitResult::None => {
                // Start box selection
                self.state.box_selection = BoxSelection {
                    active: true,
                    start_grid: event.grid,
                    end_grid: event.grid,
                    start_screen: (event.screen_x, event.screen_y),
                    end_screen: (event.screen_x, event.screen_y),
                };
                if !event.modifiers.shift {
                    EventAction::ClearSelection
                } else {
                    EventAction::None
                }
            }
        }
    }

    fn wire_tool_down(&mut self, event: &MouseEvent, schematic: &SchematicState) -> EventAction {
        if schematic.wire_drawing.active {
            EventAction::AddWireSegment { pos: event.grid }
        } else {
            EventAction::StartWire { pos: event.grid }
        }
    }

    fn place_tool_down(
        &mut self,
        event: &MouseEvent,
        kind: ComponentType,
        rotation: Rotation,
    ) -> EventAction {
        EventAction::PlaceComponent {
            kind,
            pos: event.grid,
            rotation,
        }
    }

    fn start_selection_drag(
        &mut self,
        event: &MouseEvent,
        schematic: &SchematicState,
    ) -> EventAction {
        self.state.drag = DragOperation {
            active: true,
            start_grid: event.grid,
            current_grid: event.grid,
            component_id: None,
            wire_id: None,
            junction_point: None,
            multi_selection: true,
        };
        EventAction::StartDragSelection { start_grid: event.grid }
    }

    // =========================================================================
    // Middle Mouse Button (Pan)
    // =========================================================================

    fn on_middle_down(&mut self, event: &MouseEvent) -> EventAction {
        self.state.pan = PanOperation {
            active: true,
            last_screen: (event.screen_x, event.screen_y),
        };
        EventAction::None
    }

    fn on_middle_up(&mut self, _event: &MouseEvent) -> EventAction {
        self.state.pan.active = false;
        EventAction::None
    }

    fn handle_pan_move(&mut self, event: &MouseEvent) -> EventAction {
        let (last_x, last_y) = self.state.pan.last_screen;
        let delta_x = event.screen_x - last_x;
        let delta_y = event.screen_y - last_y;
        self.state.pan.last_screen = (event.screen_x, event.screen_y);
        EventAction::Pan { delta_x, delta_y }
    }

    // =========================================================================
    // Right Mouse Button (Context Menu)
    // =========================================================================

    fn on_right_down(&mut self, event: &MouseEvent, _schematic: &SchematicState) -> EventAction {
        EventAction::ShowContextMenu {
            screen_x: event.screen_x,
            screen_y: event.screen_y,
            world: event.grid,
        }
    }

    // =========================================================================
    // Drag Operations
    // =========================================================================

    fn handle_drag_move(&mut self, event: &MouseEvent) -> EventAction {
        self.state.drag.current_grid = event.grid;
        EventAction::UpdateDrag { current_grid: event.grid }
    }

    fn complete_drag(&mut self) -> EventAction {
        let delta = self.state.drag.delta();
        self.state.drag.active = false;

        if delta.x == 0 && delta.y == 0 {
            EventAction::CancelDrag
        } else {
            EventAction::CompleteDrag { delta }
        }
    }

    // =========================================================================
    // Box Selection
    // =========================================================================

    fn handle_box_selection_move(
        &mut self,
        event: &MouseEvent,
        schematic: &SchematicState,
    ) -> EventAction {
        self.state.box_selection.end_grid = event.grid;
        self.state.box_selection.end_screen = (event.screen_x, event.screen_y);

        // Calculate selection in real-time
        let (min, max) = self.state.box_selection.bounds();

        let component_ids: Vec<u64> = schematic
            .components
            .iter()
            .filter(|c| c.pos.x >= min.x && c.pos.x <= max.x && c.pos.y >= min.y && c.pos.y <= max.y)
            .map(|c| c.id)
            .collect();

        let wire_ids: Vec<u64> = schematic
            .wires
            .iter()
            .filter(|w| {
                w.points.iter().any(|p| p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y)
            })
            .map(|w| w.id)
            .collect();

        EventAction::SelectBox { component_ids, wire_ids }
    }

    fn complete_box_selection(&mut self, schematic: &SchematicState) -> EventAction {
        let (min, max) = self.state.box_selection.bounds();
        self.state.box_selection.active = false;

        let component_ids: Vec<u64> = schematic
            .components
            .iter()
            .filter(|c| c.pos.x >= min.x && c.pos.x <= max.x && c.pos.y >= min.y && c.pos.y <= max.y)
            .map(|c| c.id)
            .collect();

        let wire_ids: Vec<u64> = schematic
            .wires
            .iter()
            .filter(|w| {
                w.points.iter().any(|p| p.x >= min.x && p.x <= max.x && p.y >= min.y && p.y <= max.y)
            })
            .map(|w| w.id)
            .collect();

        EventAction::SelectBox { component_ids, wire_ids }
    }

    // =========================================================================
    // Hover Detection
    // =========================================================================

    fn update_hover(&mut self, event: &MouseEvent, schematic: &SchematicState) {
        let hit = self.hit_tester.test_all(
            &schematic.components,
            &schematic.wires,
            event.world_x,
            event.world_y,
        );

        self.state.hovered_component = match &hit {
            HitResult::Component { id, .. } => Some(*id),
            HitResult::Terminal { component_id, .. } => Some(*component_id),
            _ => None,
        };

        self.state.hovered_wire = match &hit {
            HitResult::Wire { id, .. } => Some(*id),
            _ => None,
        };

        // Check for junction hover (wire endpoints)
        self.state.hovered_junction = None;
        if matches!(schematic.tool, Tool::Select) {
            for wire in &schematic.wires {
                if let Some(first) = wire.points.first() {
                    if *first == event.grid {
                        self.state.hovered_junction = Some(*first);
                        break;
                    }
                }
                if let Some(last) = wire.points.last() {
                    if *last == event.grid {
                        self.state.hovered_junction = Some(*last);
                        break;
                    }
                }
            }
        }
    }

    // =========================================================================
    // Escape Key
    // =========================================================================

    fn on_escape(&mut self, schematic: &SchematicState) -> EventAction {
        // Cancel in priority order
        if self.state.drag.active {
            self.state.drag.active = false;
            return EventAction::CancelDrag;
        }

        if self.state.box_selection.active {
            self.state.box_selection.active = false;
            return EventAction::ClearSelection;
        }

        if schematic.wire_drawing.active {
            return EventAction::CancelWire;
        }

        EventAction::ClearSelection
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    // =========================================================================
    // Test Helpers
    // =========================================================================

    fn make_handler() -> SchematicEventHandler {
        SchematicEventHandler::default()
    }

    fn make_event(screen_x: f64, screen_y: f64, grid_x: i32, grid_y: i32) -> MouseEvent {
        MouseEvent {
            screen_x,
            screen_y,
            world_x: grid_x as f32,
            world_y: grid_y as f32,
            grid: Point::new(grid_x, grid_y),
            button: None,
            wheel_delta: 0.0,
            modifiers: Modifiers::default(),
        }
    }

    fn make_schematic() -> SchematicState {
        let mut sch = SchematicState::default();
        sch.components.push(Component::new(1, ComponentType::Resistor, Point::new(10, 10)));
        sch.components.push(Component::new(2, ComponentType::Capacitor, Point::new(30, 10)));
        sch.wires.push(Wire::new(1, vec![Point::new(0, 0), Point::new(20, 0)]));
        sch
    }

    // =========================================================================
    // MouseEvent Tests
    // =========================================================================

    #[test]
    fn test_mouse_event_new() {
        let evt = MouseEvent::new(100.0, 100.0, (0.0, 0.0), 1.0, 10);
        assert_eq!(evt.screen_x, 100.0);
        assert_eq!(evt.world_x, 100.0);
        assert_eq!(evt.grid.x, 100);
    }

    #[test]
    fn test_mouse_event_with_pan() {
        let evt = MouseEvent::new(100.0, 100.0, (50.0, 50.0), 1.0, 10);
        assert_eq!(evt.world_x, 50.0);
        assert_eq!(evt.grid.x, 50);
    }

    #[test]
    fn test_mouse_event_with_zoom() {
        let evt = MouseEvent::new(100.0, 100.0, (0.0, 0.0), 2.0, 10);
        assert_eq!(evt.world_x, 50.0);
        assert_eq!(evt.grid.x, 50);
    }

    #[test]
    fn test_mouse_event_grid_snap() {
        let evt = MouseEvent::new(15.0, 15.0, (0.0, 0.0), 1.0, 10);
        assert_eq!(evt.grid.x, 20);  // 15 rounds to 20
        assert_eq!(evt.grid.y, 20);
    }

    #[test]
    fn test_mouse_event_with_button() {
        let evt = make_event(0.0, 0.0, 0, 0).with_button(MouseButton::Left);
        assert_eq!(evt.button, Some(MouseButton::Left));
    }

    #[test]
    fn test_mouse_event_with_modifiers() {
        let mods = Modifiers { ctrl: true, shift: false, alt: false };
        let evt = make_event(0.0, 0.0, 0, 0).with_modifiers(mods);
        assert!(evt.modifiers.ctrl);
    }

    // =========================================================================
    // DragOperation Tests
    // =========================================================================

    #[test]
    fn test_drag_operation_delta() {
        let drag = DragOperation {
            active: true,
            start_grid: Point::new(10, 10),
            current_grid: Point::new(15, 20),
            ..Default::default()
        };
        let delta = drag.delta();
        assert_eq!(delta.x, 5);
        assert_eq!(delta.y, 10);
    }

    #[test]
    fn test_drag_operation_is_significant() {
        let mut drag = DragOperation::default();
        assert!(!drag.is_significant());

        drag.active = true;
        drag.start_grid = Point::new(0, 0);
        drag.current_grid = Point::new(0, 0);
        assert!(!drag.is_significant()); // Same position

        drag.current_grid = Point::new(1, 0);
        assert!(drag.is_significant());
    }

    // =========================================================================
    // BoxSelection Tests
    // =========================================================================

    #[test]
    fn test_box_selection_bounds() {
        let box_sel = BoxSelection {
            active: true,
            start_grid: Point::new(20, 30),
            end_grid: Point::new(10, 40),
            ..Default::default()
        };
        let (min, max) = box_sel.bounds();
        assert_eq!(min.x, 10);
        assert_eq!(min.y, 30);
        assert_eq!(max.x, 20);
        assert_eq!(max.y, 40);
    }

    #[test]
    fn test_box_selection_bounds_negative() {
        let box_sel = BoxSelection {
            active: true,
            start_grid: Point::new(-10, -20),
            end_grid: Point::new(10, 20),
            ..Default::default()
        };
        let (min, max) = box_sel.bounds();
        assert_eq!(min.x, -10);
        assert_eq!(min.y, -20);
        assert_eq!(max.x, 10);
        assert_eq!(max.y, 20);
    }

    // =========================================================================
    // InteractionState Tests
    // =========================================================================

    #[test]
    fn test_interaction_state_is_busy() {
        let mut state = InteractionState::default();
        assert!(!state.is_busy());

        state.drag.active = true;
        assert!(state.is_busy());

        state.drag.active = false;
        state.box_selection.active = true;
        assert!(state.is_busy());

        state.box_selection.active = false;
        state.pan.active = true;
        assert!(state.is_busy());
    }

    #[test]
    fn test_interaction_state_reset() {
        let mut state = InteractionState::default();
        state.drag.active = true;
        state.box_selection.active = true;
        state.pan.active = true;

        state.reset();

        assert!(!state.is_busy());
    }

    // =========================================================================
    // EventAction Tests
    // =========================================================================

    #[test]
    fn test_event_action_is_none() {
        assert!(EventAction::None.is_none());
        assert!(!EventAction::ClearSelection.is_none());
    }

    #[test]
    fn test_event_action_combine_none() {
        let action = EventAction::ClearSelection;
        let combined = EventAction::None.combine(action.clone());
        assert!(matches!(combined, EventAction::ClearSelection));
    }

    #[test]
    fn test_event_action_combine_two_actions() {
        let a1 = EventAction::ClearSelection;
        let a2 = EventAction::DeleteSelection;
        let combined = a1.combine(a2);
        assert!(matches!(combined, EventAction::Multiple(ref v) if v.len() == 2));
    }

    // =========================================================================
    // SchematicEventHandler - Pan Tests
    // =========================================================================

    #[test]
    fn test_pan_start() {
        let mut handler = make_handler();
        let evt = make_event(100.0, 100.0, 10, 10).with_button(MouseButton::Middle);
        let sch = make_schematic();

        handler.on_mouse_down(&evt, &sch);

        assert!(handler.state.pan.active);
        assert_eq!(handler.state.pan.last_screen, (100.0, 100.0));
    }

    #[test]
    fn test_pan_move() {
        let mut handler = make_handler();
        let sch = make_schematic();

        // Start pan
        let down = make_event(100.0, 100.0, 10, 10).with_button(MouseButton::Middle);
        handler.on_mouse_down(&down, &sch);

        // Move
        let move_evt = make_event(120.0, 130.0, 12, 13);
        let action = handler.on_mouse_move(&move_evt, &sch);

        assert!(matches!(action, EventAction::Pan { delta_x, delta_y } if delta_x == 20.0 && delta_y == 30.0));
    }

    #[test]
    fn test_pan_end() {
        let mut handler = make_handler();
        let sch = make_schematic();

        // Start pan
        let down = make_event(100.0, 100.0, 10, 10).with_button(MouseButton::Middle);
        handler.on_mouse_down(&down, &sch);

        // End pan
        let up = make_event(120.0, 130.0, 12, 13).with_button(MouseButton::Middle);
        handler.on_mouse_up(&up, &sch);

        assert!(!handler.state.pan.active);
    }

    // =========================================================================
    // SchematicEventHandler - Zoom Tests
    // =========================================================================

    #[test]
    fn test_zoom_wheel() {
        let mut handler = make_handler();
        let evt = make_event(100.0, 100.0, 10, 10).with_wheel(1.0);

        let action = handler.on_wheel(&evt);

        assert!(matches!(action, EventAction::Zoom { center_x, center_y, delta } 
            if center_x == 100.0 && center_y == 100.0 && delta < 0.0));
    }

    #[test]
    fn test_zoom_wheel_reverse() {
        let mut handler = make_handler();
        let evt = make_event(100.0, 100.0, 10, 10).with_wheel(-1.0);

        let action = handler.on_wheel(&evt);

        assert!(matches!(action, EventAction::Zoom { delta, .. } if delta > 0.0));
    }

    // =========================================================================
    // SchematicEventHandler - Select Tool Tests
    // =========================================================================

    #[test]
    fn test_select_empty_starts_box_selection() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Select;

        let evt = make_event(500.0, 500.0, 50, 50).with_button(MouseButton::Left);
        let action = handler.on_mouse_down(&evt, &sch);

        assert!(handler.state.box_selection.active);
        assert!(matches!(action, EventAction::ClearSelection));
    }

    #[test]
    fn test_select_with_shift_preserves_selection() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Select;

        let mods = Modifiers { shift: true, ..Default::default() };
        let evt = make_event(500.0, 500.0, 50, 50)
            .with_button(MouseButton::Left)
            .with_modifiers(mods);
        let action = handler.on_mouse_down(&evt, &sch);

        assert!(matches!(action, EventAction::None));
    }

    // =========================================================================
    // SchematicEventHandler - Box Selection Tests
    // =========================================================================

    #[test]
    fn test_box_selection_move() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Select;

        // Start box selection at empty area (away from wire at y=0)
        let down = make_event(1000.0, 1000.0, 100, 100).with_button(MouseButton::Left);
        handler.on_mouse_down(&down, &sch);

        // Move to expand box
        let move_evt = make_event(1200.0, 1150.0, 120, 115);
        let action = handler.on_mouse_move(&move_evt, &sch);

        assert!(matches!(action, EventAction::SelectBox { .. }));
        assert_eq!(handler.state.box_selection.end_grid, Point::new(120, 115));
    }

    #[test]
    fn test_box_selection_complete() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Select;

        // Start box selection at empty area (y=5 to avoid wire at y=0)
        let down = make_event(0.0, 50.0, 0, 5).with_button(MouseButton::Left);
        handler.on_mouse_down(&down, &sch);

        // Move to encompass component at (10, 10)
        let move_evt = make_event(150.0, 150.0, 15, 15);
        handler.on_mouse_move(&move_evt, &sch);

        // Release
        let up = make_event(150.0, 150.0, 15, 15).with_button(MouseButton::Left);
        let action = handler.on_mouse_up(&up, &sch);

        assert!(!handler.state.box_selection.active);
        if let EventAction::SelectBox { component_ids, .. } = action {
            assert!(component_ids.contains(&1)); // Component at (10, 10)
        } else {
            panic!("Expected SelectBox action");
        }
    }

    // =========================================================================
    // SchematicEventHandler - Wire Tool Tests
    // =========================================================================

    #[test]
    fn test_wire_tool_start() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Wire;

        let evt = make_event(100.0, 100.0, 10, 10).with_button(MouseButton::Left);
        let action = handler.on_mouse_down(&evt, &sch);

        assert!(matches!(action, EventAction::StartWire { pos } if pos == Point::new(10, 10)));
    }

    #[test]
    fn test_wire_tool_add_segment() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Wire;
        sch.wire_drawing.active = true;

        let evt = make_event(200.0, 100.0, 20, 10).with_button(MouseButton::Left);
        let action = handler.on_mouse_down(&evt, &sch);

        assert!(matches!(action, EventAction::AddWireSegment { pos } if pos == Point::new(20, 10)));
    }

    // =========================================================================
    // SchematicEventHandler - Place Tool Tests
    // =========================================================================

    #[test]
    fn test_place_tool_component() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Place(ComponentType::Resistor);
        sch.preview_rotation = Rotation::R90;

        let evt = make_event(100.0, 100.0, 10, 10).with_button(MouseButton::Left);
        let action = handler.on_mouse_down(&evt, &sch);

        assert!(matches!(action, EventAction::PlaceComponent { 
            kind: ComponentType::Resistor, 
            pos,
            rotation: Rotation::R90 
        } if pos == Point::new(10, 10)));
    }

    // =========================================================================
    // SchematicEventHandler - Keyboard Tests
    // =========================================================================

    #[test]
    fn test_escape_cancels_drag() {
        let mut handler = make_handler();
        let sch = make_schematic();
        handler.state.drag.active = true;

        let action = handler.on_key_down("Escape", Modifiers::default(), &sch);

        assert!(!handler.state.drag.active);
        assert!(matches!(action, EventAction::CancelDrag));
    }

    #[test]
    fn test_escape_cancels_wire() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.wire_drawing.active = true;

        let action = handler.on_key_down("Escape", Modifiers::default(), &sch);

        assert!(matches!(action, EventAction::CancelWire));
    }

    #[test]
    fn test_delete_key() {
        let mut handler = make_handler();
        let sch = make_schematic();

        let action = handler.on_key_down("Delete", Modifiers::default(), &sch);

        assert!(matches!(action, EventAction::DeleteSelection));
    }

    #[test]
    fn test_rotate_key() {
        let mut handler = make_handler();
        let sch = make_schematic();

        let action = handler.on_key_down("r", Modifiers::default(), &sch);

        assert!(matches!(action, EventAction::RotatePreview));
    }

    // =========================================================================
    // SchematicEventHandler - Context Menu Tests
    // =========================================================================

    #[test]
    fn test_right_click_context_menu() {
        let mut handler = make_handler();
        let sch = make_schematic();

        let evt = make_event(100.0, 200.0, 10, 20).with_button(MouseButton::Right);
        let action = handler.on_mouse_down(&evt, &sch);

        assert!(matches!(action, EventAction::ShowContextMenu { 
            screen_x, screen_y, world 
        } if screen_x == 100.0 && screen_y == 200.0 && world == Point::new(10, 20)));
    }

    // =========================================================================
    // SchematicEventHandler - Hover Tests
    // =========================================================================

    #[test]
    fn test_hover_junction() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Select;

        // Wire endpoint is at (0, 0)
        let evt = make_event(0.0, 0.0, 0, 0);
        handler.on_mouse_move(&evt, &sch);

        assert_eq!(handler.state.hovered_junction, Some(Point::new(0, 0)));
    }

    #[test]
    fn test_hover_clears_on_move() {
        let mut handler = make_handler();
        let mut sch = make_schematic();
        sch.tool = Tool::Select;

        // Move to junction
        let evt1 = make_event(0.0, 0.0, 0, 0);
        handler.on_mouse_move(&evt1, &sch);
        assert!(handler.state.hovered_junction.is_some());

        // Move away
        let evt2 = make_event(500.0, 500.0, 50, 50);
        handler.on_mouse_move(&evt2, &sch);
        assert!(handler.state.hovered_junction.is_none());
    }
}

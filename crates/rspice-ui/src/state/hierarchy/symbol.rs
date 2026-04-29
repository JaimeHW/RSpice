use serde::{Deserialize, Serialize};

use crate::state::Point;

use super::{InterfacePin, PinDirection, PinType};

/// Symbol graphical content for cell representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolContent {
    /// Symbol bounding box (width, height in grid units)
    pub bounds: (i32, i32),
    /// Symbol pins with positions
    pub pins: Vec<SymbolPin>,
    /// SVG graphic content or drawing primitives
    pub graphics: SymbolGraphics,
}

/// Symbol pin with position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SymbolPin {
    /// Pin name (matches interface pin)
    pub name: String,
    /// Position relative to symbol origin (grid units)
    pub position: Point,
    /// Pin orientation (which way pin sticks out)
    pub orientation: PinOrientation,
}

/// Pin orientation on symbol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum PinOrientation {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

/// Symbol graphics representation
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SymbolGraphics {
    /// SVG content for the symbol body
    pub svg: String,
    /// Drawing primitives (alternative to SVG)
    pub primitives: Vec<DrawingPrimitive>,
}

/// Drawing primitive for symbol graphics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DrawingPrimitive {
    /// Rectangle
    Rect {
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    },
    /// Line
    Line { x1: i32, y1: i32, x2: i32, y2: i32 },
    /// Circle (or ellipse)
    Circle { cx: i32, cy: i32, r: i32 },
    /// Arc
    Arc {
        cx: i32,
        cy: i32,
        r: i32,
        start_angle: f64,
        end_angle: f64,
    },
    /// Polyline
    Polyline { points: Vec<(i32, i32)> },
    /// Text label
    Text {
        x: i32,
        y: i32,
        text: String,
        anchor: TextAnchor,
    },
}

/// Text anchor position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum TextAnchor {
    #[default]
    Start,
    Middle,
    End,
}

impl SymbolContent {
    /// Create a simple rectangular symbol
    pub fn rectangle(width: i32, height: i32, pins: Vec<SymbolPin>) -> Self {
        let graphics = SymbolGraphics {
            svg: String::new(),
            primitives: vec![DrawingPrimitive::Rect {
                x: 0,
                y: 0,
                width,
                height,
            }],
        };

        Self {
            bounds: (width, height),
            pins,
            graphics,
        }
    }

    /// Generate a pin-aware symbol body from a cell interface.
    pub fn generated(name: &str, interface_pins: &[InterfacePin]) -> Self {
        const PIN_PITCH: i32 = 12;
        const EDGE_MARGIN: i32 = 10;

        let mut left: Vec<&InterfacePin> = Vec::new();
        let mut right: Vec<&InterfacePin> = Vec::new();
        let mut top: Vec<&InterfacePin> = Vec::new();
        let mut bottom: Vec<&InterfacePin> = Vec::new();

        for pin in interface_pins {
            match pin.pin_type {
                PinType::Power => top.push(pin),
                PinType::Ground => bottom.push(pin),
                PinType::Clock | PinType::Signal => match pin.direction {
                    PinDirection::Output => right.push(pin),
                    PinDirection::Input | PinDirection::InOut => left.push(pin),
                },
            }
        }

        let vertical_slots = left.len().max(right.len()).max(1) as i32;
        let horizontal_slots = top.len().max(bottom.len()).max(1) as i32;

        let width = (EDGE_MARGIN * 2 + (horizontal_slots - 1) * PIN_PITCH + 24).max(48);
        let height = (EDGE_MARGIN * 2 + (vertical_slots - 1) * PIN_PITCH + 24).max(48);

        let mut pins = Vec::with_capacity(interface_pins.len());
        for (idx, pin) in left.iter().enumerate() {
            let y = EDGE_MARGIN + idx as i32 * PIN_PITCH + 12;
            pins.push(SymbolPin::left(&pin.name, y));
        }
        for (idx, pin) in right.iter().enumerate() {
            let y = EDGE_MARGIN + idx as i32 * PIN_PITCH + 12;
            pins.push(SymbolPin::right(&pin.name, width, y));
        }
        for (idx, pin) in top.iter().enumerate() {
            let x = EDGE_MARGIN + idx as i32 * PIN_PITCH + 12;
            pins.push(SymbolPin::top(&pin.name, x));
        }
        for (idx, pin) in bottom.iter().enumerate() {
            let x = EDGE_MARGIN + idx as i32 * PIN_PITCH + 12;
            pins.push(SymbolPin::bottom(&pin.name, x, height));
        }

        let mut graphics = SymbolGraphics::default();
        graphics.primitives.push(DrawingPrimitive::Rect {
            x: 0,
            y: 0,
            width,
            height,
        });
        graphics.primitives.push(DrawingPrimitive::Text {
            x: width / 2,
            y: height / 2,
            text: name.to_string(),
            anchor: TextAnchor::Middle,
        });

        Self {
            bounds: (width, height),
            pins,
            graphics,
        }
    }
}

impl SymbolPin {
    /// Create a new symbol pin
    pub fn new(name: &str, x: i32, y: i32, orientation: PinOrientation) -> Self {
        Self {
            name: name.to_string(),
            position: Point::new(x, y),
            orientation,
        }
    }

    /// Create left-side pin
    pub fn left(name: &str, y: i32) -> Self {
        Self::new(name, 0, y, PinOrientation::Left)
    }

    /// Create right-side pin
    pub fn right(name: &str, width: i32, y: i32) -> Self {
        Self::new(name, width, y, PinOrientation::Right)
    }

    /// Create top-side pin
    pub fn top(name: &str, x: i32) -> Self {
        Self::new(name, x, 0, PinOrientation::Top)
    }

    /// Create bottom-side pin
    pub fn bottom(name: &str, x: i32, height: i32) -> Self {
        Self::new(name, x, height, PinOrientation::Bottom)
    }
}

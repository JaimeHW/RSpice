//! Component Symbols
//!
//! SVG path definitions for schematic component symbols.
//! All symbols are designed to fit within a 40x40 pixel bounding box,
//! centered at origin (0, 0).

/// Symbol drawing instructions
pub struct Symbol {
    /// SVG path data
    pub path: &'static str,

    /// Viewbox dimensions (width, height)
    pub size: (i32, i32),

    /// Fill color (empty string for stroke-only)
    pub fill: &'static str,

    /// Terminal positions relative to center
    pub terminals: &'static [TerminalDef],
}

/// Terminal definition
pub struct TerminalDef {
    pub name: &'static str,
    pub x: i32,
    pub y: i32,
}

impl Symbol {
    /// Get SVG path data with proper transforms
    pub fn svg_path(&self, x: f64, y: f64, rotation: i32) -> String {
        format!(
            r#"<g transform="translate({},{}) rotate({})"><path d="{}" stroke="currentColor" stroke-width="2" fill="{}"/></g>"#,
            x,
            y,
            rotation,
            self.path,
            if self.fill.is_empty() {
                "none"
            } else {
                self.fill
            }
        )
    }
}

// =============================================================================
// Symbol Definitions
// =============================================================================

/// Resistor symbol (zigzag)
pub const RESISTOR: Symbol = Symbol {
    path: "M -20 0 L -15 0 L -12 -6 L -6 6 L 0 -6 L 6 6 L 12 -6 L 15 0 L 20 0",
    size: (40, 20),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "+",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "-",
            x: 20,
            y: 0,
        },
    ],
};

/// Capacitor symbol (parallel plates)
pub const CAPACITOR: Symbol = Symbol {
    path: "M -20 0 L -4 0 M -4 -8 L -4 8 M 4 -8 L 4 8 M 4 0 L 20 0",
    size: (40, 20),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "+",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "-",
            x: 20,
            y: 0,
        },
    ],
};

/// Inductor symbol (coils)
pub const INDUCTOR: Symbol = Symbol {
    path: "M -20 0 L -15 0 \
           A 4 4 0 0 1 -7 0 \
           A 4 4 0 0 1 1 0 \
           A 4 4 0 0 1 9 0 \
           A 4 4 0 0 1 17 0 \
           L 20 0",
    size: (40, 16),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "+",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "-",
            x: 20,
            y: 0,
        },
    ],
};

/// Diode symbol (triangle with bar)
pub const DIODE: Symbol = Symbol {
    path: "M -20 0 L -6 0 L -6 -8 L 6 0 L -6 8 L -6 0 M 6 -8 L 6 8 M 6 0 L 20 0",
    size: (40, 20),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "A",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "K",
            x: 20,
            y: 0,
        },
    ],
};

/// Ground symbol (3 horizontal lines)
pub const GROUND: Symbol = Symbol {
    path: "M 0 -10 L 0 0 M -10 0 L 10 0 M -6 5 L 6 5 M -2 10 L 2 10",
    size: (20, 20),
    fill: "",
    terminals: &[TerminalDef {
        name: "",
        x: 0,
        y: -10,
    }],
};

/// Voltage source symbol (circle with + and -)
pub const VOLTAGE_SOURCE: Symbol = Symbol {
    path: "M 0 -20 L 0 -10 M 0 10 L 0 20 M 0 0 m -10 0 a 10 10 0 1 0 20 0 a 10 10 0 1 0 -20 0 M -4 -4 L 4 -4 M 0 -8 L 0 0 M -4 4 L 4 4",
    size: (24, 44),
    fill: "",
    terminals: &[
        TerminalDef { name: "+", x: 0, y: -20 },
        TerminalDef { name: "-", x: 0, y: 20 },
    ],
};

/// Current source symbol (circle with arrow)
pub const CURRENT_SOURCE: Symbol = Symbol {
    path: "M 0 -20 L 0 -10 M 0 10 L 0 20 M 0 0 m -10 0 a 10 10 0 1 0 20 0 a 10 10 0 1 0 -20 0 M 0 -6 L 0 6 M -3 3 L 0 6 L 3 3",
    size: (24, 44),
    fill: "",
    terminals: &[
        TerminalDef { name: "+", x: 0, y: -20 },
        TerminalDef { name: "-", x: 0, y: 20 },
    ],
};

/// NPN BJT symbol - professional encircled style
/// Base on left, vertical bar inside circle, Emitter at top-right with arrow out, Collector at bottom-right
pub const NPN_BJT: Symbol = Symbol {
    // Circle: center at (0,0), radius 12
    // Base line from left to circle edge
    // Vertical bar inside circle
    // Collector line from bar to bottom-right lead
    // Emitter line from bar to top-right lead with outward arrow
    path: "M -20 0 L -12 0 \
           M 0 0 m -12 0 a 12 12 0 1 0 24 0 a 12 12 0 1 0 -24 0 \
           M -4 -8 L -4 8 \
           M -4 -4 L 8 -10 L 10 -20 \
           M -4 4 L 8 10 L 10 20 \
           M 4 7 L 8 10 L 5 11",
    size: (40, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "B",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "E",
            x: 10,
            y: -20,
        },
        TerminalDef {
            name: "C",
            x: 10,
            y: 20,
        },
    ],
};

/// PNP BJT symbol - professional encircled style
/// Base on left, vertical bar inside circle, Collector at top-right, Emitter at bottom-right with arrow in
pub const PNP_BJT: Symbol = Symbol {
    // Same structure as NPN but with arrow pointing inward on emitter
    path: "M -20 0 L -12 0 \
           M 0 0 m -12 0 a 12 12 0 1 0 24 0 a 12 12 0 1 0 -24 0 \
           M -4 -8 L -4 8 \
           M -4 -4 L 8 -10 L 10 -20 \
           M -4 4 L 8 10 L 10 20 \
           M -1 6 L -4 4 L -1 2",
    size: (40, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "B",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "C",
            x: 10,
            y: -20,
        },
        TerminalDef {
            name: "E",
            x: 10,
            y: 20,
        },
    ],
};

/// NMOS symbol
pub const NMOS: Symbol = Symbol {
    path: "M -20 0 L -10 0 M -10 -10 L -10 10 M -6 -8 L -6 -2 M -6 -2 L -6 2 M -6 2 L -6 8 \
           M -6 -5 L 10 -5 L 10 -20 M -6 0 L 10 0 M -6 5 L 10 5 L 10 20 \
           M 4 2 L 10 0 L 4 -2",
    size: (40, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "G",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "D",
            x: 10,
            y: -20,
        },
        TerminalDef {
            name: "S",
            x: 10,
            y: 20,
        },
    ],
};

/// PMOS symbol
pub const PMOS: Symbol = Symbol {
    path: "M -20 0 L -14 0 M -10 0 m -3 0 a 3 3 0 1 0 6 0 a 3 3 0 1 0 -6 0 \
           M -10 -10 L -10 10 M -6 -8 L -6 -2 M -6 -2 L -6 2 M -6 2 L -6 8 \
           M -6 -5 L 10 -5 L 10 -20 M -6 0 L 10 0 M -6 5 L 10 5 L 10 20 \
           M 4 -2 L 10 0 L 4 2",
    size: (40, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "G",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "D",
            x: 10,
            y: 20,
        },
        TerminalDef {
            name: "S",
            x: 10,
            y: -20,
        },
    ],
};

/// AC voltage source symbol (circle with sine wave)
pub const VOLTAGE_SOURCE_AC: Symbol = Symbol {
    path: "M 0 -20 L 0 -10 M 0 10 L 0 20 M 0 0 m -10 0 a 10 10 0 1 0 20 0 a 10 10 0 1 0 -20 0 \
           M -5 0 Q -2.5 -5 0 0 Q 2.5 5 5 0",
    size: (24, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "+",
            x: 0,
            y: -20,
        },
        TerminalDef {
            name: "-",
            x: 0,
            y: 20,
        },
    ],
};

/// Pulse voltage source symbol (circle with pulse)
pub const VOLTAGE_SOURCE_PULSE: Symbol = Symbol {
    path: "M 0 -20 L 0 -10 M 0 10 L 0 20 M 0 0 m -10 0 a 10 10 0 1 0 20 0 a 10 10 0 1 0 -20 0 \
           M -5 3 L -3 3 L -3 -3 L 3 -3 L 3 3 L 5 3",
    size: (24, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "+",
            x: 0,
            y: -20,
        },
        TerminalDef {
            name: "-",
            x: 0,
            y: 20,
        },
    ],
};

/// Sinusoidal voltage source (same as AC for display)
pub const VOLTAGE_SOURCE_SIN: Symbol = VOLTAGE_SOURCE_AC;

/// N-JFET symbol (similar to MOSFET but with solid gate)
pub const NJFET: Symbol = Symbol {
    path: "M -20 0 L -6 0 M -6 -10 L -6 10 \
           M -6 -5 L 10 -5 L 10 -20 M -6 5 L 10 5 L 10 20 \
           M 4 2 L 10 0 L 4 -2",
    size: (40, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "G",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "D",
            x: 10,
            y: -20,
        },
        TerminalDef {
            name: "S",
            x: 10,
            y: 20,
        },
    ],
};

/// P-JFET symbol
pub const PJFET: Symbol = Symbol {
    path: "M -20 0 L -6 0 M -6 -10 L -6 10 \
           M -6 -5 L 10 -5 L 10 -20 M -6 5 L 10 5 L 10 20 \
           M -2 2 L -6 0 L -2 -2",
    size: (40, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "G",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "D",
            x: 10,
            y: 20,
        },
        TerminalDef {
            name: "S",
            x: 10,
            y: -20,
        },
    ],
};

/// Coupled inductor symbol (two inductors with coupling line)
pub const COUPLED_INDUCTOR: Symbol = Symbol {
    path: "M -15 -10 A 4 4 0 0 1 -7 -10 A 4 4 0 0 1 1 -10 A 4 4 0 0 1 9 -10 L 15 -10 \
           M -15 10 A 4 4 0 0 1 -7 10 A 4 4 0 0 1 1 10 A 4 4 0 0 1 9 10 L 15 10 \
           M 0 -6 L 0 6",
    size: (40, 24),
    fill: "",
    terminals: &[],
};

/// VCVS symbol (diamond with E)
pub const VCVS: Symbol = Symbol {
    path: "M 0 -15 L 12 0 L 0 15 L -12 0 Z M -20 -10 L -12 -5 M -20 10 L -12 5 M 12 -5 L 20 -10 M 12 5 L 20 10",
    size: (44, 34),
    fill: "",
    terminals: &[
        TerminalDef { name: "O+", x: 20, y: -10 },
        TerminalDef { name: "O-", x: 20, y: 10 },
        TerminalDef { name: "C+", x: -20, y: -10 },
        TerminalDef { name: "C-", x: -20, y: 10 },
    ],
};

/// VCCS symbol (diamond with arrow)
pub const VCCS: Symbol = Symbol {
    path: "M 0 -15 L 12 0 L 0 15 L -12 0 Z M -20 -10 L -12 -5 M -20 10 L -12 5 M 12 -5 L 20 -10 M 12 5 L 20 10 \
           M 0 -5 L 0 5 M -2 3 L 0 5 L 2 3",
    size: (44, 34),
    fill: "",
    terminals: &[
        TerminalDef { name: "O+", x: 20, y: -10 },
        TerminalDef { name: "O-", x: 20, y: 10 },
        TerminalDef { name: "C+", x: -20, y: -10 },
        TerminalDef { name: "C-", x: -20, y: 10 },
    ],
};

/// CCVS symbol (diamond with H)
pub const CCVS: Symbol = VCVS;

/// CCCS symbol (diamond with F)
pub const CCCS: Symbol = VCCS;

// =============================================================================
// Symbol Lookup
// =============================================================================

use crate::state::ComponentType;

/// Get the symbol for a component type
pub fn get_symbol(kind: ComponentType) -> &'static Symbol {
    match kind {
        ComponentType::Resistor => &RESISTOR,
        ComponentType::Capacitor => &CAPACITOR,
        ComponentType::Inductor => &INDUCTOR,
        ComponentType::CoupledInductor => &COUPLED_INDUCTOR,
        ComponentType::Diode => &DIODE,
        ComponentType::Ground => &GROUND,
        ComponentType::VoltageSource => &VOLTAGE_SOURCE,
        ComponentType::CurrentSource => &CURRENT_SOURCE,
        ComponentType::NpnBjt => &NPN_BJT,
        ComponentType::PnpBjt => &PNP_BJT,
        ComponentType::Nmos => &NMOS,
        ComponentType::Pmos => &PMOS,
        ComponentType::Njfet => &NJFET,
        ComponentType::Pjfet => &PJFET,
        ComponentType::VoltageSourceAc => &VOLTAGE_SOURCE_AC,
        ComponentType::VoltageSourcePulse => &VOLTAGE_SOURCE_PULSE,
        ComponentType::VoltageSourceSin => &VOLTAGE_SOURCE_SIN,
        ComponentType::Vcvs => &VCVS,
        ComponentType::Vccs => &VCCS,
        ComponentType::Ccvs => &CCVS,
        ComponentType::Cccs => &CCCS,
    }
}

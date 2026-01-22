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

/// NPN BJT symbol - encircled style
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

/// PNP BJT symbol - encircled style
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
// XSPICE Symbol Definitions
// =============================================================================

/// XSPICE Gain Block (rectangle with ×k label)
pub const XSPICE_GAIN: Symbol = Symbol {
    path: "M -15 -12 L 15 -12 L 15 12 L -15 12 Z M -20 0 L -15 0 M 15 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Summer (triangle with Σ)
pub const XSPICE_SUMMER: Symbol = Symbol {
    path: "M -10 -15 L 15 0 L -10 15 Z M -20 -10 L -10 -5 M -20 10 L -10 5 M 15 0 L 20 0",
    size: (40, 34),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in1",
            x: -20,
            y: -10,
        },
        TerminalDef {
            name: "in2",
            x: -20,
            y: 10,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Multiplier (rectangle with × symbol)
pub const XSPICE_MULTIPLIER: Symbol = Symbol {
    path:
        "M -12 -12 L 12 -12 L 12 12 L -12 12 Z M -20 -10 L -12 -5 M -20 10 L -12 5 M 12 0 L 20 0 \
           M -4 -4 L 4 4 M -4 4 L 4 -4",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in1",
            x: -20,
            y: -10,
        },
        TerminalDef {
            name: "in2",
            x: -20,
            y: 10,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Divider (rectangle with ÷ symbol)
pub const XSPICE_DIVIDER: Symbol = Symbol {
    path:
        "M -12 -12 L 12 -12 L 12 12 L -12 12 Z M -20 -10 L -12 -5 M -20 10 L -12 5 M 12 0 L 20 0 \
           M -5 0 L 5 0 M 0 -4 m -1.5 0 a 1.5 1.5 0 1 0 3 0 a 1.5 1.5 0 1 0 -3 0 \
           M 0 4 m -1.5 0 a 1.5 1.5 0 1 0 3 0 a 1.5 1.5 0 1 0 -3 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in1",
            x: -20,
            y: -10,
        },
        TerminalDef {
            name: "in2",
            x: -20,
            y: 10,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Limiter (rectangle with clipped sine)
pub const XSPICE_LIMITER: Symbol = Symbol {
    path: "M -15 -12 L 15 -12 L 15 12 L -15 12 Z M -20 0 L -15 0 M 15 0 L 20 0 \
           M -8 4 L -4 4 Q 0 -8 4 4 L 8 4",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Integrator (rectangle with ∫ symbol)
pub const XSPICE_INTEGRATOR: Symbol = Symbol {
    path: "M -15 -12 L 15 -12 L 15 12 L -15 12 Z M -20 0 L -15 0 M 15 0 L 20 0 \
           M 2 -8 C -4 -8 -4 0 0 0 C 4 0 4 8 -2 8",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Differentiator (rectangle with d/dt)
pub const XSPICE_DIFFERENTIATOR: Symbol = Symbol {
    path: "M -15 -12 L 15 -12 L 15 12 L -15 12 Z M -20 0 L -15 0 M 15 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Inverter (NOT gate - triangle with bubble)
pub const XSPICE_INVERTER: Symbol = Symbol {
    path: "M -10 -12 L 10 0 L -10 12 Z M 12 0 m -3 0 a 3 3 0 1 0 6 0 a 3 3 0 1 0 -6 0 \
           M -20 0 L -10 0 M 15 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Buffer (triangle)
pub const XSPICE_BUFFER: Symbol = Symbol {
    path: "M -10 -12 L 10 0 L -10 12 Z M -20 0 L -10 0 M 10 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE AND Gate (flat back, curved front)
pub const XSPICE_AND_GATE: Symbol = Symbol {
    path: "M -10 -12 L -10 12 L 2 12 A 12 12 0 0 0 2 -12 Z \
           M -20 -8 L -10 -8 M -20 8 L -10 8 M 14 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "a",
            x: -20,
            y: -8,
        },
        TerminalDef {
            name: "b",
            x: -20,
            y: 8,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE OR Gate (curved back and front)
pub const XSPICE_OR_GATE: Symbol = Symbol {
    path: "M -12 -12 Q -6 0 -12 12 Q 0 10 6 12 Q 14 0 6 -12 Q 0 -10 -12 -12 \
           M -20 -8 L -8 -8 M -20 8 L -8 8 M 14 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "a",
            x: -20,
            y: -8,
        },
        TerminalDef {
            name: "b",
            x: -20,
            y: 8,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE NAND Gate (AND with bubble)
pub const XSPICE_NAND_GATE: Symbol = Symbol {
    path: "M -10 -12 L -10 12 L 2 12 A 12 12 0 0 0 2 -12 Z \
           M 14 0 m -3 0 a 3 3 0 1 0 6 0 a 3 3 0 1 0 -6 0 \
           M -20 -8 L -10 -8 M -20 8 L -10 8 M 17 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "a",
            x: -20,
            y: -8,
        },
        TerminalDef {
            name: "b",
            x: -20,
            y: 8,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE NOR Gate (OR with bubble)
pub const XSPICE_NOR_GATE: Symbol = Symbol {
    path: "M -12 -12 Q -6 0 -12 12 Q 0 10 6 12 Q 14 0 6 -12 Q 0 -10 -12 -12 \
           M 14 0 m -3 0 a 3 3 0 1 0 6 0 a 3 3 0 1 0 -6 0 \
           M -20 -8 L -8 -8 M -20 8 L -8 8 M 17 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "a",
            x: -20,
            y: -8,
        },
        TerminalDef {
            name: "b",
            x: -20,
            y: 8,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE XOR Gate (OR with extra curve at input)
pub const XSPICE_XOR_GATE: Symbol = Symbol {
    path: "M -12 -12 Q -6 0 -12 12 Q 0 10 6 12 Q 14 0 6 -12 Q 0 -10 -12 -12 \
           M -15 -12 Q -9 0 -15 12 \
           M -20 -8 L -10 -8 M -20 8 L -10 8 M 14 0 L 20 0",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "a",
            x: -20,
            y: -8,
        },
        TerminalDef {
            name: "b",
            x: -20,
            y: 8,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE Tri-State Buffer (triangle with enable)
pub const XSPICE_TRISTATE: Symbol = Symbol {
    path: "M -10 -12 L 10 0 L -10 12 Z M -20 0 L -10 0 M 10 0 L 20 0 \
           M 0 -20 L 0 -6",
    size: (40, 48),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "en",
            x: 0,
            y: -20,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE D Flip-Flop (rectangle with D, CLK, Q)
pub const XSPICE_D_FLIPFLOP: Symbol = Symbol {
    path: "M -15 -18 L 15 -18 L 15 18 L -15 18 Z \
           M -20 -10 L -15 -10 M -20 10 L -15 10 M 15 -10 L 20 -10 M 15 10 L 20 10 \
           M -15 7 L -12 10 L -15 13",
    size: (40, 44),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "d",
            x: -20,
            y: -10,
        },
        TerminalDef {
            name: "clk",
            x: -20,
            y: 10,
        },
        TerminalDef {
            name: "q",
            x: 20,
            y: -10,
        },
        TerminalDef {
            name: "qbar",
            x: 20,
            y: 10,
        },
    ],
};

/// XSPICE JK Flip-Flop (rectangle with J, K, CLK, Q)
pub const XSPICE_JK_FLIPFLOP: Symbol = Symbol {
    path: "M -15 -20 L 15 -20 L 15 20 L -15 20 Z \
           M -20 -12 L -15 -12 M -20 0 L -15 0 M -20 12 L -15 12 \
           M 15 -10 L 20 -10 M 15 10 L 20 10 \
           M -15 -3 L -12 0 L -15 3",
    size: (40, 48),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "j",
            x: -20,
            y: -12,
        },
        TerminalDef {
            name: "clk",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "k",
            x: -20,
            y: 12,
        },
        TerminalDef {
            name: "q",
            x: 20,
            y: -10,
        },
        TerminalDef {
            name: "qbar",
            x: 20,
            y: 10,
        },
    ],
};

/// XSPICE SR Latch (rectangle with S, R, Q)
pub const XSPICE_SR_LATCH: Symbol = Symbol {
    path: "M -15 -15 L 15 -15 L 15 15 L -15 15 Z \
           M -20 -8 L -15 -8 M -20 8 L -15 8 M 15 -8 L 20 -8 M 15 8 L 20 8",
    size: (40, 36),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "s",
            x: -20,
            y: -8,
        },
        TerminalDef {
            name: "r",
            x: -20,
            y: 8,
        },
        TerminalDef {
            name: "q",
            x: 20,
            y: -8,
        },
        TerminalDef {
            name: "qbar",
            x: 20,
            y: 8,
        },
    ],
};

/// XSPICE ADC Bridge (rectangle with A→D label)
pub const XSPICE_ADC_BRIDGE: Symbol = Symbol {
    path: "M -15 -12 L 15 -12 L 15 12 L -15 12 Z M -20 0 L -15 0 M 15 0 L 20 0 \
           M -6 0 L 6 0 M 3 -3 L 6 0 L 3 3",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

/// XSPICE DAC Bridge (rectangle with D→A label)
pub const XSPICE_DAC_BRIDGE: Symbol = Symbol {
    path: "M -15 -12 L 15 -12 L 15 12 L -15 12 Z M -20 0 L -15 0 M 15 0 L 20 0 \
           M -6 0 L 6 0 M 3 -3 L 6 0 L 3 3",
    size: (40, 28),
    fill: "",
    terminals: &[
        TerminalDef {
            name: "in",
            x: -20,
            y: 0,
        },
        TerminalDef {
            name: "out",
            x: 20,
            y: 0,
        },
    ],
};

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
        // XSPICE Analog Behavioral
        ComponentType::XspiceGain => &XSPICE_GAIN,
        ComponentType::XspiceSummer => &XSPICE_SUMMER,
        ComponentType::XspiceMultiplier => &XSPICE_MULTIPLIER,
        ComponentType::XspiceDivider => &XSPICE_DIVIDER,
        ComponentType::XspiceLimiter => &XSPICE_LIMITER,
        ComponentType::XspiceIntegrator => &XSPICE_INTEGRATOR,
        ComponentType::XspiceDifferentiator => &XSPICE_DIFFERENTIATOR,
        // XSPICE Digital Gates
        ComponentType::XspiceInverter => &XSPICE_INVERTER,
        ComponentType::XspiceBuffer => &XSPICE_BUFFER,
        ComponentType::XspiceAndGate => &XSPICE_AND_GATE,
        ComponentType::XspiceOrGate => &XSPICE_OR_GATE,
        ComponentType::XspiceNandGate => &XSPICE_NAND_GATE,
        ComponentType::XspiceNorGate => &XSPICE_NOR_GATE,
        ComponentType::XspiceXorGate => &XSPICE_XOR_GATE,
        ComponentType::XspiceTristate => &XSPICE_TRISTATE,
        // XSPICE Sequential
        ComponentType::XspiceDFlipFlop => &XSPICE_D_FLIPFLOP,
        ComponentType::XspiceJkFlipFlop => &XSPICE_JK_FLIPFLOP,
        ComponentType::XspiceSrLatch => &XSPICE_SR_LATCH,
        // XSPICE Bridges
        ComponentType::XspiceAdcBridge => &XSPICE_ADC_BRIDGE,
        ComponentType::XspiceDacBridge => &XSPICE_DAC_BRIDGE,
    }
}

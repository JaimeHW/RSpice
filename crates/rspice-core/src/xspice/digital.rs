//! Digital Signal Types
//!
//! Implements the 12-state digital value system used by XSPICE.
//! Compatible with ngspice's digital event-driven simulation.

use std::fmt;

//=============================================================================
// Digital State
//=============================================================================

/// 12-state digital logic state
///
/// This enumeration provides compatibility with ngspice's XSPICE digital system.
/// States are organized by logic level and strength.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum DigitalState {
    /// Strong logic zero
    Zero,
    /// Strong logic one
    One,
    /// Strong unknown (conflict or uninitialized)
    #[default]
    Unknown,
    /// Resistive logic zero (weak pull-down)
    ZeroR,
    /// Resistive logic one (weak pull-up)
    OneR,
    /// Resistive unknown
    UnknownR,
    /// High-Z tending to zero
    ZeroZ,
    /// High-Z tending to one
    OneZ,
    /// High-Z unknown
    UnknownZ,
    /// High impedance (disconnected)
    HighZ,
}

impl DigitalState {
    /// Check if this state is a logic high (1 or 1-ish)
    pub fn is_high(&self) -> bool {
        matches!(
            self,
            DigitalState::One | DigitalState::OneR | DigitalState::OneZ
        )
    }

    /// Check if this state is a logic low (0 or 0-ish)
    pub fn is_low(&self) -> bool {
        matches!(
            self,
            DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ
        )
    }

    /// Check if this state is unknown/undefined
    pub fn is_unknown(&self) -> bool {
        matches!(
            self,
            DigitalState::Unknown | DigitalState::UnknownR | DigitalState::UnknownZ
        )
    }

    /// Check if this state is high impedance
    pub fn is_high_z(&self) -> bool {
        matches!(
            self,
            DigitalState::HighZ | DigitalState::ZeroZ | DigitalState::OneZ | DigitalState::UnknownZ
        )
    }

    /// Get the logic level (0, 1, or X for unknown)
    pub fn logic_level(&self) -> Option<bool> {
        match self {
            DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => Some(false),
            DigitalState::One | DigitalState::OneR | DigitalState::OneZ => Some(true),
            _ => None,
        }
    }

    /// Invert the logic level
    pub fn invert(&self) -> DigitalState {
        match self {
            DigitalState::Zero => DigitalState::One,
            DigitalState::One => DigitalState::Zero,
            DigitalState::ZeroR => DigitalState::OneR,
            DigitalState::OneR => DigitalState::ZeroR,
            DigitalState::ZeroZ => DigitalState::OneZ,
            DigitalState::OneZ => DigitalState::ZeroZ,
            _ => DigitalState::Unknown,
        }
    }

    /// AND operation between two states
    pub fn and(&self, other: &DigitalState) -> DigitalState {
        match (self.logic_level(), other.logic_level()) {
            (Some(false), _) | (_, Some(false)) => DigitalState::Zero,
            (Some(true), Some(true)) => DigitalState::One,
            _ => DigitalState::Unknown,
        }
    }

    /// OR operation between two states
    pub fn or(&self, other: &DigitalState) -> DigitalState {
        match (self.logic_level(), other.logic_level()) {
            (Some(true), _) | (_, Some(true)) => DigitalState::One,
            (Some(false), Some(false)) => DigitalState::Zero,
            _ => DigitalState::Unknown,
        }
    }

    /// XOR operation between two states
    pub fn xor(&self, other: &DigitalState) -> DigitalState {
        match (self.logic_level(), other.logic_level()) {
            (Some(a), Some(b)) => {
                if a ^ b {
                    DigitalState::One
                } else {
                    DigitalState::Zero
                }
            }
            _ => DigitalState::Unknown,
        }
    }

    /// Convert to analog voltage
    pub fn to_voltage(&self, v_low: f64, v_high: f64, v_unknown: f64) -> f64 {
        match self {
            DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => v_low,
            DigitalState::One | DigitalState::OneR | DigitalState::OneZ => v_high,
            _ => v_unknown,
        }
    }

    /// Create from analog voltage using thresholds
    pub fn from_voltage(v: f64, v_low: f64, v_high: f64) -> DigitalState {
        if v <= v_low {
            DigitalState::Zero
        } else if v >= v_high {
            DigitalState::One
        } else {
            DigitalState::Unknown
        }
    }

    /// Parse from character representation
    pub fn from_char(c: char) -> Option<DigitalState> {
        match c {
            '0' | 'l' | 'L' => Some(DigitalState::Zero),
            '1' | 'h' | 'H' => Some(DigitalState::One),
            'x' | 'X' | 'u' | 'U' => Some(DigitalState::Unknown),
            'z' | 'Z' => Some(DigitalState::HighZ),
            _ => None,
        }
    }

    /// Convert to character representation
    pub fn to_char(&self) -> char {
        match self {
            DigitalState::Zero => '0',
            DigitalState::One => '1',
            DigitalState::Unknown => 'X',
            DigitalState::ZeroR => 'l',
            DigitalState::OneR => 'h',
            DigitalState::UnknownR => 'x',
            DigitalState::ZeroZ => 'L',
            DigitalState::OneZ => 'H',
            DigitalState::UnknownZ => 'u',
            DigitalState::HighZ => 'Z',
        }
    }
}

impl fmt::Display for DigitalState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DigitalState::Zero => write!(f, "0s"),
            DigitalState::One => write!(f, "1s"),
            DigitalState::Unknown => write!(f, "Xs"),
            DigitalState::ZeroR => write!(f, "0r"),
            DigitalState::OneR => write!(f, "1r"),
            DigitalState::UnknownR => write!(f, "Xr"),
            DigitalState::ZeroZ => write!(f, "0z"),
            DigitalState::OneZ => write!(f, "1z"),
            DigitalState::UnknownZ => write!(f, "Xz"),
            DigitalState::HighZ => write!(f, "Z"),
        }
    }
}

//=============================================================================
// Digital Strength
//=============================================================================

/// Drive strength for digital signals
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DigitalStrength {
    /// No drive (not driving)
    Undetermined,
    /// High impedance
    HighZ,
    /// Resistive (weak) drive
    Resistive,
    /// Strong drive (default)
    #[default]
    Strong,
}

impl DigitalStrength {
    /// Get the stronger of two strengths
    pub fn max(self, other: DigitalStrength) -> DigitalStrength {
        std::cmp::max(self, other)
    }
}

//=============================================================================
// Digital Value
//=============================================================================

/// Complete digital value with state and strength
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DigitalValue {
    /// Logic state
    pub state: DigitalState,
    /// Drive strength
    pub strength: DigitalStrength,
}

impl Default for DigitalValue {
    fn default() -> Self {
        // ngspice's `digital` user-defined-node initializer sets event nodes
        // to ZERO with UNDETERMINED strength before any code model drives them.
        Self::new(DigitalState::Zero, DigitalStrength::Undetermined)
    }
}

impl DigitalValue {
    /// Create a new digital value
    pub fn new(state: DigitalState, strength: DigitalStrength) -> Self {
        Self { state, strength }
    }

    /// Create a strong zero
    pub fn zero() -> Self {
        Self::new(DigitalState::Zero, DigitalStrength::Strong)
    }

    /// Create a strong one
    pub fn one() -> Self {
        Self::new(DigitalState::One, DigitalStrength::Strong)
    }

    /// Create unknown
    pub fn unknown() -> Self {
        Self::new(DigitalState::Unknown, DigitalStrength::Strong)
    }

    /// Create high-Z
    pub fn high_z() -> Self {
        Self::new(DigitalState::HighZ, DigitalStrength::HighZ)
    }

    /// Create from boolean
    pub fn from_bool(b: bool) -> Self {
        if b { Self::one() } else { Self::zero() }
    }

    /// Convert to boolean (None if unknown/high-z)
    pub fn to_bool(&self) -> Option<bool> {
        self.state.logic_level()
    }

    /// Invert the value
    pub fn invert(&self) -> Self {
        Self::new(self.state.invert(), self.strength)
    }

    /// Resolve bus contention between two values
    ///
    /// When multiple drivers are connected, the stronger one wins.
    /// If strengths are equal, unknown results.
    pub fn resolve(&self, other: &DigitalValue) -> DigitalValue {
        match self.strength.cmp(&other.strength) {
            std::cmp::Ordering::Greater => *self,
            std::cmp::Ordering::Less => *other,
            std::cmp::Ordering::Equal => {
                // Same strength - check for contention
                if self.state == other.state {
                    *self
                } else {
                    // Contention - unknown
                    DigitalValue::new(DigitalState::Unknown, self.strength)
                }
            }
        }
    }

    /// Format using ngspice's XSPICE digital `eprint` token spelling.
    pub fn to_ngspice_token(&self) -> String {
        let state = match self.state {
            DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => '0',
            DigitalState::One | DigitalState::OneR | DigitalState::OneZ => '1',
            DigitalState::Unknown | DigitalState::UnknownR | DigitalState::UnknownZ => 'U',
            DigitalState::HighZ => 'U',
        };
        let strength = match self.strength {
            DigitalStrength::Strong => 's',
            DigitalStrength::Resistive => 'r',
            DigitalStrength::HighZ => 'z',
            DigitalStrength::Undetermined => 'u',
        };
        format!("{state}{strength}")
    }
}

impl fmt::Display for DigitalValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.state)
    }
}

impl From<bool> for DigitalValue {
    fn from(b: bool) -> Self {
        Self::from_bool(b)
    }
}

//=============================================================================
// Tests
//=============================================================================

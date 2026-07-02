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

/// ngspice's XSPICE digital user-defined-node resolver table.
///
/// Table indexes are strength-major: strong, resistive, high-Z, undetermined;
/// each strength group stores zero, one, unknown.
const NGSPICE_DIGITAL_RESOLVE_MAP: [[usize; 12]; 12] = [
    [0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 2, 2],
    [2, 1, 2, 1, 1, 1, 1, 1, 1, 2, 1, 2],
    [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
    [0, 1, 2, 3, 5, 5, 3, 3, 3, 9, 11, 11],
    [0, 1, 2, 5, 4, 5, 4, 4, 4, 11, 10, 11],
    [0, 1, 2, 5, 5, 5, 5, 5, 5, 11, 11, 11],
    [0, 1, 2, 3, 4, 5, 6, 8, 8, 9, 11, 11],
    [0, 1, 2, 3, 4, 5, 8, 7, 8, 11, 10, 11],
    [0, 1, 2, 3, 4, 5, 8, 8, 8, 11, 11, 11],
    [0, 2, 2, 9, 11, 11, 9, 11, 11, 9, 11, 11],
    [2, 1, 2, 11, 10, 11, 11, 10, 11, 11, 10, 11],
    [2, 2, 2, 11, 11, 11, 11, 11, 11, 11, 11, 11],
];

fn ngspice_state_index(state: DigitalState) -> usize {
    match state {
        DigitalState::Zero | DigitalState::ZeroR | DigitalState::ZeroZ => 0,
        DigitalState::One | DigitalState::OneR | DigitalState::OneZ => 1,
        DigitalState::Unknown
        | DigitalState::UnknownR
        | DigitalState::UnknownZ
        | DigitalState::HighZ => 2,
    }
}

fn ngspice_strength_index(strength: DigitalStrength) -> usize {
    match strength {
        DigitalStrength::Strong => 0,
        DigitalStrength::Resistive => 1,
        DigitalStrength::HighZ => 2,
        DigitalStrength::Undetermined => 3,
    }
}

fn ngspice_resolve_index(value: DigitalValue) -> usize {
    ngspice_strength_index(value.strength) * 3 + ngspice_state_index(value.state)
}

fn digital_value_from_ngspice_resolve_index(index: usize) -> DigitalValue {
    match index {
        0 => DigitalValue::new(DigitalState::Zero, DigitalStrength::Strong),
        1 => DigitalValue::new(DigitalState::One, DigitalStrength::Strong),
        2 => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Strong),
        3 => DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive),
        4 => DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive),
        5 => DigitalValue::new(DigitalState::UnknownR, DigitalStrength::Resistive),
        6 => DigitalValue::new(DigitalState::ZeroZ, DigitalStrength::HighZ),
        7 => DigitalValue::new(DigitalState::OneZ, DigitalStrength::HighZ),
        8 => DigitalValue::new(DigitalState::UnknownZ, DigitalStrength::HighZ),
        9 => DigitalValue::new(DigitalState::Zero, DigitalStrength::Undetermined),
        10 => DigitalValue::new(DigitalState::One, DigitalStrength::Undetermined),
        11 => DigitalValue::new(DigitalState::Unknown, DigitalStrength::Undetermined),
        _ => unreachable!("ngspice digital resolver index is out of range"),
    }
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
    /// ngspice uses a fixed 12-state user-defined-node resolver table rather
    /// than simple strength ordering.
    pub fn resolve(&self, other: &DigitalValue) -> DigitalValue {
        let lhs = ngspice_resolve_index(*self);
        let rhs = ngspice_resolve_index(*other);
        digital_value_from_ngspice_resolve_index(NGSPICE_DIGITAL_RESOLVE_MAP[lhs][rhs])
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digital_resolution_matches_ngspice_undetermined_contention() {
        let strong_zero = DigitalValue::zero();
        let undetermined_one = DigitalValue::new(DigitalState::One, DigitalStrength::Undetermined);

        assert_eq!(
            strong_zero.resolve(&undetermined_one),
            DigitalValue::unknown()
        );
        assert_eq!(
            undetermined_one.resolve(&strong_zero),
            DigitalValue::unknown()
        );
    }

    #[test]
    fn digital_resolution_preserves_ngspice_resolved_strength_variants() {
        let zero_r = DigitalValue::new(DigitalState::ZeroR, DigitalStrength::Resistive);
        let one_r = DigitalValue::new(DigitalState::OneR, DigitalStrength::Resistive);
        assert_eq!(
            zero_r.resolve(&one_r),
            DigitalValue::new(DigitalState::UnknownR, DigitalStrength::Resistive)
        );

        let zero_z = DigitalValue::new(DigitalState::ZeroZ, DigitalStrength::HighZ);
        let one_z = DigitalValue::new(DigitalState::OneZ, DigitalStrength::HighZ);
        assert_eq!(
            zero_z.resolve(&one_z),
            DigitalValue::new(DigitalState::UnknownZ, DigitalStrength::HighZ)
        );
    }

    #[test]
    fn digital_resolution_normalizes_rspice_high_z_state_to_ngspice_unknown_z() {
        assert_eq!(
            DigitalValue::high_z().resolve(&DigitalValue::high_z()),
            DigitalValue::new(DigitalState::UnknownZ, DigitalStrength::HighZ)
        );
        assert_eq!(
            DigitalValue::high_z().resolve(&DigitalValue::one()),
            DigitalValue::one()
        );
    }
}

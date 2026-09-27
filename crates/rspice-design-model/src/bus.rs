//! Shared bus declarations, selectors and bounded name parsing.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

/// Highest supported bus member index: a 4,096-member budget, `0..=4095`.
///
/// The budget prevents hostile or corrupt project files from requesting
/// unbounded member expansion. It is a budget rather than a machine limit
/// because every member a declaration expands to becomes a conductor the
/// drawing, the connectivity model and the deck each carry one of, and 4,096
/// of them is already well past the widest bus a schematic is drawn with — a
/// vector wider than that is a generated structure, not a drawing.
pub const MAX_BUS_MEMBER_INDEX: u32 = 4_095;

/// Delimiter style used by a typed bus name.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusNotation {
    /// Verilog/SPICE-style `DATA[7:0]` notation.
    #[default]
    Square,
    /// Alternate EDA-style `DATA<7:0>` notation.
    Angle,
}

impl BusNotation {
    /// The pair a member is written between. Every rendering of an authored
    /// bit — a member's own `Display`, a range header, the deck's inverse —
    /// asks here rather than spelling a bracket of its own.
    pub const fn delimiters(self) -> (char, char) {
        match self {
            Self::Square => ('[', ']'),
            Self::Angle => ('<', '>'),
        }
    }
}

/// Ordering of members in a range declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusDirection {
    Ascending,
    Descending,
}

/// The kind of electrical object a bus-tap selection must connect to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusTargetKind {
    /// A scalar member connects to an ordinary wire/net.
    Wire,
    /// A multi-member slice connects to another bus.
    Bus,
}

/// One expanded member of a bus declaration or slice.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BusMember {
    pub name: String,
    pub index: u32,
    pub notation: BusNotation,
}

impl fmt::Display for BusMember {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (open, close) = self.notation.delimiters();
        write!(formatter, "{}{open}{}{close}", self.name, self.index)
    }
}

/// A validated typed bus declaration such as `DATA[15:0]`.
///
/// Declarations always contain at least two members. Use [`BusSlice`] for a
/// scalar member or a narrower range selected by a tap.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BusDeclaration {
    pub name: String,
    pub msb: u32,
    pub lsb: u32,
    pub notation: BusNotation,
}

impl BusDeclaration {
    pub fn new(
        name: impl Into<String>,
        msb: u32,
        lsb: u32,
        notation: BusNotation,
    ) -> Result<Self, BusParseError> {
        let declaration = Self {
            name: name.into(),
            msb,
            lsb,
            notation,
        };
        declaration.validate()?;
        Ok(declaration)
    }

    /// Parse square- or angle-bracket range notation.
    pub fn parse(input: &str) -> Result<Self, BusParseError> {
        let parsed = ParsedBusName::parse(input, false)?;
        Self::new(parsed.name, parsed.msb, parsed.lsb, parsed.notation)
    }

    /// Validate a deserialized or programmatically assembled declaration.
    pub fn validate(&self) -> Result<(), BusParseError> {
        validate_identifier(&self.name)?;
        validate_index(self.msb)?;
        validate_index(self.lsb)?;
        if self.msb == self.lsb {
            return Err(BusParseError::DeclarationWidthTooSmall);
        }
        Ok(())
    }

    pub fn width(&self) -> usize {
        self.msb.abs_diff(self.lsb) as usize + 1
    }

    pub fn direction(&self) -> BusDirection {
        if self.msb < self.lsb {
            BusDirection::Ascending
        } else {
            BusDirection::Descending
        }
    }

    /// Expand members in declaration order, preserving direction and style.
    pub fn members(&self) -> Vec<BusMember> {
        expand_members(&self.name, self.msb, self.lsb, self.notation)
    }

    pub fn contains_index(&self, index: u32) -> bool {
        let low = self.msb.min(self.lsb);
        let high = self.msb.max(self.lsb);
        (low..=high).contains(&index)
    }

    /// Position of `index` in declaration order.
    ///
    /// Declaration order is what a port list, an instance node list and a
    /// projected bit set all agree on, so a bit's position — not its numeric
    /// index — is what maps one vector onto another of the same width.
    pub fn bit_position(&self, index: u32) -> Option<usize> {
        self.contains_index(index).then(|| {
            if self.msb <= self.lsb {
                (index - self.msb) as usize
            } else {
                (self.msb - index) as usize
            }
        })
    }

    /// Validate one scalar or slice against this bus's declared type.
    pub fn validate_slice(&self, slice: &BusSlice) -> Result<(), BusParseError> {
        self.validate()?;
        slice.validate()?;
        if self.name != slice.name {
            return Err(BusParseError::MixedBase {
                expected: self.name.clone(),
                found: slice.name.clone(),
            });
        }
        if self.notation != slice.notation {
            return Err(BusParseError::MixedNotation);
        }
        if !self.contains_index(slice.msb) || !self.contains_index(slice.lsb) {
            return Err(BusParseError::SelectorOutOfRange);
        }
        // A vector slice may intentionally enumerate members in the opposite
        // direction to express reversible bit-order mapping into a matching
        // destination bus. Membership remains exact here; destination width,
        // range, and ambiguity are validated by the tap transaction.
        Ok(())
    }

    /// Validate a group of selectors and prove that each declared member is
    /// owned by at most one selector.
    pub fn validate_slices(&self, slices: &[BusSlice]) -> Result<(), BusParseError> {
        let mut ranges = Vec::with_capacity(slices.len());
        for slice in slices {
            self.validate_slice(slice)?;
            ranges.push((slice.msb.min(slice.lsb), slice.msb.max(slice.lsb)));
        }
        ranges.sort_unstable();
        for pair in ranges.windows(2) {
            let (_, previous_high) = pair[0];
            let (next_low, _) = pair[1];
            if next_low <= previous_high {
                return Err(BusParseError::DuplicateMember(next_low));
            }
        }
        Ok(())
    }
}

impl fmt::Display for BusDeclaration {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        format_range(formatter, &self.name, self.msb, self.lsb, self.notation)
    }
}

/// A validated scalar member or contiguous slice selected from a typed bus.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct BusSlice {
    pub name: String,
    pub msb: u32,
    pub lsb: u32,
    pub notation: BusNotation,
}

impl BusSlice {
    pub fn new(
        name: impl Into<String>,
        msb: u32,
        lsb: u32,
        notation: BusNotation,
    ) -> Result<Self, BusParseError> {
        let slice = Self {
            name: name.into(),
            msb,
            lsb,
            notation,
        };
        slice.validate()?;
        Ok(slice)
    }

    /// Parse `NAME[index]`, `NAME<index>`, or a contiguous range.
    pub fn parse(input: &str) -> Result<Self, BusParseError> {
        let parsed = ParsedBusName::parse(input, true)?;
        Self::new(parsed.name, parsed.msb, parsed.lsb, parsed.notation)
    }

    pub fn validate(&self) -> Result<(), BusParseError> {
        validate_identifier(&self.name)?;
        validate_index(self.msb)?;
        validate_index(self.lsb)
    }

    pub fn width(&self) -> usize {
        self.msb.abs_diff(self.lsb) as usize + 1
    }

    pub fn is_scalar(&self) -> bool {
        self.msb == self.lsb
    }

    pub fn direction(&self) -> BusDirection {
        if self.msb < self.lsb {
            BusDirection::Ascending
        } else {
            BusDirection::Descending
        }
    }

    pub fn target_kind(&self) -> BusTargetKind {
        if self.is_scalar() {
            BusTargetKind::Wire
        } else {
            BusTargetKind::Bus
        }
    }

    pub fn members(&self) -> Vec<BusMember> {
        expand_members(&self.name, self.msb, self.lsb, self.notation)
    }
}

impl fmt::Display for BusSlice {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_scalar() {
            let (open, close) = self.notation.delimiters();
            write!(formatter, "{}{open}{}{close}", self.name, self.msb)
        } else {
            format_range(formatter, &self.name, self.msb, self.lsb, self.notation)
        }
    }
}

/// Structured bus parsing and validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusParseError {
    Empty,
    InvalidSyntax,
    InvalidIdentifier,
    InvalidIndex,
    /// The member index a declaration or selector asked for, above
    /// [`MAX_BUS_MEMBER_INDEX`]. Carried as written so the refusal echoes the
    /// author's own spelling, a literal too wide for `u32` included.
    IndexOutOfRange(String),
    DeclarationWidthTooSmall,
    MixedBase {
        expected: String,
        found: String,
    },
    MixedNotation,
    DirectionMismatch,
    SelectorOutOfRange,
    DuplicateMember(u32),
    InvalidGeometry,
    InvalidBusReference,
    UndeclaredBus,
    DeclarationMismatch,
    /// The durable object changed after an editor captured its baseline.
    StaleObject,
    /// A selector edit would create a known scalar/bus or range mismatch at
    /// the retained destination anchor.
    InvalidDestination,
    ReadOnly,
}

impl fmt::Display for BusParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => formatter.write_str("bus declaration is empty"),
            Self::InvalidSyntax => formatter.write_str("invalid bus declaration syntax"),
            Self::InvalidIdentifier => formatter.write_str("invalid bus base name"),
            Self::InvalidIndex => formatter.write_str("invalid bus member index"),
            Self::IndexOutOfRange(index) => {
                write!(
                    formatter,
                    "bus member index {index} is out of range: the highest member \
                     a bus declares is {MAX_BUS_MEMBER_INDEX}"
                )
            }
            Self::DeclarationWidthTooSmall => {
                formatter.write_str("a bus declaration must contain at least two members")
            }
            Self::MixedBase { expected, found } => {
                write!(
                    formatter,
                    "selector base {found} does not match bus base {expected}"
                )
            }
            Self::MixedNotation => {
                formatter.write_str("selector delimiter style does not match bus")
            }
            Self::DirectionMismatch => formatter.write_str("selector direction does not match bus"),
            Self::SelectorOutOfRange => {
                formatter.write_str("selector is outside the declared bus range")
            }
            Self::DuplicateMember(index) => {
                write!(formatter, "bus member {index} is declared more than once")
            }
            Self::InvalidGeometry => formatter.write_str("invalid bus or bus-tap geometry"),
            Self::InvalidBusReference => {
                formatter.write_str("bus tap does not reference its source bus")
            }
            Self::UndeclaredBus => formatter.write_str("bus tap requires a typed bus declaration"),
            Self::DeclarationMismatch => {
                formatter.write_str("bus declaration does not match the pending tap configuration")
            }
            Self::StaleObject => {
                formatter.write_str("the object changed while its properties were open")
            }
            Self::InvalidDestination => {
                formatter.write_str("bus-tap destination is incompatible with the selector")
            }
            Self::ReadOnly => formatter.write_str("schematic is read-only"),
        }
    }
}

impl Error for BusParseError {}

struct ParsedBusName {
    name: String,
    msb: u32,
    lsb: u32,
    notation: BusNotation,
}

impl ParsedBusName {
    fn parse(input: &str, allow_scalar: bool) -> Result<Self, BusParseError> {
        let input = input.trim();
        if input.is_empty() {
            return Err(BusParseError::Empty);
        }

        let square = input.find('[').map(|index| (index, BusNotation::Square));
        let angle = input.find('<').map(|index| (index, BusNotation::Angle));
        let (open_index, notation) = match (square, angle) {
            (Some(_), Some(_)) => return Err(BusParseError::InvalidSyntax),
            (Some(found), None) | (None, Some(found)) => found,
            (None, None) => return Err(BusParseError::InvalidSyntax),
        };
        let (_, close) = notation.delimiters();
        if !input.ends_with(close) || input[..open_index].contains([']', '>']) {
            return Err(BusParseError::InvalidSyntax);
        }
        let raw_name = &input[..open_index];
        if raw_name.trim() != raw_name {
            return Err(BusParseError::InvalidIdentifier);
        }
        let name = raw_name;
        validate_identifier(name)?;
        let body = &input[open_index + 1..input.len() - close.len_utf8()];
        if body.trim() != body || body.is_empty() {
            return Err(BusParseError::InvalidSyntax);
        }
        let mut parts = body.split(':');
        let msb = parse_index(parts.next().ok_or(BusParseError::InvalidSyntax)?)?;
        let second = parts.next();
        if parts.next().is_some() {
            return Err(BusParseError::InvalidSyntax);
        }
        let lsb = match second {
            Some(value) if !value.is_empty() => parse_index(value)?,
            Some(_) => return Err(BusParseError::InvalidSyntax),
            None if allow_scalar => msb,
            None => return Err(BusParseError::DeclarationWidthTooSmall),
        };
        Ok(Self {
            name: name.to_owned(),
            msb,
            lsb,
            notation,
        })
    }
}

fn validate_identifier(name: &str) -> Result<(), BusParseError> {
    let mut chars = name.chars();
    let Some(first) = chars.next() else {
        return Err(BusParseError::InvalidIdentifier);
    };
    if !(first.is_ascii_alphabetic() || first == '_')
        || !chars.all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '.' | '$'))
    {
        return Err(BusParseError::InvalidIdentifier);
    }
    Ok(())
}

fn parse_index(value: &str) -> Result<u32, BusParseError> {
    if value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(BusParseError::InvalidIndex);
    }
    // A literal too wide for `u32` is past the budget by any reading, so it
    // earns the same refusal an in-range type would have produced. What it
    // cannot do is restate itself as a `u32`, which is why the refusal carries
    // the digits rather than a number.
    let index = value
        .parse::<u32>()
        .map_err(|_| BusParseError::IndexOutOfRange(value.to_owned()))?;
    validate_index(index)?;
    Ok(index)
}

fn validate_index(index: u32) -> Result<(), BusParseError> {
    if index > MAX_BUS_MEMBER_INDEX {
        Err(BusParseError::IndexOutOfRange(index.to_string()))
    } else {
        Ok(())
    }
}

fn expand_members(name: &str, msb: u32, lsb: u32, notation: BusNotation) -> Vec<BusMember> {
    let mut members = Vec::with_capacity(msb.abs_diff(lsb) as usize + 1);
    if msb <= lsb {
        for index in msb..=lsb {
            members.push(BusMember {
                name: name.to_owned(),
                index,
                notation,
            });
        }
    } else {
        for index in (lsb..=msb).rev() {
            members.push(BusMember {
                name: name.to_owned(),
                index,
                notation,
            });
        }
    }
    members
}

fn format_range(
    formatter: &mut fmt::Formatter<'_>,
    name: &str,
    msb: u32,
    lsb: u32,
    notation: BusNotation,
) -> fmt::Result {
    let (open, close) = notation.delimiters();
    write!(formatter, "{name}{open}{msb}:{lsb}{close}")
}

/// The vector a name declares, or `None` when the name is one conductor.
///
/// A name IS its declaration. `DATA[7:0]` carries eight conductors wherever it
/// is written — on bus geometry, on an interface port, on a symbol pin, on an
/// instance terminal — and every other name carries one. Holding that rule in
/// a single place is what stops a port, a pin and the bus they meet on from
/// disagreeing about how wide a connection is; nothing downstream re-reads the
/// delimiters itself.
pub fn declared_vector(name: &str) -> Option<BusDeclaration> {
    BusDeclaration::parse(name.trim()).ok()
}

/// Conductors a name carries: the declared width of a vector, otherwise one.
pub fn declared_width(name: &str) -> usize {
    declared_vector(name).map_or(1, |declaration| declaration.width())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declarations_preserve_direction_style_and_member_order() {
        let descending = BusDeclaration::parse("DATA[15:12]").unwrap();
        assert_eq!(descending.direction(), BusDirection::Descending);
        assert_eq!(descending.notation, BusNotation::Square);
        assert_eq!(descending.width(), 4);
        assert_eq!(
            descending
                .members()
                .into_iter()
                .map(|member| member.to_string())
                .collect::<Vec<_>>(),
            ["DATA[15]", "DATA[14]", "DATA[13]", "DATA[12]"]
        );

        let ascending = BusDeclaration::parse("ADDR<0:3>").unwrap();
        assert_eq!(ascending.direction(), BusDirection::Ascending);
        assert_eq!(ascending.notation, BusNotation::Angle);
        assert_eq!(ascending.to_string(), "ADDR<0:3>");
    }

    #[test]
    fn scalar_and_slice_parsing_exposes_connection_target_kind() {
        let scalar = BusSlice::parse("DATA[3]").unwrap();
        assert!(scalar.is_scalar());
        assert_eq!(scalar.target_kind(), BusTargetKind::Wire);
        assert_eq!(scalar.to_string(), "DATA[3]");

        let slice = BusSlice::parse("DATA[7:0]").unwrap();
        assert_eq!(slice.width(), 8);
        assert_eq!(slice.target_kind(), BusTargetKind::Bus);
    }

    #[test]
    fn malformed_and_single_member_declarations_are_rejected() {
        for malformed in [
            "",
            "DATA",
            "7DATA[7:0]",
            "DATA[7:]",
            "DATA[-1:0]",
            "DATA[7:0",
            "DATA[7:0>",
            "DATA [7:0]",
            "DATA[7:0]junk",
            "DATA[7:0:1]",
        ] {
            assert!(
                BusDeclaration::parse(malformed).is_err(),
                "{malformed:?} must be rejected"
            );
        }
        assert_eq!(
            BusDeclaration::parse("DATA[3:3]"),
            Err(BusParseError::DeclarationWidthTooSmall)
        );
        assert_eq!(
            BusDeclaration::parse("DATA[4096:0]"),
            Err(BusParseError::IndexOutOfRange("4096".to_owned()))
        );
    }

    #[test]
    fn slice_validation_accepts_reversible_order_but_rejects_base_style_and_range_errors() {
        let declaration = BusDeclaration::parse("DATA[15:0]").unwrap();
        assert!(
            declaration
                .validate_slice(&BusSlice::parse("DATA[7:0]").unwrap())
                .is_ok()
        );
        assert_eq!(
            declaration.validate_slice(&BusSlice::parse("ADDR[7:0]").unwrap()),
            Err(BusParseError::MixedBase {
                expected: "DATA".into(),
                found: "ADDR".into()
            })
        );
        assert_eq!(
            declaration.validate_slice(&BusSlice::parse("DATA<7:0>").unwrap()),
            Err(BusParseError::MixedNotation)
        );
        assert!(
            declaration
                .validate_slice(&BusSlice::parse("DATA[0:7]").unwrap())
                .is_ok()
        );
        assert!(
            declaration
                .validate_slice(&BusSlice::parse("DATA[0:3]").unwrap())
                .is_ok()
        );
        assert_eq!(
            declaration.validate_slice(&BusSlice::parse("DATA[20:16]").unwrap()),
            Err(BusParseError::SelectorOutOfRange)
        );
    }

    #[test]
    fn overlapping_selector_members_are_rejected() {
        let declaration = BusDeclaration::parse("DATA[15:0]").unwrap();
        let selectors = [
            BusSlice::parse("DATA[7:4]").unwrap(),
            BusSlice::parse("DATA[5]").unwrap(),
        ];
        assert_eq!(
            declaration.validate_slices(&selectors),
            Err(BusParseError::DuplicateMember(5))
        );
    }

    #[test]
    fn a_name_is_its_own_width_declaration() {
        let declaration = declared_vector("DATA[7:0]").expect("a range name declares a vector");
        assert_eq!(declaration.width(), 8);
        assert_eq!(declared_width("DATA[7:0]"), 8);
        assert_eq!(declared_width("ADDR<0:2>"), 3);
        // Everything else carries one conductor, including a single-member
        // selector, which is a bit of a bus and never a bus itself.
        for scalar in ["OUT", "DATA[3]", "DATA", "0", "vdd!"] {
            assert!(declared_vector(scalar).is_none(), "{scalar}");
            assert_eq!(declared_width(scalar), 1, "{scalar}");
        }
    }

    #[test]
    fn bit_position_follows_declaration_order_at_either_end() {
        let descending = BusDeclaration::parse("DATA[7:0]").unwrap();
        assert_eq!(descending.bit_position(7), Some(0));
        assert_eq!(descending.bit_position(0), Some(7));
        assert_eq!(descending.bit_position(8), None);

        let ascending = BusDeclaration::parse("DATA[0:7]").unwrap();
        assert_eq!(ascending.bit_position(7), Some(7));
        assert_eq!(ascending.bit_position(0), Some(0));
    }
}

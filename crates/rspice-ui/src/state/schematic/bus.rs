//! Typed bus and bus-tap design data.
//!
//! Bus declarations are deliberately parsed into a durable semantic model.
//! Downstream connectivity and netlisting code never has to reinterpret an
//! arbitrary display string, which keeps range direction, delimiter style,
//! and scalar-versus-slice intent unambiguous across persistence boundaries.

use std::error::Error;
use std::fmt;

use serde::{Deserialize, Serialize};

use super::{Point, WireRoutingMode};

/// Highest supported bus member index.
///
/// The limit prevents hostile or corrupt project files from requesting
/// unbounded member expansion while remaining far above practical IC buses.
pub const MAX_BUS_MEMBER_INDEX: u32 = 1_048_575;

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
    const fn delimiters(self) -> (char, char) {
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

/// Exact mutation scope resolved for one bus-property transaction.
///
/// The GUI uses the same domain plan that will commit the edit, so a
/// connected-network refactor is never presented as a single-object change.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BusPropertyImpact {
    pub connected_buses: usize,
    pub buses_changed: usize,
    pub taps_changed: usize,
}

impl BusPropertyImpact {
    pub const fn has_changes(self) -> bool {
        self.buses_changed != 0 || self.taps_changed != 0
    }
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

/// User-controlled orientation of a bus tap.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BusTapOrientation {
    #[default]
    Automatic,
    Left,
    Right,
    Up,
    Down,
}

/// Validated configuration retained while the bus-tap placement tool is armed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingBusTap {
    pub bus_declaration: BusDeclaration,
    pub slice: BusSlice,
    pub orientation: BusTapOrientation,
}

impl PendingBusTap {
    pub fn new(
        bus_declaration: BusDeclaration,
        slice: BusSlice,
        orientation: BusTapOrientation,
    ) -> Result<Self, BusParseError> {
        bus_declaration.validate_slice(&slice)?;
        Ok(Self {
            bus_declaration,
            slice,
            orientation,
        })
    }
}

/// Runtime state machine for interactive bus routing.
#[derive(Debug, Clone, Default)]
pub struct BusDrawing {
    pub points: Vec<Point>,
    pub active: bool,
    pub preview_pos: Option<Point>,
    pub routing_mode: WireRoutingMode,
    pub declaration: Option<BusDeclaration>,
}

impl BusDrawing {
    pub fn start(&mut self, position: Point, declaration: Option<BusDeclaration>) {
        self.points.clear();
        self.points.push(position);
        self.active = true;
        self.preview_pos = Some(position);
        self.declaration = declaration;
    }

    pub fn update_preview(&mut self, position: Point) {
        if self.active {
            self.preview_pos = Some(position);
        }
    }

    pub fn add_point(&mut self, position: Point) {
        let Some(start) = self.points.last().copied() else {
            return;
        };
        if !self.active || start == position {
            return;
        }
        for point in self.routing_mode.suggest_route(start, position) {
            if self.points.last() != Some(&point) {
                self.points.push(point);
            }
        }
        self.preview_pos = Some(position);
    }

    pub fn preview_path(&self) -> Vec<Point> {
        let (Some(start), Some(end)) = (self.points.last().copied(), self.preview_pos) else {
            return Vec::new();
        };
        let mut path = vec![start];
        if start != end {
            path.extend(self.routing_mode.suggest_route(start, end));
        }
        path
    }

    pub fn cancel(&mut self) {
        self.points.clear();
        self.active = false;
        self.preview_pos = None;
        self.declaration = None;
    }
}

/// A durable polyline carrying an optional typed declaration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Bus {
    pub id: u64,
    pub points: Vec<Point>,
    pub declaration: Option<BusDeclaration>,
}

impl Bus {
    pub fn new(
        id: u64,
        points: Vec<Point>,
        declaration: Option<BusDeclaration>,
    ) -> Result<Self, BusParseError> {
        let bus = Self {
            id,
            points,
            declaration,
        };
        bus.validate()?;
        Ok(bus)
    }

    pub fn segment(
        id: u64,
        start: Point,
        end: Point,
        declaration: Option<BusDeclaration>,
    ) -> Result<Self, BusParseError> {
        Self::new(id, vec![start, end], declaration)
    }

    pub fn validate(&self) -> Result<(), BusParseError> {
        if self.points.len() < 2 || self.points.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(BusParseError::InvalidGeometry);
        }
        if let Some(declaration) = &self.declaration {
            declaration.validate()?;
        }
        Ok(())
    }

    pub fn contains_point(&self, point: Point) -> bool {
        self.points
            .windows(2)
            .any(|pair| point_on_segment(point, pair[0], pair[1]))
    }

    pub fn translate(&mut self, delta: Point) {
        for point in &mut self.points {
            point.x = point.x.saturating_add(delta.x);
            point.y = point.y.saturating_add(delta.y);
        }
    }
}

/// A typed scalar or slice connection emerging from a declared bus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BusTap {
    pub id: u64,
    pub bus_id: u64,
    pub bus_point: Point,
    pub connection_point: Point,
    pub slice: BusSlice,
    pub orientation: BusTapOrientation,
}

impl BusTap {
    pub fn new(
        id: u64,
        bus: &Bus,
        bus_point: Point,
        connection_point: Point,
        slice: BusSlice,
        orientation: BusTapOrientation,
    ) -> Result<Self, BusParseError> {
        let tap = Self {
            id,
            bus_id: bus.id,
            bus_point,
            connection_point,
            slice,
            orientation,
        };
        tap.validate_against_bus(bus)?;
        Ok(tap)
    }

    pub fn validate_against_bus(&self, bus: &Bus) -> Result<(), BusParseError> {
        if self.bus_id != bus.id || !bus.contains_point(self.bus_point) {
            return Err(BusParseError::InvalidBusReference);
        }
        if self.bus_point == self.connection_point {
            return Err(BusParseError::InvalidGeometry);
        }
        let declaration = bus
            .declaration
            .as_ref()
            .ok_or(BusParseError::UndeclaredBus)?;
        declaration.validate_slice(&self.slice)
    }

    pub fn width(&self) -> usize {
        self.slice.width()
    }

    pub fn members(&self) -> Vec<BusMember> {
        self.slice.members()
    }

    pub fn target_kind(&self) -> BusTargetKind {
        self.slice.target_kind()
    }

    pub fn translate(&mut self, delta: Point) {
        self.bus_point.x = self.bus_point.x.saturating_add(delta.x);
        self.bus_point.y = self.bus_point.y.saturating_add(delta.y);
        self.connection_point.x = self.connection_point.x.saturating_add(delta.x);
        self.connection_point.y = self.connection_point.y.saturating_add(delta.y);
    }
}

/// Structured bus parsing and validation failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BusParseError {
    Empty,
    InvalidSyntax,
    InvalidIdentifier,
    InvalidIndex,
    IndexOutOfRange,
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
            Self::IndexOutOfRange => formatter.write_str("bus member index is out of range"),
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
    let index = value
        .parse::<u32>()
        .map_err(|_| BusParseError::IndexOutOfRange)?;
    validate_index(index)?;
    Ok(index)
}

fn validate_index(index: u32) -> Result<(), BusParseError> {
    if index > MAX_BUS_MEMBER_INDEX {
        Err(BusParseError::IndexOutOfRange)
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

fn point_on_segment(point: Point, start: Point, end: Point) -> bool {
    let px = i128::from(point.x);
    let py = i128::from(point.y);
    let ax = i128::from(start.x);
    let ay = i128::from(start.y);
    let bx = i128::from(end.x);
    let by = i128::from(end.y);
    let cross = (px - ax) * (by - ay) - (py - ay) * (bx - ax);
    cross == 0 && px >= ax.min(bx) && px <= ax.max(bx) && py >= ay.min(by) && py <= ay.max(by)
}

pub(crate) fn nearest_lattice_point_on_segment(point: Point, start: Point, end: Point) -> Point {
    let dx = i64::from(end.x) - i64::from(start.x);
    let dy = i64::from(end.y) - i64::from(start.y);
    let steps = gcd_u64(dx.unsigned_abs(), dy.unsigned_abs());
    if steps == 0 {
        return start;
    }
    let dx128 = i128::from(dx);
    let dy128 = i128::from(dy);
    let denominator = dx128 * dx128 + dy128 * dy128;
    let numerator = (i128::from(point.x) - i128::from(start.x)) * dx128
        + (i128::from(point.y) - i128::from(start.y)) * dy128;
    let steps128 = i128::from(steps);
    let step = if numerator <= 0 {
        0
    } else if numerator >= denominator {
        steps128
    } else {
        (numerator * steps128 + denominator / 2) / denominator
    };
    let x = i128::from(start.x) + dx128 * step / steps128;
    let y = i128::from(start.y) + dy128 * step / steps128;
    Point::new(
        i32::try_from(x).expect("lattice projection remains inside its i32 segment"),
        i32::try_from(y).expect("lattice projection remains inside its i32 segment"),
    )
}

fn gcd_u64(mut left: u64, mut right: u64) -> u64 {
    while right != 0 {
        let remainder = left % right;
        left = right;
        right = remainder;
    }
    left
}

impl Bus {
    /// Nearest grid point on the polyline and its squared distance.
    pub fn nearest_point(&self, point: Point) -> Option<(Point, i128)> {
        self.points
            .windows(2)
            .map(|pair| nearest_lattice_point_on_segment(point, pair[0], pair[1]))
            .map(|candidate| {
                let dx = i128::from(candidate.x) - i128::from(point.x);
                let dy = i128::from(candidate.y) - i128::from(point.y);
                (candidate, dx * dx + dy * dy)
            })
            .min_by_key(|(_, distance)| *distance)
    }
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
            BusDeclaration::parse("DATA[1048576:0]"),
            Err(BusParseError::IndexOutOfRange)
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
    fn tap_validation_checks_source_geometry_and_type() {
        let declaration = BusDeclaration::parse("DATA[7:0]").unwrap();
        let bus = Bus::segment(4, Point::new(0, 0), Point::new(20, 0), Some(declaration)).unwrap();
        let tap = BusTap::new(
            5,
            &bus,
            Point::new(10, 0),
            Point::new(10, 5),
            BusSlice::parse("DATA[3]").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        assert_eq!(tap.target_kind(), BusTargetKind::Wire);
        assert_eq!(tap.members()[0].to_string(), "DATA[3]");
        assert!(
            BusTap::new(
                6,
                &bus,
                Point::new(10, 1),
                Point::new(10, 5),
                BusSlice::parse("DATA[3]").unwrap(),
                BusTapOrientation::Automatic,
            )
            .is_err()
        );
    }

    #[test]
    fn durable_models_preserve_stable_ids_and_types_through_serde() {
        let bus = Bus::segment(
            41,
            Point::new(-5, 2),
            Point::new(15, 2),
            Some(BusDeclaration::parse("CTRL<0:3>").unwrap()),
        )
        .unwrap();
        let tap = BusTap::new(
            42,
            &bus,
            Point::new(5, 2),
            Point::new(5, 8),
            BusSlice::parse("CTRL<2>").unwrap(),
            BusTapOrientation::Down,
        )
        .unwrap();
        let encoded = serde_json::to_string(&(bus.clone(), tap.clone())).unwrap();
        let decoded: (Bus, BusTap) = serde_json::from_str(&encoded).unwrap();
        assert_eq!(decoded, (bus, tap));
    }

    #[test]
    fn extreme_coordinate_geometry_does_not_overflow() {
        let diagonal = Bus::segment(
            1,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MAX),
            None,
        )
        .unwrap();
        assert!(diagonal.contains_point(Point::origin()));
        let bus = Bus::segment(
            2,
            Point::new(i32::MIN, i32::MIN),
            Point::new(i32::MAX, i32::MIN),
            None,
        )
        .unwrap();
        let (nearest, distance) = bus.nearest_point(Point::new(i32::MIN, i32::MAX)).unwrap();
        assert!(distance > i128::from(i64::MAX));
        assert_eq!(nearest, Point::new(i32::MIN, i32::MIN));
    }

    #[test]
    fn nearest_point_on_any_angle_bus_is_always_an_exact_lattice_member() {
        let sparse = Bus::segment(20, Point::new(0, 0), Point::new(10, 3), None).unwrap();
        let (sparse_hit, _) = sparse.nearest_point(Point::new(5, 2)).unwrap();
        assert!(sparse.contains_point(sparse_hit));
        assert!(matches!(
            sparse_hit,
            Point { x: 0, y: 0 } | Point { x: 10, y: 3 }
        ));

        let dense = Bus::segment(21, Point::new(0, 0), Point::new(10, 4), None).unwrap();
        let (dense_hit, _) = dense.nearest_point(Point::new(6, 3)).unwrap();
        assert_eq!(dense_hit, Point::new(5, 2));
        assert!(dense.contains_point(dense_hit));
    }
}

//! Schematics drawn in LTspice (`.asc`) and QSPICE (`.qsch`).
//!
//! Both readers produce one [`ForeignSchematic`]: the records of the source
//! drawing in its own units, with the y axis pointing down and every symbol
//! orientation stated the way LTspice states it. Nothing here decides what a
//! record becomes in RSpice; that is the importer's job.

use std::fmt;

mod behaviour;
mod directives;
mod functional;
mod import;
mod installed;
mod ltspice;
mod mapping;
mod qspice;

pub use directives::{
    ForeignCard, ForeignCardRole, defined_model_names, foreign_directive_cards, keep_directive_text,
};
pub use import::{
    ForeignDirective, ForeignImport, ForeignImportNotice, ForeignLibrary, ForeignModelFile,
    ForeignModelSource, ForeignNoticeSeverity, ForeignSymbolCell, ForeignSymbolSource,
    bind_instances_to_sources, import_foreign_schematic,
};
pub use installed::{InstalledForeignLibrary, is_encrypted};
pub use ltspice::{parse_asc, parse_asy};
pub use qspice::parse_qsch;

#[cfg(test)]
mod import_tests;
#[cfg(test)]
mod tests;

/// The program a foreign schematic was drawn in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ForeignTool {
    Ltspice,
    Qspice,
}

impl ForeignTool {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Ltspice => "LTspice",
            Self::Qspice => "QSPICE",
        }
    }

    /// Drawing units between two adjacent grid points.
    pub const fn units_per_grid(self) -> i32 {
        match self {
            Self::Ltspice => 16,
            Self::Qspice => 100,
        }
    }
}

/// A point in the source drawing's units, y pointing down.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ForeignPoint {
    pub x: i32,
    pub y: i32,
}

impl ForeignPoint {
    pub const fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub const fn offset(self, by: ForeignPoint) -> Self {
        Self::new(self.x.saturating_add(by.x), self.y.saturating_add(by.y))
    }
}

/// How a symbol is turned on the sheet, in LTspice's terms: `R<n>` turns it
/// clockwise by `n` degrees, and `M<n>` turns it the same way and then
/// mirrors it left to right.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct ForeignOrientation {
    /// Clockwise quarter turns, 0 to 3.
    pub quarter_turns: u8,
    pub mirrored: bool,
}

impl ForeignOrientation {
    pub const ALL: [Self; 8] = [
        Self::new(0, false),
        Self::new(1, false),
        Self::new(2, false),
        Self::new(3, false),
        Self::new(0, true),
        Self::new(1, true),
        Self::new(2, true),
        Self::new(3, true),
    ];

    pub const fn new(quarter_turns: u8, mirrored: bool) -> Self {
        Self {
            quarter_turns: quarter_turns % 4,
            mirrored,
        }
    }

    /// Reads `R0`…`R270` and `M0`…`M270`.
    pub fn from_ltspice(text: &str) -> Option<Self> {
        let (mirrored, degrees) = match text.as_bytes().first()? {
            b'R' | b'r' => (false, &text[1..]),
            b'M' | b'm' => (true, &text[1..]),
            _ => return None,
        };
        let quarter_turns = match degrees {
            "0" => 0,
            "90" => 1,
            "180" => 2,
            "270" => 3,
            _ => return None,
        };
        Some(Self::new(quarter_turns, mirrored))
    }

    /// Places a point given relative to the symbol origin.
    pub const fn apply(self, point: ForeignPoint) -> ForeignPoint {
        let (x, y) = (point.x, point.y);
        let (x, y) = match self.quarter_turns {
            1 => (-y, x),
            2 => (-x, -y),
            3 => (y, -x),
            _ => (x, y),
        };
        if self.mirrored {
            ForeignPoint::new(-x, y)
        } else {
            ForeignPoint::new(x, y)
        }
    }

    pub fn ltspice_name(self) -> String {
        format!(
            "{}{}",
            if self.mirrored { 'M' } else { 'R' },
            u16::from(self.quarter_turns) * 90
        )
    }
}

/// One named attribute of a placed symbol (`SYMATTR` in LTspice).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignAttribute {
    pub name: String,
    pub value: String,
}

/// Where a symbol draws one of its attributes (`WINDOW` in LTspice), relative
/// to the symbol origin and before the symbol's orientation is applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignWindow {
    /// LTspice's attribute number: 0 the instance name, 3 the value.
    pub attribute: u32,
    pub offset: ForeignPoint,
    pub alignment: String,
}

/// One electrical pin of a symbol, relative to its origin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignPin {
    pub name: String,
    /// Position of the pin on the instance line, from 1.
    pub spice_order: usize,
    pub position: ForeignPoint,
}

/// Drawing primitives, in the units of the drawing that holds them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForeignShape {
    Line(ForeignPoint, ForeignPoint),
    Rectangle(ForeignPoint, ForeignPoint),
    /// An ellipse inscribed in the box between two corners.
    Ellipse(ForeignPoint, ForeignPoint),
    /// The part of the ellipse inscribed in the box between the first two
    /// points, counter-clockwise from the ray through the third point to the
    /// ray through the fourth.
    Arc([ForeignPoint; 4]),
}

/// A symbol's definition: what its `.asy` file says, or what a QSPICE
/// schematic embeds for the component.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ForeignSymbolDefinition {
    /// `CELL` or `BLOCK` for LTspice; QSPICE's device type character.
    pub symbol_type: String,
    pub pins: Vec<ForeignPin>,
    pub shapes: Vec<ForeignShape>,
    /// Default attributes: `Prefix`, `Value`, `SpiceModel`, `ModelFile`, …
    pub attributes: Vec<ForeignAttribute>,
    pub windows: Vec<ForeignWindow>,
}

impl ForeignSymbolDefinition {
    pub fn attribute(&self, name: &str) -> Option<&str> {
        find_attribute(&self.attributes, name)
    }

    /// Pins in instance-line order.
    pub fn ordered_pins(&self) -> Vec<&ForeignPin> {
        let mut pins = self.pins.iter().collect::<Vec<_>>();
        pins.sort_by_key(|pin| pin.spice_order);
        pins
    }
}

/// A placed symbol.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSymbol {
    /// The symbol's name as the drawing writes it, library folders included
    /// (`res`, `Digital\and`, `OpAmps\LT1001`).
    pub symbol: String,
    pub position: ForeignPoint,
    pub orientation: ForeignOrientation,
    pub attributes: Vec<ForeignAttribute>,
    pub windows: Vec<ForeignWindow>,
    /// The definition the drawing carries itself (QSPICE always does).
    pub embedded: Option<ForeignSymbolDefinition>,
    /// Source line of the record, from 1.
    pub line: usize,
}

impl ForeignSymbol {
    pub fn attribute(&self, name: &str) -> Option<&str> {
        find_attribute(&self.attributes, name)
    }

    /// The symbol's file name without folders: `Digital\and` is `and`.
    pub fn base_name(&self) -> &str {
        self.symbol
            .rsplit(['\\', '/'])
            .next()
            .unwrap_or(&self.symbol)
    }
}

fn find_attribute<'a>(attributes: &'a [ForeignAttribute], name: &str) -> Option<&'a str> {
    attributes
        .iter()
        .find(|attribute| attribute.name.eq_ignore_ascii_case(name))
        .map(|attribute| attribute.value.as_str())
}

/// How a net flag joins the drawing to the outside.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForeignPortDirection {
    Input,
    Output,
    Bidirectional,
}

/// A net name placed on a point (`FLAG`); `0` names ground.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignFlag {
    pub position: ForeignPoint,
    pub name: String,
    /// Set when an `IOPIN` makes the flag a pin of the drawing.
    pub port: Option<ForeignPortDirection>,
}

impl ForeignFlag {
    pub fn is_ground(&self) -> bool {
        self.name == "0"
    }
}

/// Whether a text block is simulator input or a note to the reader.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ForeignTextKind {
    Directive,
    Comment,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignText {
    pub position: ForeignPoint,
    pub kind: ForeignTextKind,
    /// The text with its line breaks restored.
    pub text: String,
    pub line: usize,
}

/// A waveform readout placed on a net (`DATAFLAG`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignDataFlag {
    pub position: ForeignPoint,
    pub expression: String,
}

/// A whole foreign drawing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignSchematic {
    pub tool: ForeignTool,
    pub wires: Vec<(ForeignPoint, ForeignPoint)>,
    /// Net names QSPICE records on each wire; empty for LTspice.
    pub wire_nets: Vec<String>,
    pub flags: Vec<ForeignFlag>,
    pub symbols: Vec<ForeignSymbol>,
    pub texts: Vec<ForeignText>,
    pub shapes: Vec<ForeignShape>,
    /// Junction dots the drawing states (QSPICE); LTspice derives them.
    pub junctions: Vec<ForeignPoint>,
    pub data_flags: Vec<ForeignDataFlag>,
    pub bus_taps: Vec<(ForeignPoint, ForeignPoint)>,
}

impl ForeignSchematic {
    pub fn new(tool: ForeignTool) -> Self {
        Self {
            tool,
            wires: Vec::new(),
            wire_nets: Vec::new(),
            flags: Vec::new(),
            symbols: Vec::new(),
            texts: Vec::new(),
            shapes: Vec::new(),
            junctions: Vec::new(),
            data_flags: Vec::new(),
            bus_taps: Vec::new(),
        }
    }
}

/// Why a foreign file could not be read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignParseError {
    pub line: usize,
    pub message: String,
}

impl ForeignParseError {
    pub(crate) fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }
}

impl fmt::Display for ForeignParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.line == 0 {
            f.write_str(&self.message)
        } else {
            write!(f, "line {}: {}", self.line, self.message)
        }
    }
}

impl std::error::Error for ForeignParseError {}

/// Text from the bytes LTspice and QSPICE write: UTF-16 (little or big
/// endian, with or without a byte-order mark), UTF-8, or Windows Latin-1.
pub fn decode_foreign_text(bytes: &[u8]) -> String {
    if let Some(rest) = bytes.strip_prefix(&[0xFF, 0xFE]) {
        return decode_utf16(rest, u16::from_le_bytes);
    }
    if let Some(rest) = bytes.strip_prefix(&[0xFE, 0xFF]) {
        return decode_utf16(rest, u16::from_be_bytes);
    }
    // UTF-16 without a mark: ASCII text puts a zero in every other byte.
    let probe = &bytes[..bytes.len().min(64)];
    if probe.len() >= 4 {
        let zeros_odd = probe.iter().skip(1).step_by(2).filter(|b| **b == 0).count();
        let zeros_even = probe.iter().step_by(2).filter(|b| **b == 0).count();
        let half = probe.len() / 2;
        if zeros_odd * 4 >= half * 3 && zeros_even == 0 {
            return decode_utf16(bytes, u16::from_le_bytes);
        }
        if zeros_even * 4 >= half * 3 && zeros_odd == 0 {
            return decode_utf16(bytes, u16::from_be_bytes);
        }
    }
    let bytes = bytes.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(bytes);
    match std::str::from_utf8(bytes) {
        Ok(text) => text.to_owned(),
        Err(_) => bytes.iter().map(|byte| windows_1252(*byte)).collect(),
    }
}

fn decode_utf16(bytes: &[u8], word: fn([u8; 2]) -> u16) -> String {
    let words = bytes
        .chunks_exact(2)
        .map(|pair| word([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    String::from_utf16_lossy(&words)
}

/// The Windows code page both programs fall back to, where 0x80–0x9F hold
/// punctuation rather than Latin-1's control characters.
fn windows_1252(byte: u8) -> char {
    const HIGH: [char; 32] = [
        '\u{20AC}', '\u{81}', '\u{201A}', '\u{192}', '\u{201E}', '\u{2026}', '\u{2020}',
        '\u{2021}', '\u{2C6}', '\u{2030}', '\u{160}', '\u{2039}', '\u{152}', '\u{8D}', '\u{17D}',
        '\u{8F}', '\u{90}', '\u{2018}', '\u{2019}', '\u{201C}', '\u{201D}', '\u{2022}', '\u{2013}',
        '\u{2014}', '\u{2DC}', '\u{2122}', '\u{161}', '\u{203A}', '\u{153}', '\u{9D}', '\u{17E}',
        '\u{178}',
    ];
    match byte {
        0x80..=0x9F => HIGH[usize::from(byte - 0x80)],
        _ => char::from(byte),
    }
}

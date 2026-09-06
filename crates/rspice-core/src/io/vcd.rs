//! Value Change Dump files, read and written.
//!
//! VCD is the interchange format for event-driven results: irregular
//! per-signal timelines on an integer tick grid, which is what a digital
//! event history already is. [`super::ltspice_raw`] and [`super::raw_export`]
//! carry a sampled table; this module carries transitions.
//!
//! ## Four-state, not analog
//!
//! A logic value here is a vector of [`VcdBit`] — `0`, `1`, `x`, `z` — one
//! entry per declared bit. Nothing is mapped onto `f64`, so `x` and `z`
//! survive a round trip and a variable may be as wide as the format allows.
//! A reader that flattens a vector into a float has to stop at 53 bits,
//! because that is where an integer stops being exact in binary64; that is a
//! limit of the mapping, not of VCD, and this module does not have it.
//!
//! Strength is the other half of RSpice's twelve-state digital value, and VCD
//! cannot express it: the format has four bit states and no drive strength.
//! The projection that drops it lives beside the engine's result types in
//! [`crate::execution::event_vcd_document`], not here, because a format
//! module never names a result type.
//!
//! ## What the reader accepts
//!
//! What IEEE 1364 declares: `$date`, `$version`, `$comment`, `$timescale`,
//! nested `$scope`/`$upscope`, `$var` with aliases (two declarations sharing
//! one identifier code), `$enddefinitions`, `#` time stamps, scalar / `b`
//! vector / `r` real value changes, and the `$dumpvars`, `$dumpon`,
//! `$dumpoff` and `$dumpall` blocks — whose contents are ordinary value
//! changes and are read as such. A directive may be written on one line or
//! spread over several. Unrecognised `$` directives are skipped to their
//! `$end`.
//!
//! Reading is bounded before it allocates, the way [`super::ltspice_raw`] is:
//! [`ResourceLimits::max_external_data_bytes`] caps the input,
//! `max_external_data_values` caps declarations and value changes, and
//! `max_result_values` caps the bits and reals the document retains.
//!
//! ## What the writer emits
//!
//! [`write_vcd`] regenerates identifier codes in the standard printable-ASCII
//! scheme rather than reusing whatever the document carries, so its output is
//! a function of the data alone. Every logic variable is declared `wire` and
//! every scope `module`: the format's other variable and scope types carry no
//! information this document models, and inventing one back would be a claim
//! the data does not support. Everything that is modelled — ticks, bits,
//! reals, widths, aliases and scope paths — comes back from
//! [`parse_vcd_reader`] exactly as it went in.
//!
//! ## Versioning
//!
//! VCD has no schema version, and this module does not invent one. What
//! [`VCD_WRITER_VERSION`] stamps into `$version` is provenance — which build
//! produced the file — and a reader must not gate on it: a dump another tool
//! wrote carries that tool's string, is read all the same, and comes back out
//! of [`write_vcd`] still carrying it. The compatibility contract is the
//! format itself, so the way to extend what RSpice dumps is to declare more
//! `$var`s, never to add a directive a foreign reader would not know.

use std::collections::HashMap;
use std::fmt;
use std::io::{Read, Write};
use std::path::Path;

use thiserror::Error;

use crate::resource::{
    ResourceKind, ResourceLimitError, ResourceLimits, ResourceReadError, read_bytes_limited,
    read_file_bytes_limited,
};

/// The `$version` string RSpice stamps on the documents it produces.
pub const VCD_WRITER_VERSION: &str = concat!("RSpice ", env!("CARGO_PKG_VERSION"), " VCD writer");

/// Errors raised while reading or writing a VCD document.
#[derive(Debug, Error)]
pub enum VcdError {
    /// A configured read limit was reached.
    #[error(transparent)]
    ResourceLimit(#[from] ResourceLimitError),

    /// The underlying stream failed.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// The source is not valid UTF-8. VCD is a text format.
    #[error("VCD source is not UTF-8: {0}")]
    Encoding(#[from] std::str::Utf8Error),

    /// A token could not be read as the grammar requires.
    #[error("VCD line {line}: {message}")]
    Syntax {
        /// One-based line the offending token started on.
        line: usize,
        /// What the grammar expected instead.
        message: String,
    },

    /// The file is well-formed token by token but incomplete as a whole.
    #[error("{0}")]
    Structure(String),

    /// A document was handed to the writer that VCD cannot express.
    #[error("VCD document cannot be written: {0}")]
    Unwritable(String),
}

impl From<ResourceReadError> for VcdError {
    fn from(error: ResourceReadError) -> Self {
        match error {
            ResourceReadError::Io(error) => Self::Io(error),
            ResourceReadError::ResourceLimit(error) => Self::ResourceLimit(error),
        }
    }
}

/// The unit half of a `$timescale`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VcdTimeUnit {
    /// Seconds, `s`.
    Seconds,
    /// Milliseconds, `ms`.
    Milliseconds,
    /// Microseconds, `us`.
    Microseconds,
    /// Nanoseconds, `ns`.
    Nanoseconds,
    /// Picoseconds, `ps`.
    Picoseconds,
    /// Femtoseconds, `fs`.
    Femtoseconds,
}

impl VcdTimeUnit {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Seconds => "s",
            Self::Milliseconds => "ms",
            Self::Microseconds => "us",
            Self::Nanoseconds => "ns",
            Self::Picoseconds => "ps",
            Self::Femtoseconds => "fs",
        }
    }

    const fn femtoseconds(self) -> u64 {
        match self {
            Self::Seconds => 1_000_000_000_000_000,
            Self::Milliseconds => 1_000_000_000_000,
            Self::Microseconds => 1_000_000_000,
            Self::Nanoseconds => 1_000_000,
            Self::Picoseconds => 1_000,
            Self::Femtoseconds => 1,
        }
    }

    const fn seconds(self) -> f64 {
        match self {
            Self::Seconds => 1.0,
            Self::Milliseconds => 1e-3,
            Self::Microseconds => 1e-6,
            Self::Nanoseconds => 1e-9,
            Self::Picoseconds => 1e-12,
            Self::Femtoseconds => 1e-15,
        }
    }
}

impl fmt::Display for VcdTimeUnit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

/// The magnitude half of a `$timescale`.
///
/// VCD admits exactly these three, so the type admits exactly these three:
/// `$timescale 2 ns` is not a timescale the format can carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VcdMagnitude {
    /// `1`.
    One,
    /// `10`.
    Ten,
    /// `100`.
    Hundred,
}

impl VcdMagnitude {
    const fn value(self) -> u64 {
        match self {
            Self::One => 1,
            Self::Ten => 10,
            Self::Hundred => 100,
        }
    }
}

impl fmt::Display for VcdMagnitude {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.value())
    }
}

/// The duration of one tick: a magnitude and a unit, as `$timescale` spells it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VcdTimescale {
    /// `1`, `10` or `100`.
    pub magnitude: VcdMagnitude,
    /// The unit the magnitude multiplies.
    pub unit: VcdTimeUnit,
}

impl VcdTimescale {
    /// Every timescale VCD can express, coarsest tick first.
    ///
    /// The order is what a chooser wants: the first entry whose period divides
    /// every event time is the coarsest scale that still keeps each time an
    /// exact integer tick.
    pub const ALL: [Self; 18] = [
        Self::of(VcdMagnitude::Hundred, VcdTimeUnit::Seconds),
        Self::of(VcdMagnitude::Ten, VcdTimeUnit::Seconds),
        Self::of(VcdMagnitude::One, VcdTimeUnit::Seconds),
        Self::of(VcdMagnitude::Hundred, VcdTimeUnit::Milliseconds),
        Self::of(VcdMagnitude::Ten, VcdTimeUnit::Milliseconds),
        Self::of(VcdMagnitude::One, VcdTimeUnit::Milliseconds),
        Self::of(VcdMagnitude::Hundred, VcdTimeUnit::Microseconds),
        Self::of(VcdMagnitude::Ten, VcdTimeUnit::Microseconds),
        Self::of(VcdMagnitude::One, VcdTimeUnit::Microseconds),
        Self::of(VcdMagnitude::Hundred, VcdTimeUnit::Nanoseconds),
        Self::of(VcdMagnitude::Ten, VcdTimeUnit::Nanoseconds),
        Self::of(VcdMagnitude::One, VcdTimeUnit::Nanoseconds),
        Self::of(VcdMagnitude::Hundred, VcdTimeUnit::Picoseconds),
        Self::of(VcdMagnitude::Ten, VcdTimeUnit::Picoseconds),
        Self::of(VcdMagnitude::One, VcdTimeUnit::Picoseconds),
        Self::of(VcdMagnitude::Hundred, VcdTimeUnit::Femtoseconds),
        Self::of(VcdMagnitude::Ten, VcdTimeUnit::Femtoseconds),
        Self::of(VcdMagnitude::One, VcdTimeUnit::Femtoseconds),
    ];

    const fn of(magnitude: VcdMagnitude, unit: VcdTimeUnit) -> Self {
        Self { magnitude, unit }
    }

    /// Duration of one tick in whole femtoseconds.
    ///
    /// Exact, which is what a tick calculation needs. [`Self::seconds`] is the
    /// same quantity in the units a waveform axis is drawn in, and is not.
    pub const fn femtoseconds(self) -> u64 {
        self.magnitude.value() * self.unit.femtoseconds()
    }

    /// Duration of one tick in seconds.
    pub fn seconds(self) -> f64 {
        self.magnitude.value() as f64 * self.unit.seconds()
    }
}

impl fmt::Display for VcdTimescale {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} {}", self.magnitude, self.unit)
    }
}

/// Whether a signal carries logic bits or a real number.
///
/// VCD's other variable types — `reg`, `integer`, `event`, `time` and the
/// rest — all carry logic bits, and are read as [`Self::Logic`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VcdSignalKind {
    /// A `wire`-class variable: as many four-state bits as it is wide.
    Logic,
    /// A `real`-class variable: one `f64`.
    Real,
}

/// One four-state bit.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VcdBit {
    /// `0`.
    Zero,
    /// `1`.
    One,
    /// `x`: unknown, or a conflict between drivers.
    Unknown,
    /// `z`: high impedance — nothing is driving.
    HighImpedance,
}

impl VcdBit {
    /// The character a dump spells this bit with: `0`, `1`, `x` or `z`.
    ///
    /// Public because a caller assembling a whole word — a bus event, a
    /// viewer's label — pushes one character per bit, and going through
    /// [`fmt::Display`] to get it allocates a `String` per bit.
    pub const fn as_char(self) -> char {
        match self {
            Self::Zero => '0',
            Self::One => '1',
            Self::Unknown => 'x',
            Self::HighImpedance => 'z',
        }
    }

    const fn from_byte(byte: u8) -> Option<Self> {
        match byte {
            b'0' => Some(Self::Zero),
            b'1' => Some(Self::One),
            b'x' | b'X' => Some(Self::Unknown),
            b'z' | b'Z' => Some(Self::HighImpedance),
            _ => None,
        }
    }

    /// The bit a shorter vector is left-extended with, per IEEE 1364: a value
    /// whose leading bit is `0` or `1` extends with `0`, one whose leading bit
    /// is `x` or `z` extends with itself.
    const fn extension(self) -> Self {
        match self {
            Self::Zero | Self::One => Self::Zero,
            Self::Unknown => Self::Unknown,
            Self::HighImpedance => Self::HighImpedance,
        }
    }
}

impl fmt::Display for VcdBit {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.as_char())
    }
}

/// The value a signal takes at one tick.
#[derive(Debug, Clone, PartialEq)]
pub enum VcdValue {
    /// Logic bits, most significant first, exactly as many as the signal is
    /// wide: the reader left-extends a short vector before storing it.
    Logic(Vec<VcdBit>),
    /// A finite real value.
    Real(f64),
}

/// One value change: when, and to what.
#[derive(Debug, Clone, PartialEq)]
pub struct VcdChange {
    /// Time in ticks of the document's [`VcdTimescale`].
    pub tick: u64,
    /// The value the signal holds from this tick until its next change.
    pub value: VcdValue,
}

/// One `$var` declaration: a name and the scopes enclosing it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VcdVariable {
    /// Enclosing `$scope` names, outermost first.
    pub scope: Vec<String>,
    /// The declared name, including any bit-select suffix such as `[3:0]`.
    pub name: String,
}

impl VcdVariable {
    /// The dotted name a waveform viewer shows: `scope.path.name`.
    pub fn scoped_name(&self) -> String {
        let mut scoped = String::new();
        for level in &self.scope {
            scoped.push_str(level);
            scoped.push('.');
        }
        scoped.push_str(&self.name);
        scoped
    }
}

/// One identifier code and the timeline it carries.
///
/// Aliases are why `variables` is a list: two `$var` declarations may share an
/// identifier code, which makes them two names for one timeline. They are kept
/// as declared rather than flattened into duplicate signals, because a viewer
/// showing both should still be able to say they are the same wire.
#[derive(Debug, Clone, PartialEq)]
pub struct VcdSignal {
    /// The identifier code the file used. [`write_vcd`] regenerates these, so
    /// this is what was read rather than what will be written.
    pub identifier: String,
    /// Every declared name for this timeline; never empty.
    pub variables: Vec<VcdVariable>,
    /// Declared bit width. Real variables conventionally declare 64.
    pub width: u32,
    /// Whether the changes carry bits or reals.
    pub kind: VcdSignalKind,
    /// Value changes, in tick order.
    pub changes: Vec<VcdChange>,
}

/// A whole VCD file: its header, its variables, and their timelines.
#[derive(Debug, Clone, PartialEq)]
pub struct VcdDocument {
    /// `$date`, verbatim. Empty when the file declared none.
    ///
    /// The caller supplies this rather than the writer reading a clock: core
    /// runs on `wasm32`, which has none, and a produced file should be a
    /// function of the data so that two runs of one deck compare equal.
    pub date: String,
    /// `$version`, verbatim. [`VCD_WRITER_VERSION`] on documents RSpice built.
    pub version: String,
    /// `$comment` blocks, in file order.
    pub comments: Vec<String>,
    /// The duration of one tick.
    pub timescale: VcdTimescale,
    /// Signals, in declaration order.
    pub signals: Vec<VcdSignal>,
}

impl VcdDocument {
    /// An empty document at `timescale`, stamped with [`VCD_WRITER_VERSION`].
    pub fn new(timescale: VcdTimescale) -> Self {
        Self {
            date: String::new(),
            version: VCD_WRITER_VERSION.to_string(),
            comments: Vec::new(),
            timescale,
            signals: Vec::new(),
        }
    }

    /// Stamp each signal with the identifier code [`write_vcd`] will give it.
    ///
    /// The writer does not consult [`VcdSignal::identifier`], so this changes
    /// no output. It makes a document built in memory equal to the document
    /// read back from its own bytes, which is the property a round trip is
    /// worth asserting on.
    pub fn assign_canonical_identifiers(&mut self) {
        for (index, signal) in self.signals.iter_mut().enumerate() {
            signal.identifier = canonical_identifier(index);
        }
    }
}

// -------------------------------------------------------------------------
// Reading

/// Parse a VCD file from a path.
pub fn parse_vcd_file(path: &Path) -> Result<VcdDocument, VcdError> {
    parse_vcd_file_with_limits(path, ResourceLimits::default())
}

/// Parse a VCD file with explicit external-data and retained-result limits.
pub fn parse_vcd_file_with_limits(
    path: &Path,
    resource_limits: ResourceLimits,
) -> Result<VcdDocument, VcdError> {
    let bytes = read_file_bytes_limited(
        path,
        ResourceKind::ExternalDataBytes,
        resource_limits.max_external_data_bytes,
    )?;
    parse_vcd_bytes(&bytes, resource_limits)
}

/// Parse VCD data from a reader.
pub fn parse_vcd_reader<R: Read>(reader: R) -> Result<VcdDocument, VcdError> {
    parse_vcd_reader_with_limits(reader, ResourceLimits::default())
}

/// Parse VCD data from a reader with explicit resource limits.
///
/// The limits bound the work before it is done: the byte budget caps the
/// input, the external-value budget caps declarations and value changes, and
/// the result-value budget caps the bits and reals the document retains.
pub fn parse_vcd_reader_with_limits<R: Read>(
    reader: R,
    resource_limits: ResourceLimits,
) -> Result<VcdDocument, VcdError> {
    let bytes = read_bytes_limited(
        reader,
        ResourceKind::ExternalDataBytes,
        resource_limits.max_external_data_bytes,
    )?;
    parse_vcd_bytes(&bytes, resource_limits)
}

fn parse_vcd_bytes(bytes: &[u8], resource_limits: ResourceLimits) -> Result<VcdDocument, VcdError> {
    parse_vcd_text(std::str::from_utf8(bytes)?, resource_limits)
}

/// Whitespace-delimited tokens, each with the line it started on.
///
/// VCD is a token stream, not a line format: a directive may sit on one line
/// or be spread over several, and both spellings are common. Tokenizing once
/// removes the line-joining special cases and still knows where a bad token
/// was.
struct Tokens<'a> {
    rest: &'a str,
    line: usize,
}

impl<'a> Tokens<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            rest: text,
            line: 1,
        }
    }

    fn advance(&mut self) -> Option<(&'a str, usize)> {
        let trimmed = self.rest.trim_start();
        let skipped = self.rest.len() - trimmed.len();
        self.line += self.rest.as_bytes()[..skipped]
            .iter()
            .filter(|byte| **byte == b'\n')
            .count();
        self.rest = trimmed;
        if self.rest.is_empty() {
            return None;
        }
        let line = self.line;
        let end = self
            .rest
            .find(char::is_whitespace)
            .unwrap_or(self.rest.len());
        let (token, rest) = self.rest.split_at(end);
        self.rest = rest;
        Some((token, line))
    }
}

fn syntax(line: usize, message: impl Into<String>) -> VcdError {
    VcdError::Syntax {
        line,
        message: message.into(),
    }
}

fn reserve_failure(what: &str, error: std::collections::TryReserveError) -> VcdError {
    VcdError::Structure(format!("unable to allocate {what}: {error}"))
}

/// Consume a directive's body up to its `$end`.
fn block_tokens<'a>(
    tokens: &mut Tokens<'a>,
    keyword: &str,
    line: usize,
) -> Result<Vec<&'a str>, VcdError> {
    let mut body: Vec<&str> = Vec::new();
    loop {
        let Some((token, _)) = tokens.advance() else {
            return Err(syntax(
                line,
                format!("${keyword} is not terminated by $end"),
            ));
        };
        if token == "$end" {
            return Ok(body);
        }
        body.try_reserve(1)
            .map_err(|error| reserve_failure("a VCD directive body", error))?;
        body.push(token);
    }
}

fn block_text(tokens: &mut Tokens<'_>, keyword: &str, line: usize) -> Result<String, VcdError> {
    Ok(block_tokens(tokens, keyword, line)?.join(" "))
}

fn expect_empty_block(tokens: &mut Tokens<'_>, keyword: &str, line: usize) -> Result<(), VcdError> {
    if block_tokens(tokens, keyword, line)?.is_empty() {
        Ok(())
    } else {
        Err(syntax(line, format!("${keyword} takes no arguments")))
    }
}

fn parse_timescale(body: &str, line: usize) -> Result<VcdTimescale, VcdError> {
    let compact: String = body.split_whitespace().collect();
    let split = compact
        .find(|character: char| !character.is_ascii_digit())
        .ok_or_else(|| syntax(line, format!("$timescale '{body}' names no unit")))?;
    let (magnitude_text, unit_text) = compact.split_at(split);
    let magnitude = match magnitude_text.parse::<u32>() {
        Ok(1) => VcdMagnitude::One,
        Ok(10) => VcdMagnitude::Ten,
        Ok(100) => VcdMagnitude::Hundred,
        _ => {
            return Err(syntax(
                line,
                format!("$timescale magnitude '{magnitude_text}' must be 1, 10 or 100"),
            ));
        }
    };
    let unit = match unit_text.to_ascii_lowercase().as_str() {
        "s" => VcdTimeUnit::Seconds,
        "ms" => VcdTimeUnit::Milliseconds,
        "us" => VcdTimeUnit::Microseconds,
        "ns" => VcdTimeUnit::Nanoseconds,
        "ps" => VcdTimeUnit::Picoseconds,
        "fs" => VcdTimeUnit::Femtoseconds,
        other => {
            return Err(syntax(
                line,
                format!("unsupported $timescale unit '{other}'"),
            ));
        }
    };
    Ok(VcdTimescale { magnitude, unit })
}

/// Read a `b`-prefixed vector, left-extended to the declared width.
fn parse_bits(text: &str, width: u32, line: usize) -> Result<Vec<VcdBit>, VcdError> {
    let width = width as usize;
    if text.is_empty() {
        return Err(syntax(line, "vector value change carries no bits"));
    }
    if text.len() > width {
        return Err(syntax(
            line,
            format!(
                "vector value change is {} bits wide, but the variable declares {width}",
                text.len()
            ),
        ));
    }
    let mut parsed: Vec<VcdBit> = Vec::new();
    parsed
        .try_reserve_exact(text.len())
        .map_err(|error| reserve_failure("a VCD vector value", error))?;
    for byte in text.bytes() {
        let bit = VcdBit::from_byte(byte)
            .ok_or_else(|| syntax(line, format!("'{}' is not a VCD bit state", byte as char)))?;
        parsed.push(bit);
    }
    let Some(leading) = parsed.first().copied() else {
        return Err(syntax(line, "vector value change carries no bits"));
    };
    if parsed.len() == width {
        return Ok(parsed);
    }
    let mut bits: Vec<VcdBit> = Vec::new();
    bits.try_reserve_exact(width)
        .map_err(|error| reserve_failure("a VCD vector value", error))?;
    bits.resize(width - parsed.len(), leading.extension());
    bits.append(&mut parsed);
    Ok(bits)
}

/// A scalar change is a one-bit vector, extended by the same rule.
fn scalar_bits(bit: VcdBit, width: u32) -> Result<Vec<VcdBit>, VcdError> {
    let width = width as usize;
    let mut bits: Vec<VcdBit> = Vec::new();
    bits.try_reserve_exact(width)
        .map_err(|error| reserve_failure("a VCD scalar value", error))?;
    bits.resize(width, bit.extension());
    if let Some(last) = bits.last_mut() {
        *last = bit;
    }
    Ok(bits)
}

/// The signals read so far, and the resource budgets they have spent.
struct ReaderState {
    signals: Vec<VcdSignal>,
    by_identifier: HashMap<String, usize>,
    structural_items: usize,
    retained_values: usize,
}

impl ReaderState {
    fn charge_structural(&mut self, limits: ResourceLimits) -> Result<(), VcdError> {
        self.structural_items = self.structural_items.saturating_add(1);
        ResourceLimitError::ensure(
            ResourceKind::ExternalDataValues,
            self.structural_items,
            limits.max_external_data_values,
        )?;
        Ok(())
    }

    fn charge_retained(&mut self, values: usize, limits: ResourceLimits) -> Result<(), VcdError> {
        self.retained_values = self.retained_values.saturating_add(values);
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            self.retained_values,
            limits.max_result_values,
        )?;
        Ok(())
    }
}

fn parse_vcd_text(text: &str, limits: ResourceLimits) -> Result<VcdDocument, VcdError> {
    let mut tokens = Tokens::new(text);
    let mut date = String::new();
    let mut version = String::new();
    let mut comments: Vec<String> = Vec::new();
    let mut timescale: Option<VcdTimescale> = None;
    let mut scopes: Vec<String> = Vec::new();
    let mut state = ReaderState {
        signals: Vec::new(),
        by_identifier: HashMap::new(),
        structural_items: 0,
        retained_values: 0,
    };
    let mut definitions = true;
    let mut dumping = false;
    let mut tick = 0_u64;

    while let Some((token, line)) = tokens.advance() {
        if let Some(keyword) = token.strip_prefix('$') {
            match keyword {
                "date" => date = block_text(&mut tokens, keyword, line)?,
                "version" => version = block_text(&mut tokens, keyword, line)?,
                "comment" => {
                    let comment = block_text(&mut tokens, keyword, line)?;
                    comments
                        .try_reserve(1)
                        .map_err(|error| reserve_failure("a VCD comment", error))?;
                    comments.push(comment);
                }
                "timescale" => {
                    let body = block_text(&mut tokens, keyword, line)?;
                    if timescale.replace(parse_timescale(&body, line)?).is_some() {
                        return Err(syntax(line, "$timescale is declared more than once"));
                    }
                }
                "scope" => {
                    let body = block_tokens(&mut tokens, keyword, line)?;
                    let Some(name) = body.get(1) else {
                        return Err(syntax(line, "$scope names no type and instance"));
                    };
                    state.charge_structural(limits)?;
                    scopes
                        .try_reserve(1)
                        .map_err(|error| reserve_failure("a VCD scope", error))?;
                    scopes.push((*name).to_string());
                }
                "upscope" => {
                    expect_empty_block(&mut tokens, keyword, line)?;
                    if scopes.pop().is_none() {
                        return Err(syntax(line, "$upscope leaves no enclosing scope"));
                    }
                }
                "var" => declare_variable(&mut tokens, &mut state, &scopes, limits, line)?,
                "enddefinitions" => {
                    expect_empty_block(&mut tokens, keyword, line)?;
                    definitions = false;
                }
                "dumpvars" | "dumpon" | "dumpoff" | "dumpall" => {
                    if definitions {
                        return Err(syntax(
                            line,
                            format!("${keyword} appears before $enddefinitions"),
                        ));
                    }
                    dumping = true;
                }
                "end" => {
                    if !dumping {
                        return Err(syntax(line, "$end closes no directive"));
                    }
                    dumping = false;
                }
                _ => {
                    block_tokens(&mut tokens, keyword, line)?;
                }
            }
            continue;
        }

        if let Some(digits) = token.strip_prefix('#') {
            if definitions {
                return Err(syntax(line, "time stamp appears before $enddefinitions"));
            }
            let next = digits
                .parse::<u64>()
                .map_err(|_| syntax(line, format!("'{token}' is not a time stamp")))?;
            if next < tick {
                return Err(syntax(
                    line,
                    format!("time stamp #{next} moves back from #{tick}"),
                ));
            }
            tick = next;
            continue;
        }

        if definitions {
            return Err(syntax(line, format!("'{token}' is not a VCD declaration")));
        }
        record_change(&mut tokens, &mut state, token, tick, limits, line)?;
    }

    if definitions {
        return Err(VcdError::Structure(
            "VCD is missing $enddefinitions".to_string(),
        ));
    }
    let Some(timescale) = timescale else {
        return Err(VcdError::Structure("VCD is missing $timescale".to_string()));
    };
    if let Some(open) = scopes.last() {
        return Err(VcdError::Structure(format!(
            "VCD ends inside scope '{open}'"
        )));
    }
    if dumping {
        return Err(VcdError::Structure(
            "VCD ends inside a value-dump block".to_string(),
        ));
    }

    Ok(VcdDocument {
        date,
        version,
        comments,
        timescale,
        signals: state.signals,
    })
}

fn declare_variable(
    tokens: &mut Tokens<'_>,
    state: &mut ReaderState,
    scopes: &[String],
    limits: ResourceLimits,
    line: usize,
) -> Result<(), VcdError> {
    let body = block_tokens(tokens, "var", line)?;
    let (Some(var_type), Some(width_text), Some(identifier), Some(name_fields)) =
        (body.first(), body.get(1), body.get(2), body.get(3..))
    else {
        return Err(syntax(
            line,
            "$var needs a type, a width, an identifier and a name",
        ));
    };
    if name_fields.is_empty() {
        return Err(syntax(line, "$var names no variable"));
    }
    let name = name_fields.join(" ");
    let width = width_text
        .parse::<u32>()
        .map_err(|_| syntax(line, format!("'{width_text}' is not a $var width")))?;
    if width == 0 {
        return Err(syntax(line, format!("variable '{name}' declares width 0")));
    }
    // One change on this variable materializes `width` bits, so the width is
    // itself a value budget and is checked before anything reserves on it.
    ResourceLimitError::ensure(
        ResourceKind::ExternalDataValues,
        width as usize,
        limits.max_external_data_values,
    )?;
    let kind = match var_type.to_ascii_lowercase().as_str() {
        "real" | "realtime" => VcdSignalKind::Real,
        _ => VcdSignalKind::Logic,
    };

    state.charge_structural(limits)?;
    let mut scope: Vec<String> = Vec::new();
    scope
        .try_reserve_exact(scopes.len())
        .map_err(|error| reserve_failure("a VCD scope path", error))?;
    scope.extend(scopes.iter().cloned());
    let variable = VcdVariable { scope, name };

    if let Some(existing) = state.by_identifier.get(*identifier).copied() {
        let signal = state
            .signals
            .get_mut(existing)
            .ok_or_else(|| syntax(line, format!("alias '{identifier}' names no signal")))?;
        if signal.width != width {
            return Err(syntax(
                line,
                format!(
                    "alias '{}' declares width {width}, but identifier '{identifier}' is {} wide",
                    variable.name, signal.width
                ),
            ));
        }
        if signal.kind != kind {
            return Err(syntax(
                line,
                format!(
                    "alias '{}' changes the kind of identifier '{identifier}'",
                    variable.name
                ),
            ));
        }
        signal
            .variables
            .try_reserve(1)
            .map_err(|error| reserve_failure("a VCD alias", error))?;
        signal.variables.push(variable);
        return Ok(());
    }

    state
        .signals
        .try_reserve(1)
        .map_err(|error| reserve_failure("a VCD signal", error))?;
    state
        .by_identifier
        .try_reserve(1)
        .map_err(|error| reserve_failure("a VCD identifier table", error))?;
    state
        .by_identifier
        .insert((*identifier).to_string(), state.signals.len());
    state.signals.push(VcdSignal {
        identifier: (*identifier).to_string(),
        variables: vec![variable],
        width,
        kind,
        changes: Vec::new(),
    });
    Ok(())
}

/// A value change's payload, before the declared width is known.
enum Payload<'a> {
    Scalar(VcdBit),
    Vector(&'a str),
    Real(&'a str),
}

fn record_change(
    tokens: &mut Tokens<'_>,
    state: &mut ReaderState,
    token: &str,
    tick: u64,
    limits: ResourceLimits,
    line: usize,
) -> Result<(), VcdError> {
    let (payload, identifier) = match token.chars().next() {
        Some('0' | '1' | 'x' | 'X' | 'z' | 'Z') => {
            let (level, identifier) = token.split_at(1);
            let bit = level
                .bytes()
                .next()
                .and_then(VcdBit::from_byte)
                .ok_or_else(|| syntax(line, format!("'{token}' is not a value change")))?;
            if identifier.is_empty() {
                return Err(syntax(line, "value change names no identifier"));
            }
            (Payload::Scalar(bit), identifier)
        }
        Some('b' | 'B') => {
            let (_, bits) = token.split_at(1);
            let Some((identifier, _)) = tokens.advance() else {
                return Err(syntax(line, "vector value change names no identifier"));
            };
            (Payload::Vector(bits), identifier)
        }
        Some('r' | 'R') => {
            let (_, real) = token.split_at(1);
            let Some((identifier, _)) = tokens.advance() else {
                return Err(syntax(line, "real value change names no identifier"));
            };
            (Payload::Real(real), identifier)
        }
        _ => return Err(syntax(line, format!("'{token}' is not a value change"))),
    };

    let index = state
        .by_identifier
        .get(identifier)
        .copied()
        .ok_or_else(|| syntax(line, format!("unknown identifier '{identifier}'")))?;
    let Some((width, kind)) = state
        .signals
        .get(index)
        .map(|signal| (signal.width, signal.kind))
    else {
        return Err(syntax(line, format!("unknown identifier '{identifier}'")));
    };

    // Refuse a mismatch, then charge the budget, then build the value: the
    // limits have to bound the allocation, not report it afterwards.
    match (&payload, kind) {
        (Payload::Real(_), VcdSignalKind::Logic) => {
            return Err(syntax(
                line,
                format!("identifier '{identifier}' is a logic variable, not a real one"),
            ));
        }
        (Payload::Scalar(_) | Payload::Vector(_), VcdSignalKind::Real) => {
            return Err(syntax(
                line,
                format!("identifier '{identifier}' is a real variable, not a logic one"),
            ));
        }
        _ => {}
    }
    state.charge_structural(limits)?;
    state.charge_retained(
        match kind {
            VcdSignalKind::Logic => width as usize,
            VcdSignalKind::Real => 1,
        },
        limits,
    )?;

    let value = match payload {
        Payload::Scalar(bit) => VcdValue::Logic(scalar_bits(bit, width)?),
        Payload::Vector(bits) => VcdValue::Logic(parse_bits(bits, width, line)?),
        Payload::Real(text) => {
            let real = text
                .parse::<f64>()
                .map_err(|_| syntax(line, format!("'{text}' is not a real value")))?;
            if !real.is_finite() {
                return Err(syntax(line, format!("real value '{text}' is not finite")));
            }
            VcdValue::Real(real)
        }
    };

    let signal = state
        .signals
        .get_mut(index)
        .ok_or_else(|| syntax(line, format!("unknown identifier '{identifier}'")))?;
    signal
        .changes
        .try_reserve(1)
        .map_err(|error| reserve_failure("a VCD value change", error))?;
    signal.changes.push(VcdChange { tick, value });
    Ok(())
}

// -------------------------------------------------------------------------
// Writing

/// Serialize a document as VCD.
///
/// Identifier codes are regenerated in the standard printable-ASCII scheme
/// (`!`, `"`, … `~`, `!!`, …) in signal order, so the bytes depend only on the
/// data. Logic variables are declared `wire` and scopes `module`; the module
/// documentation says why the declared types are not carried.
///
/// The whole document is validated before a byte is written, so a document VCD
/// cannot express fails without leaving a partial file behind.
pub fn write_vcd(writer: impl Write, document: &VcdDocument) -> Result<(), VcdError> {
    validate_writable(document)?;
    let mut writer = writer;

    write_block(&mut writer, "date", &document.date)?;
    write_block(&mut writer, "version", &document.version)?;
    for comment in &document.comments {
        write_block(&mut writer, "comment", comment)?;
    }
    write_block(&mut writer, "timescale", &document.timescale.to_string())?;

    let mut identifiers: Vec<String> = Vec::new();
    identifiers
        .try_reserve_exact(document.signals.len())
        .map_err(|error| reserve_failure("VCD identifier codes", error))?;
    for index in 0..document.signals.len() {
        identifiers.push(canonical_identifier(index));
    }

    let mut open: Vec<&str> = Vec::new();
    for (signal, identifier) in document.signals.iter().zip(identifiers.iter()) {
        let keyword = match signal.kind {
            VcdSignalKind::Logic => "wire",
            VcdSignalKind::Real => "real",
        };
        for variable in &signal.variables {
            retarget_scope(&mut writer, &mut open, &variable.scope)?;
            writeln!(
                writer,
                "$var {keyword} {} {identifier} {} $end",
                signal.width, variable.name
            )?;
        }
    }
    retarget_scope(&mut writer, &mut open, &[])?;
    writeln!(writer, "$enddefinitions $end")?;

    write_changes(&mut writer, document, &identifiers)
}

fn write_block(writer: &mut impl Write, keyword: &str, body: &str) -> Result<(), VcdError> {
    if body.is_empty() {
        writeln!(writer, "${keyword}\n$end")?;
    } else {
        writeln!(writer, "${keyword}\n\t{body}\n$end")?;
    }
    Ok(())
}

fn retarget_scope<'a>(
    writer: &mut impl Write,
    open: &mut Vec<&'a str>,
    target: &'a [String],
) -> Result<(), VcdError> {
    let shared = open
        .iter()
        .zip(target.iter())
        .take_while(|(current, wanted)| *current == wanted)
        .count();
    while open.len() > shared {
        open.pop();
        writeln!(writer, "$upscope $end")?;
    }
    for level in target.iter().skip(shared) {
        open.push(level.as_str());
        writeln!(writer, "$scope module {level} $end")?;
    }
    Ok(())
}

fn write_changes(
    writer: &mut impl Write,
    document: &VcdDocument,
    identifiers: &[String],
) -> Result<(), VcdError> {
    let total: usize = document
        .signals
        .iter()
        .map(|signal| signal.changes.len())
        .sum();
    let mut ordered: Vec<(u64, usize, usize)> = Vec::new();
    ordered
        .try_reserve_exact(total)
        .map_err(|error| reserve_failure("the VCD change order", error))?;
    for (signal_index, signal) in document.signals.iter().enumerate() {
        for (change_index, change) in signal.changes.iter().enumerate() {
            ordered.push((change.tick, signal_index, change_index));
        }
    }
    ordered.sort_unstable();

    for (group_index, group) in ordered
        .chunk_by(|left, right| left.0 == right.0)
        .enumerate()
    {
        let Some((tick, _, _)) = group.first() else {
            continue;
        };
        writeln!(writer, "#{tick}")?;
        if group_index == 0 {
            writeln!(writer, "$dumpvars")?;
        }
        for (_, signal_index, change_index) in group {
            let (Some(signal), Some(identifier)) = (
                document.signals.get(*signal_index),
                identifiers.get(*signal_index),
            ) else {
                return Err(unwritable("a value change names no signal"));
            };
            let Some(change) = signal.changes.get(*change_index) else {
                return Err(unwritable("a value change names no time"));
            };
            match &change.value {
                VcdValue::Logic(bits) => match bits.as_slice() {
                    [bit] => writeln!(writer, "{bit}{identifier}")?,
                    bits => {
                        write!(writer, "b")?;
                        for bit in bits {
                            write!(writer, "{bit}")?;
                        }
                        writeln!(writer, " {identifier}")?;
                    }
                },
                VcdValue::Real(real) => writeln!(writer, "r{real:?} {identifier}")?,
            }
        }
        if group_index == 0 {
            writeln!(writer, "$end")?;
        }
    }
    Ok(())
}

/// The identifier code the writer gives the signal at `index`.
///
/// A bijection onto the non-empty strings over printable ASCII 33..=126, so no
/// two signals collide however many there are.
fn canonical_identifier(mut index: usize) -> String {
    const ALPHABET: usize = 94;
    let mut identifier = String::new();
    loop {
        let digit = (index % ALPHABET) as u8;
        identifier.push(char::from(b'!' + digit));
        if index < ALPHABET {
            return identifier;
        }
        index = index / ALPHABET - 1;
    }
}

fn unwritable(message: impl Into<String>) -> VcdError {
    VcdError::Unwritable(message.into())
}

/// Whether a `$var` name survives the round trip through the token grammar.
pub(crate) fn is_writable_variable_name(name: &str) -> bool {
    !name.is_empty()
        && name.split_whitespace().collect::<Vec<_>>().join(" ") == name
        && !name.split_whitespace().any(|token| token == "$end")
}

/// Whether a `$scope` name survives it. A scope name is one token.
pub(crate) fn is_writable_scope_name(name: &str) -> bool {
    let mut tokens = name.split_whitespace();
    tokens.next() == Some(name) && tokens.next().is_none() && name != "$end"
}

fn signal_label(signal: &VcdSignal) -> String {
    signal
        .variables
        .first()
        .map_or_else(|| signal.identifier.clone(), VcdVariable::scoped_name)
}

fn validate_writable(document: &VcdDocument) -> Result<(), VcdError> {
    for text in [&document.date, &document.version]
        .into_iter()
        .chain(document.comments.iter())
    {
        if text.split_whitespace().any(|token| token == "$end") {
            return Err(unwritable("a header block contains the token '$end'"));
        }
    }

    for signal in &document.signals {
        let label = signal_label(signal);
        if signal.width == 0 {
            return Err(unwritable(format!("'{label}' declares width 0")));
        }
        if signal.variables.is_empty() {
            return Err(unwritable("a signal declares no variable name"));
        }
        for variable in &signal.variables {
            if !is_writable_variable_name(&variable.name) {
                return Err(unwritable(format!(
                    "'{}' is not a writable $var name",
                    variable.name
                )));
            }
            for level in &variable.scope {
                if !is_writable_scope_name(level) {
                    return Err(unwritable(format!(
                        "'{level}' is not a writable scope name"
                    )));
                }
            }
        }
        let mut previous: Option<u64> = None;
        for change in &signal.changes {
            if previous.is_some_and(|earlier| change.tick < earlier) {
                return Err(unwritable(format!(
                    "'{label}' changes at tick {} after a later one",
                    change.tick
                )));
            }
            previous = Some(change.tick);
            match (&change.value, signal.kind) {
                (VcdValue::Logic(bits), VcdSignalKind::Logic) => {
                    if bits.len() != signal.width as usize {
                        return Err(unwritable(format!(
                            "a change on '{label}' carries {} bits for a {}-bit variable",
                            bits.len(),
                            signal.width
                        )));
                    }
                }
                (VcdValue::Real(real), VcdSignalKind::Real) => {
                    if !real.is_finite() {
                        return Err(unwritable(format!("a change on '{label}' is not finite")));
                    }
                }
                _ => {
                    return Err(unwritable(format!(
                        "a change on '{label}' does not match its declared kind"
                    )));
                }
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The fixture the GUI's VCD adapter is pinned against, byte for byte
    /// (`result_import_adapters/tests.rs`).
    const UI_FIXTURE: &str = "$timescale 1 ns $end\n\
        $scope module top $end\n\
        $var wire 1 ! clk $end\n\
        $var wire 2 \" bus $end\n\
        $upscope $end\n\
        $enddefinitions $end\n\
        #0\n0!\nb00 \"\n#5\n1!\nb01 \"\n#10\n0!\nb10 \"\n";

    fn parse(text: &str) -> Result<VcdDocument, VcdError> {
        parse_vcd_reader(text.as_bytes())
    }

    fn bits(text: &str) -> VcdValue {
        VcdValue::Logic(
            text.bytes()
                .map(|byte| VcdBit::from_byte(byte).expect("bit state"))
                .collect(),
        )
    }

    fn ticks(signal: &VcdSignal) -> Vec<u64> {
        signal.changes.iter().map(|change| change.tick).collect()
    }

    fn values(signal: &VcdSignal) -> Vec<VcdValue> {
        signal
            .changes
            .iter()
            .map(|change| change.value.clone())
            .collect()
    }

    fn names(document: &VcdDocument) -> Vec<String> {
        document
            .signals
            .iter()
            .flat_map(|signal| signal.variables.iter().map(VcdVariable::scoped_name))
            .collect()
    }

    fn rendered(document: &VcdDocument) -> String {
        let mut bytes = Vec::new();
        write_vcd(&mut bytes, document).expect("write");
        String::from_utf8(bytes).expect("UTF-8")
    }

    fn rendered_error(document: &VcdDocument) -> String {
        let mut bytes = Vec::new();
        let error = write_vcd(&mut bytes, document).expect_err("unwritable");
        assert!(bytes.is_empty(), "a refused document wrote bytes");
        error.to_string()
    }

    #[test]
    fn the_gui_fixture_yields_the_same_signals_and_ticks() {
        let document = parse(UI_FIXTURE).expect("fixture");
        assert_eq!(document.timescale.to_string(), "1 ns");
        assert_eq!(document.timescale.seconds(), 1e-9);
        assert_eq!(names(&document), vec!["top.clk", "top.bus"]);

        let clk = document.signals.first().expect("clk");
        assert_eq!(clk.width, 1);
        assert_eq!(clk.kind, VcdSignalKind::Logic);
        assert_eq!(ticks(clk), vec![0, 5, 10]);
        assert_eq!(values(clk), vec![bits("0"), bits("1"), bits("0")]);

        let bus = document.signals.get(1).expect("bus");
        assert_eq!(bus.width, 2);
        assert_eq!(ticks(bus), vec![0, 5, 10]);
        assert_eq!(values(bus), vec![bits("00"), bits("01"), bits("10")]);
    }

    #[test]
    fn four_state_scalars_and_vector_bits_parse() {
        // The case the GUI's f64-mapping adapter refuses outright.
        let document = parse(
            "$timescale 1 ns $end\n\
             $var wire 1 ! a $end\n\
             $var wire 4 \" b $end\n\
             $enddefinitions $end\n\
             #0\nx!\nbxz01 \"\n#1\nZ!\nbXXzz \"\n",
        )
        .expect("four-state");
        let scalar = document.signals.first().expect("a");
        assert_eq!(
            values(scalar),
            vec![
                VcdValue::Logic(vec![VcdBit::Unknown]),
                VcdValue::Logic(vec![VcdBit::HighImpedance]),
            ]
        );
        let vector = document.signals.get(1).expect("b");
        assert_eq!(values(vector), vec![bits("xz01"), bits("xxzz")]);
    }

    #[test]
    fn short_vectors_are_left_extended_by_their_leading_bit() {
        let document = parse(
            "$timescale 1 ns $end\n\
             $var wire 4 ! a $end\n\
             $enddefinitions $end\n\
             #0\nb1 !\n#1\nbx !\n#2\nbz !\n#3\nb0 !\n#4\nb01 !\n",
        )
        .expect("extension");
        let signal = document.signals.first().expect("a");
        assert_eq!(
            values(signal),
            vec![
                bits("0001"),
                bits("xxxx"),
                bits("zzzz"),
                bits("0000"),
                bits("0001"),
            ]
        );
        // A scalar on a wide variable extends by the same rule.
        let scalar = parse(
            "$timescale 1 ns $end\n$var wire 3 ! a $end\n$enddefinitions $end\n#0\nz!\n#1\n1!\n",
        )
        .expect("scalar extension");
        assert_eq!(
            values(scalar.signals.first().expect("a")),
            vec![bits("zzz"), bits("001")]
        );
    }

    #[test]
    fn a_vector_wider_than_its_variable_is_refused() {
        let error = parse(
            "$timescale 1 ns $end\n$var wire 2 ! a $end\n$enddefinitions $end\n#0\nb1010 !\n",
        )
        .expect_err("too wide");
        assert!(error.to_string().contains("declares 2"), "{error}");
    }

    #[test]
    fn aliases_share_one_timeline_and_must_agree() {
        let document = parse(
            "$timescale 1 ns $end\n\
             $scope module top $end\n\
             $var wire 2 ! bus $end\n\
             $var wire 2 ! mirror $end\n\
             $upscope $end\n\
             $enddefinitions $end\n\
             #0\nb01 !\n",
        )
        .expect("alias");
        assert_eq!(document.signals.len(), 1);
        assert_eq!(names(&document), vec!["top.bus", "top.mirror"]);
        assert_eq!(
            values(document.signals.first().expect("bus")),
            vec![bits("01")]
        );

        let width = parse(
            "$timescale 1 ns $end\n\
             $var wire 2 ! bus $end\n$var wire 3 ! mirror $end\n$enddefinitions $end\n",
        )
        .expect_err("alias width");
        assert!(width.to_string().contains("declares width 3"), "{width}");

        let kind = parse(
            "$timescale 1 ns $end\n\
             $var wire 64 ! bus $end\n$var real 64 ! mirror $end\n$enddefinitions $end\n",
        )
        .expect_err("alias kind");
        assert!(kind.to_string().contains("changes the kind"), "{kind}");
    }

    #[test]
    fn dump_blocks_carry_ordinary_value_changes() {
        let document = parse(
            "$timescale 1 ps $end\n\
             $var wire 1 ! a $end\n\
             $var real 64 \" b $end\n\
             $enddefinitions $end\n\
             #0\n$dumpvars\n0!\nr0.5 \"\n$end\n\
             #10\n$dumpoff\nx!\n$end\n\
             #20\n$dumpon\n1!\nr1.5 \"\n$end\n\
             #30\n$dumpall\n0!\n$end\n",
        )
        .expect("dump blocks");
        let logic = document.signals.first().expect("a");
        assert_eq!(ticks(logic), vec![0, 10, 20, 30]);
        assert_eq!(
            values(logic),
            vec![bits("0"), bits("x"), bits("1"), bits("0")]
        );
        let real = document.signals.get(1).expect("b");
        assert_eq!(ticks(real), vec![0, 20]);
        assert_eq!(values(real), vec![VcdValue::Real(0.5), VcdValue::Real(1.5)]);
    }

    #[test]
    fn timescale_spellings_and_every_unit_parse() {
        let one_line = parse("$timescale 1 ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n")
            .expect("one line");
        let multi_line =
            parse("$timescale\n  1 ns\n$end\n$var wire 1 ! a $end\n$enddefinitions $end\n")
                .expect("multi line");
        let joined = parse("$timescale 1ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n")
            .expect("joined");
        assert_eq!(one_line.timescale, multi_line.timescale);
        assert_eq!(one_line.timescale, joined.timescale);

        for expected in VcdTimescale::ALL {
            let text =
                format!("$timescale {expected} $end\n$var wire 1 ! a $end\n$enddefinitions $end\n");
            let document = parse(&text).expect("timescale");
            assert_eq!(document.timescale, expected, "{expected}");
            // Upper-case units are accepted, as in the GUI adapter.
            let upper = format!(
                "$timescale {} {} $end\n$var wire 1 ! a $end\n$enddefinitions $end\n",
                expected.magnitude,
                expected.unit.as_str().to_uppercase()
            );
            assert_eq!(parse(&upper).expect("upper").timescale, expected);
        }

        assert!(parse("$timescale 2 ns $end\n$enddefinitions $end\n").is_err());
        assert!(parse("$timescale 1 min $end\n$enddefinitions $end\n").is_err());
        assert!(
            parse("$timescale 1 ns $end\n$timescale 1 ps $end\n$enddefinitions $end\n")
                .expect_err("twice")
                .to_string()
                .contains("more than once")
        );
    }

    #[test]
    fn timescale_periods_are_exact_and_ordered() {
        let mut previous: Option<u64> = None;
        for timescale in VcdTimescale::ALL {
            let period = timescale.femtoseconds();
            assert!(
                previous.is_none_or(|earlier| earlier > period),
                "{timescale} is not finer than the entry before it"
            );
            previous = Some(period);
            assert!(
                (timescale.seconds() - period as f64 * 1e-15).abs() < timescale.seconds() * 1e-9
            );
        }
        assert_eq!(previous, Some(1));
    }

    #[test]
    fn a_time_stamp_may_not_move_backwards() {
        let error = parse(
            "$timescale 1 ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n#5\n0!\n#4\n1!\n",
        )
        .expect_err("backwards");
        assert!(error.to_string().contains("moves back from #5"), "{error}");
        // Repeating a time stamp is legal.
        assert!(
            parse(
                "$timescale 1 ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n#5\n0!\n#5\n1!\n"
            )
            .is_ok()
        );
    }

    #[test]
    fn an_unknown_identifier_is_refused() {
        let error =
            parse("$timescale 1 ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n#0\n0?\n")
                .expect_err("unknown identifier");
        assert!(
            error.to_string().contains("unknown identifier '?'"),
            "{error}"
        );
    }

    #[test]
    fn a_change_must_match_the_kind_it_was_declared_with() {
        let logic =
            parse("$timescale 1 ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n#0\nr1.0 !\n")
                .expect_err("real on wire");
        assert!(logic.to_string().contains("not a real one"), "{logic}");
        let real =
            parse("$timescale 1 ns $end\n$var real 64 ! a $end\n$enddefinitions $end\n#0\n0!\n")
                .expect_err("logic on real");
        assert!(real.to_string().contains("not a logic one"), "{real}");
        let infinite = parse(
            "$timescale 1 ns $end\n$var real 64 ! a $end\n$enddefinitions $end\n#0\nrinf !\n",
        )
        .expect_err("non-finite");
        assert!(infinite.to_string().contains("not finite"), "{infinite}");
    }

    #[test]
    fn widths_beyond_fifty_three_bits_parse() {
        let wide = "1".repeat(96);
        let text = format!(
            "$timescale 1 ns $end\n$var wire 96 ! a $end\n$enddefinitions $end\n#0\nb{wide} !\n"
        );
        let document = parse(&text).expect("wide");
        let signal = document.signals.first().expect("a");
        assert_eq!(signal.width, 96);
        assert_eq!(values(signal), vec![bits(&wide)]);
    }

    #[test]
    fn header_blocks_are_kept_as_metadata() {
        let document = parse(
            "$date\n  Sat Sep 5 2026\n$end\n\
             $version RSpice test $end\n\
             $comment first note $end\n\
             $comment\n  second note\n$end\n\
             $unknown directive nobody defines $end\n\
             $timescale 1 ns $end\n$var wire 1 ! a $end\n$enddefinitions $end\n",
        )
        .expect("metadata");
        assert_eq!(document.date, "Sat Sep 5 2026");
        assert_eq!(document.version, "RSpice test");
        assert_eq!(document.comments, vec!["first note", "second note"]);
    }

    #[test]
    fn an_incomplete_document_is_refused() {
        assert!(
            parse("$timescale 1 ns $end\n$var wire 1 ! a $end\n")
                .expect_err("no enddefinitions")
                .to_string()
                .contains("missing $enddefinitions")
        );
        assert!(
            parse("$var wire 1 ! a $end\n$enddefinitions $end\n")
                .expect_err("no timescale")
                .to_string()
                .contains("missing $timescale")
        );
        assert!(
            parse("$timescale 1 ns $end\n$scope module top $end\n$enddefinitions $end\n")
                .expect_err("open scope")
                .to_string()
                .contains("ends inside scope 'top'")
        );
        assert!(
            parse("$timescale 1 ns $end\n$comment truncated\n")
                .expect_err("truncated")
                .to_string()
                .contains("not terminated by $end")
        );
        assert!(
            parse("$timescale 1 ns $end\n$enddefinitions $end\n$upscope $end\n")
                .expect_err("unbalanced")
                .to_string()
                .contains("leaves no enclosing scope")
        );
        assert!(
            parse("$timescale 1 ns $end\n$var wire 1 ! a $end\nstray\n$enddefinitions $end\n")
                .expect_err("stray")
                .to_string()
                .contains("not a VCD declaration")
        );
        assert!(
            parse("$timescale 1 ns $end\n$var wire 0 ! a $end\n$enddefinitions $end\n")
                .expect_err("zero width")
                .to_string()
                .contains("declares width 0")
        );
    }

    #[test]
    fn every_read_limit_trips_before_the_document_is_built() {
        let bytes = ResourceLimits {
            max_external_data_bytes: 8,
            ..ResourceLimits::default()
        };
        let error =
            parse_vcd_reader_with_limits(UI_FIXTURE.as_bytes(), bytes).expect_err("byte budget");
        assert!(
            matches!(
                error,
                VcdError::ResourceLimit(limit) if limit.resource == ResourceKind::ExternalDataBytes
            ),
            "{error:?}"
        );

        let declarations = ResourceLimits {
            max_external_data_values: 1,
            ..ResourceLimits::default()
        };
        let error = parse_vcd_reader_with_limits(UI_FIXTURE.as_bytes(), declarations)
            .expect_err("declaration budget");
        assert!(
            matches!(
                error,
                VcdError::ResourceLimit(limit) if limit.resource == ResourceKind::ExternalDataValues
            ),
            "{error:?}"
        );

        let retained = ResourceLimits {
            max_result_values: 2,
            ..ResourceLimits::default()
        };
        let error = parse_vcd_reader_with_limits(UI_FIXTURE.as_bytes(), retained)
            .expect_err("retained budget");
        assert!(
            matches!(
                error,
                VcdError::ResourceLimit(limit) if limit.resource == ResourceKind::ResultValues
            ),
            "{error:?}"
        );

        // A declared width is itself a value budget: one change on the
        // variable would materialize that many bits.
        let width = ResourceLimits {
            max_external_data_values: 8,
            ..ResourceLimits::default()
        };
        let error = parse_vcd_reader_with_limits(
            "$timescale 1 ns $end\n$var wire 4096 ! a $end\n$enddefinitions $end\n".as_bytes(),
            width,
        )
        .expect_err("width budget");
        assert!(
            matches!(error, VcdError::ResourceLimit(limit) if limit.requested == 4096),
            "{error:?}"
        );
    }

    #[test]
    fn the_writer_byte_layout_is_pinned() {
        let mut document = VcdDocument::new(VcdTimescale {
            magnitude: VcdMagnitude::One,
            unit: VcdTimeUnit::Nanoseconds,
        });
        document.date = "2026-09-05".to_string();
        document.version = "pinned".to_string();
        document.comments = vec!["one small document".to_string()];
        document.signals = vec![
            VcdSignal {
                identifier: String::new(),
                variables: vec![VcdVariable {
                    scope: vec!["top".to_string()],
                    name: "clk".to_string(),
                }],
                width: 1,
                kind: VcdSignalKind::Logic,
                changes: vec![
                    VcdChange {
                        tick: 0,
                        value: bits("0"),
                    },
                    VcdChange {
                        tick: 5,
                        value: bits("1"),
                    },
                ],
            },
            VcdSignal {
                identifier: String::new(),
                variables: vec![VcdVariable {
                    scope: vec!["top".to_string(), "sub".to_string()],
                    name: "bus".to_string(),
                }],
                width: 2,
                kind: VcdSignalKind::Logic,
                changes: vec![VcdChange {
                    tick: 0,
                    value: bits("xz"),
                }],
            },
            VcdSignal {
                identifier: String::new(),
                variables: vec![VcdVariable {
                    scope: Vec::new(),
                    name: "vout".to_string(),
                }],
                width: 64,
                kind: VcdSignalKind::Real,
                changes: vec![VcdChange {
                    tick: 5,
                    value: VcdValue::Real(1.5),
                }],
            },
        ];

        assert_eq!(
            rendered(&document),
            "$date\n\t2026-09-05\n$end\n\
             $version\n\tpinned\n$end\n\
             $comment\n\tone small document\n$end\n\
             $timescale\n\t1 ns\n$end\n\
             $scope module top $end\n\
             $var wire 1 ! clk $end\n\
             $scope module sub $end\n\
             $var wire 2 \" bus $end\n\
             $upscope $end\n\
             $upscope $end\n\
             $var real 64 # vout $end\n\
             $enddefinitions $end\n\
             #0\n$dumpvars\n0!\nbxz \"\n$end\n\
             #5\n1!\nr1.5 #\n"
        );

        document.assign_canonical_identifiers();
        assert_eq!(parse(&rendered(&document)).expect("round trip"), document);
    }

    #[test]
    fn identifier_codes_never_collide() {
        assert_eq!(canonical_identifier(0), "!");
        assert_eq!(canonical_identifier(1), "\"");
        assert_eq!(canonical_identifier(93), "~");
        assert_eq!(canonical_identifier(94), "!!");
        let mut seen = std::collections::HashSet::new();
        for index in 0..5_000 {
            let identifier = canonical_identifier(index);
            assert!(
                identifier.bytes().all(|byte| (33..=126).contains(&byte)),
                "{identifier}"
            );
            assert!(seen.insert(identifier), "collision at {index}");
        }
    }

    #[test]
    fn a_written_document_reads_back_as_itself() {
        let mut document = parse(UI_FIXTURE).expect("fixture");
        document.date = "2026-09-05".to_string();
        document.assign_canonical_identifiers();
        assert_eq!(parse(&rendered(&document)).expect("round trip"), document);

        // Two names on one identifier survive as two names on one identifier.
        let mut aliased = parse(
            "$timescale 10 ps $end\n\
             $scope module top $end\n$var wire 2 ! bus $end\n$var wire 2 ! mirror $end\n\
             $scope module inner $end\n$var real 64 \" out $end\n$upscope $end\n$upscope $end\n\
             $enddefinitions $end\n#0\nb0x !\nr-1.25 \"\n#7\nbz1 !\n",
        )
        .expect("aliased");
        aliased.assign_canonical_identifiers();
        assert_eq!(parse(&rendered(&aliased)).expect("round trip"), aliased);
    }

    #[test]
    fn the_writer_refuses_what_vcd_cannot_express() {
        let base = |signal: VcdSignal| {
            let mut document = VcdDocument::new(VcdTimescale {
                magnitude: VcdMagnitude::One,
                unit: VcdTimeUnit::Nanoseconds,
            });
            document.signals = vec![signal];
            document
        };
        let variable = |name: &str| VcdVariable {
            scope: Vec::new(),
            name: name.to_string(),
        };

        let mismatched = base(VcdSignal {
            identifier: String::new(),
            variables: vec![variable("bus")],
            width: 4,
            kind: VcdSignalKind::Logic,
            changes: vec![VcdChange {
                tick: 0,
                value: bits("01"),
            }],
        });
        assert!(
            rendered_error(&mismatched).contains("carries 2 bits for a 4-bit variable"),
            "{}",
            rendered_error(&mismatched)
        );

        let unordered = base(VcdSignal {
            identifier: String::new(),
            variables: vec![variable("clk")],
            width: 1,
            kind: VcdSignalKind::Logic,
            changes: vec![
                VcdChange {
                    tick: 5,
                    value: bits("0"),
                },
                VcdChange {
                    tick: 1,
                    value: bits("1"),
                },
            ],
        });
        assert!(rendered_error(&unordered).contains("after a later one"));

        let infinite = base(VcdSignal {
            identifier: String::new(),
            variables: vec![variable("out")],
            width: 64,
            kind: VcdSignalKind::Real,
            changes: vec![VcdChange {
                tick: 0,
                value: VcdValue::Real(f64::NAN),
            }],
        });
        assert!(rendered_error(&infinite).contains("not finite"));

        let spaced = base(VcdSignal {
            identifier: String::new(),
            variables: vec![variable("two  spaces")],
            width: 1,
            kind: VcdSignalKind::Logic,
            changes: Vec::new(),
        });
        assert!(rendered_error(&spaced).contains("not a writable $var name"));

        let zero = base(VcdSignal {
            identifier: String::new(),
            variables: vec![variable("clk")],
            width: 0,
            kind: VcdSignalKind::Logic,
            changes: Vec::new(),
        });
        assert!(rendered_error(&zero).contains("declares width 0"));

        // A bit-select suffix is a legal name and is not caught by the above.
        let selected = base(VcdSignal {
            identifier: String::new(),
            variables: vec![variable("bus [3:0]")],
            width: 4,
            kind: VcdSignalKind::Logic,
            changes: Vec::new(),
        });
        assert!(rendered(&selected).contains("$var wire 4 ! bus [3:0] $end"));
    }

    #[test]
    fn the_writer_version_names_the_crate() {
        assert!(
            VCD_WRITER_VERSION.starts_with("RSpice "),
            "{VCD_WRITER_VERSION}"
        );
        assert!(
            VCD_WRITER_VERSION.ends_with(" VCD writer"),
            "{VCD_WRITER_VERSION}"
        );
        let document = VcdDocument::new(VcdTimescale {
            magnitude: VcdMagnitude::One,
            unit: VcdTimeUnit::Femtoseconds,
        });
        assert_eq!(document.version, VCD_WRITER_VERSION);
    }
}

//! Native Xyce level-4 PEM memristor equations.
//!
//! This is a solver-independent implementation of the threshold-adaptive
//! memristor model registered by Xyce 7.10 as `MEMRISTOR LEVEL=4`.  With
//! `v = v_pos - v_neg`, its terminal law and transient state equation are
//!
//! ```text
//! h(v) = I1 exp(v/V1) - I2 exp(-v/V2) + G0 v - (I1 - I2)
//! i     = x h(v)
//! dx/dt = C g(v) f(x)
//! ```
//!
//! The active Xyce master loads `F_x = C g(v) f(x)` and `Q_x = -x`.
//! Consequently, this module exposes the same charge sign. Xyce 7.10 also
//! stamps `+h(v)` in both terminal rows' state columns. The negative-row entry
//! is not the mathematical derivative of `-i`, but preserving that historical
//! source form is required for exact Xyce nonlinear-solver compatibility.

use crate::Value;
use std::error::Error;
use std::fmt;
use std::sync::Arc;

/// Xyce model level implemented by this module.
pub const XYCE_PEM_MEMRISTOR_LEVEL: u8 = 4;

/// Xyce 7.10's default positive-state table filename.
pub const XYCE_PEM_DEFAULT_POSITIVE_TABLE_FILE: &str = "filep.dat";

/// Xyce 7.10's default negative-state table filename.
pub const XYCE_PEM_DEFAULT_NEGATIVE_TABLE_FILE: &str = "filem.dat";

/// Hard byte budget for one externally supplied PEM state table.
pub(crate) const XYCE_PEM_MAX_TABLE_BYTES: usize = 64 * 1024 * 1024;

/// Hard point-count budget for one externally supplied PEM state table.
pub(crate) const XYCE_PEM_MAX_TABLE_POINTS: usize = 1_000_000;

/// Parameters on a Xyce `.MODEL ... MEMRISTOR LEVEL=4` card.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XycePemModelParams {
    pub v1: Value,
    pub v2: Value,
    pub i1: Value,
    pub i2: Value,
    pub g0: Value,
    pub v_p: Value,
    pub v_n: Value,
    pub d1: Value,
    pub d2: Value,
    pub c1: Value,
    pub c2: Value,
}

impl Default for XycePemModelParams {
    fn default() -> Self {
        Self {
            v1: 1.0,
            v2: 1.0,
            i1: 1.0,
            i2: 1.0,
            g0: 1.0,
            v_p: 1.0e-2,
            v_n: -1.0e-2,
            d1: 1.0,
            d2: 1.0,
            c1: 1.0,
            c2: 1.0,
        }
    }
}

impl XycePemModelParams {
    /// Validate every domain restriction required for finite real equations.
    ///
    /// `VP` is deliberately not required to exceed `VN`.  Xyce accepts
    /// overlapping thresholds and its ordered positive-then-negative branch
    /// selection is observable in existing model cards.
    pub fn validate(&self) -> Result<(), XycePemMemristorError> {
        let finite = [
            ("V1", self.v1),
            ("V2", self.v2),
            ("I1", self.i1),
            ("I2", self.i2),
            ("G0", self.g0),
            ("VP", self.v_p),
            ("VN", self.v_n),
            ("D1", self.d1),
            ("D2", self.d2),
            ("C1", self.c1),
            ("C2", self.c2),
        ];
        for (name, value) in finite {
            if !value.is_finite() {
                return Err(XycePemMemristorError::InvalidParameter {
                    name,
                    reason: "must be finite",
                });
            }
        }
        require(self.v1 != 0.0, "V1", "must be nonzero")?;
        require(self.v2 != 0.0, "V2", "must be nonzero")?;
        Ok(())
    }
}

/// Parameters attached to one Xyce `YMEMRISTOR` instance.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XycePemInstanceParams {
    /// Value supplied through `XO` (or its Xyce default when not given).
    pub x0: Value,
    /// Whether `XO` was explicitly present on the instance.
    pub x0_given: bool,
}

impl Default for XycePemInstanceParams {
    fn default() -> Self {
        Self {
            x0: 0.0,
            x0_given: false,
        }
    }
}

impl XycePemInstanceParams {
    pub fn validate(&self) -> Result<(), XycePemMemristorError> {
        if self.x0.is_finite() {
            Ok(())
        } else {
            Err(XycePemMemristorError::InvalidParameter {
                name: "XO",
                reason: "must be finite",
            })
        }
    }
}

/// One point in an immutable PEM state-function table.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XycePemPwlPoint {
    pub x: Value,
    pub value: Value,
}

impl XycePemPwlPoint {
    #[inline]
    pub const fn new(x: Value, value: Value) -> Self {
        Self { x, value }
    }
}

/// A sampled PWL value and its exact segment derivative.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XycePemPwlSample {
    pub value: Value,
    pub derivative: Value,
}

/// Construction failure for an immutable PEM PWL table.
#[derive(Debug, Clone, PartialEq)]
pub enum XycePemPwlTableError {
    TooFewPoints {
        count: usize,
    },
    NonFinitePoint {
        index: usize,
        column: &'static str,
    },
    NonIncreasingAbscissa {
        index: usize,
        previous: Value,
        current: Value,
    },
}

impl fmt::Display for XycePemPwlTableError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooFewPoints { count } => {
                write!(
                    f,
                    "PEM PWL table requires at least two points; found {count}"
                )
            }
            Self::NonFinitePoint { index, column } => write!(
                f,
                "PEM PWL table point {} has a non-finite {column}",
                index + 1
            ),
            Self::NonIncreasingAbscissa {
                index,
                previous,
                current,
            } => write!(
                f,
                "PEM PWL table x values must be strictly increasing; point {} is {current} after {previous}",
                index + 1
            ),
        }
    }
}

impl Error for XycePemPwlTableError {}

/// Validated immutable PWL data with cheap clone semantics.
///
/// Linear extrapolation uses the first or last segment, matching Xyce's
/// `Util::linear` interpolator.  At an interior knot, the right-hand segment
/// supplies the derivative.
#[derive(Debug, Clone, PartialEq)]
pub struct XycePemPwlTable {
    points: Arc<[XycePemPwlPoint]>,
}

impl XycePemPwlTable {
    pub fn new(points: Vec<XycePemPwlPoint>) -> Result<Self, XycePemPwlTableError> {
        if points.len() < 2 {
            return Err(XycePemPwlTableError::TooFewPoints {
                count: points.len(),
            });
        }
        for (index, point) in points.iter().enumerate() {
            if !point.x.is_finite() {
                return Err(XycePemPwlTableError::NonFinitePoint {
                    index,
                    column: "x value",
                });
            }
            if !point.value.is_finite() {
                return Err(XycePemPwlTableError::NonFinitePoint {
                    index,
                    column: "function value",
                });
            }
            if index > 0 && point.x <= points[index - 1].x {
                return Err(XycePemPwlTableError::NonIncreasingAbscissa {
                    index,
                    previous: points[index - 1].x,
                    current: point.x,
                });
            }
        }
        Ok(Self {
            points: points.into(),
        })
    }

    #[inline]
    pub fn points(&self) -> &[XycePemPwlPoint] {
        &self.points
    }

    #[inline]
    #[cfg(test)]
    pub(crate) fn shares_storage_with(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.points, &other.points)
    }

    #[inline]
    pub fn min_x(&self) -> Value {
        self.points[0].x
    }

    #[inline]
    pub fn max_x(&self) -> Value {
        self.points[self.points.len() - 1].x
    }

    pub fn evaluate(&self, x: Value) -> Result<XycePemPwlSample, XycePemMemristorError> {
        finite_input("x", x)?;
        Ok(self.evaluate_finite(x))
    }

    fn evaluate_finite(&self, x: Value) -> XycePemPwlSample {
        let upper = self.points.partition_point(|point| point.x <= x);
        let lower = if upper == 0 {
            0
        } else if upper >= self.points.len() {
            self.points.len() - 2
        } else {
            upper - 1
        };
        let left = self.points[lower];
        let right = self.points[lower + 1];
        let derivative = (right.value - left.value) / (right.x - left.x);
        XycePemPwlSample {
            value: left.value + (x - left.x) * derivative,
            derivative,
        }
    }
}

/// Failure kind from the explicitly compatible Xyce 7.10 table parser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XycePemLegacyTableParseErrorKind {
    MissingFirstValue,
    MissingSeparator,
    MissingSecondValue,
    TrailingCharacters,
    NonFiniteFirstValue,
    NonFiniteSecondValue,
    TooFewPoints,
    TooManyPoints,
    NonIncreasingAbscissa,
}

/// Deterministic source location for a Xyce 7.10 PEM table parse failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XycePemLegacyTableParseError {
    source_name: String,
    /// One-based line, or `None` for a whole-file structural failure.
    line: Option<usize>,
    /// One-based byte column when `line` is present.
    column: Option<usize>,
    kind: XycePemLegacyTableParseErrorKind,
}

impl XycePemLegacyTableParseError {
    fn at(
        source_name: &str,
        line: usize,
        column: usize,
        kind: XycePemLegacyTableParseErrorKind,
    ) -> Self {
        Self {
            source_name: source_name.to_owned(),
            line: Some(line),
            column: Some(column),
            kind,
        }
    }

    fn file(source_name: &str, kind: XycePemLegacyTableParseErrorKind) -> Self {
        Self {
            source_name: source_name.to_owned(),
            line: None,
            column: None,
            kind,
        }
    }

    #[inline]
    pub fn source_name(&self) -> &str {
        &self.source_name
    }

    #[inline]
    pub fn line(&self) -> Option<usize> {
        self.line
    }

    #[inline]
    pub fn column(&self) -> Option<usize> {
        self.column
    }

    #[inline]
    pub fn kind(&self) -> XycePemLegacyTableParseErrorKind {
        self.kind
    }
}

impl fmt::Display for XycePemLegacyTableParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.source_name)?;
        if let Some(line) = self.line {
            write!(f, ":{line}")?;
            if let Some(column) = self.column {
                write!(f, ":{column}")?;
            }
        }
        let reason = match self.kind {
            XycePemLegacyTableParseErrorKind::MissingFirstValue => "missing first numeric value",
            XycePemLegacyTableParseErrorKind::MissingSeparator => "missing separator character",
            XycePemLegacyTableParseErrorKind::MissingSecondValue => "missing second numeric value",
            XycePemLegacyTableParseErrorKind::TrailingCharacters => {
                "unexpected characters after second value"
            }
            XycePemLegacyTableParseErrorKind::NonFiniteFirstValue => "first value must be finite",
            XycePemLegacyTableParseErrorKind::NonFiniteSecondValue => "second value must be finite",
            XycePemLegacyTableParseErrorKind::TooFewPoints => {
                "table requires at least two data rows"
            }
            XycePemLegacyTableParseErrorKind::TooManyPoints => {
                "table exceeds the configured point-count safety limit"
            }
            XycePemLegacyTableParseErrorKind::NonIncreasingAbscissa => {
                "first-column values must be strictly increasing"
            }
        };
        write!(f, ": {reason}")
    }
}

impl Error for XycePemLegacyTableParseError {}

/// Parse a PEM table with Xyce 7.10's legacy two-extraction convention.
///
/// Xyce extracts `double`, then `char`, then `double`.  A comma-delimited row
/// therefore behaves normally.  On a whitespace-delimited row, formatted
/// character extraction skips the whitespace and consumes the first byte of
/// the second numeric token.  Reproducing that compatibility quirk is
/// essential for the canonical Xyce `.dat` PEM decks: for example,
/// `9.33E-02 6.30E-01` becomes `(0.0933, 0.030)`, and a leading minus sign in
/// the second token is consumed as the separator.
///
/// This parser operates on supplied text and never accesses the filesystem.
/// `source_name` is retained verbatim in every deterministic diagnostic.
/// Comment detection and record extraction follow the source reader's stream
/// position rather than physical lines. A `#` starts a comment only when it is
/// the exact next byte before formatted extraction. Unlike C++ iostream
/// failbit, malformed input produces a deterministic error instead of silently
/// truncating an otherwise valid prefix; this is an intentional fail-closed
/// diagnostic policy and does not change valid-table semantics.
pub fn parse_xyce_7_10_legacy_two_column_table(
    source_name: &str,
    text: &str,
) -> Result<XycePemPwlTable, XycePemLegacyTableParseError> {
    parse_xyce_7_10_legacy_two_column_table_bounded(source_name, text, usize::MAX)
}

/// Parse a PEM table while enforcing a hard point-count budget before each
/// allocation. This is the resource-safe entry point used for external model
/// data; the limit is independent from the byte cap applied by the loader.
pub(crate) fn parse_xyce_7_10_legacy_two_column_table_bounded(
    source_name: &str,
    text: &str,
    max_points: usize,
) -> Result<XycePemPwlTable, XycePemLegacyTableParseError> {
    let mut points: Vec<XycePemPwlPoint> = Vec::new();
    let mut source_lines = Vec::new();
    let mut cursor = 0usize;
    let mut tracked_offset = 0usize;
    let mut tracked_line = 1usize;
    let mut tracked_line_start = 0usize;

    while cursor < text.len() {
        // Xyce peeks before formatted extraction. Consequently, only a '#'
        // at the stream's exact current position begins a comment; whitespace
        // before it is consumed by the next numeric extraction instead.
        if text.as_bytes()[cursor] == b'#' {
            cursor = text[cursor..]
                .find('\n')
                .map_or(text.len(), |relative| cursor + relative + 1);
            continue;
        }

        // `operator>>(double)` and `operator>>(char)` both skip whitespace.
        // Parsing is stream-oriented, so records may span lines or multiple
        // records may share one line just as in Xyce's implementation.
        skip_ascii_whitespace(text, &mut cursor);
        if cursor >= text.len() {
            break;
        }
        let first_offset = cursor;
        let (line_number, first_column) = advance_line_column(
            text,
            first_offset,
            &mut tracked_offset,
            &mut tracked_line,
            &mut tracked_line_start,
        );
        let first = parse_value_prefix(text, &mut cursor).ok_or_else(|| {
            XycePemLegacyTableParseError::at(
                source_name,
                line_number,
                first_column,
                XycePemLegacyTableParseErrorKind::MissingFirstValue,
            )
        })?;
        if !first.is_finite() {
            return Err(XycePemLegacyTableParseError::at(
                source_name,
                line_number,
                first_column,
                XycePemLegacyTableParseErrorKind::NonFiniteFirstValue,
            ));
        }

        skip_ascii_whitespace(text, &mut cursor);
        if cursor >= text.len() {
            let (line, column) = advance_line_column(
                text,
                cursor,
                &mut tracked_offset,
                &mut tracked_line,
                &mut tracked_line_start,
            );
            return Err(XycePemLegacyTableParseError::at(
                source_name,
                line,
                column,
                XycePemLegacyTableParseErrorKind::MissingSeparator,
            ));
        }
        // Intentionally discard exactly one byte.  All accepted numeric and
        // delimiter characters are ASCII, as in Xyce's `char` extraction.
        cursor += 1;
        skip_ascii_whitespace(text, &mut cursor);

        let (second_line, second_column) = advance_line_column(
            text,
            cursor,
            &mut tracked_offset,
            &mut tracked_line,
            &mut tracked_line_start,
        );
        let second = parse_value_prefix(text, &mut cursor).ok_or_else(|| {
            XycePemLegacyTableParseError::at(
                source_name,
                second_line,
                second_column,
                XycePemLegacyTableParseErrorKind::MissingSecondValue,
            )
        })?;
        if !second.is_finite() {
            return Err(XycePemLegacyTableParseError::at(
                source_name,
                second_line,
                second_column,
                XycePemLegacyTableParseErrorKind::NonFiniteSecondValue,
            ));
        }
        if let Some(previous) = points.last() {
            if first <= previous.x {
                return Err(XycePemLegacyTableParseError::at(
                    source_name,
                    line_number,
                    first_column,
                    XycePemLegacyTableParseErrorKind::NonIncreasingAbscissa,
                ));
            }
        }
        if points.len() >= max_points {
            return Err(XycePemLegacyTableParseError::at(
                source_name,
                line_number,
                first_column,
                XycePemLegacyTableParseErrorKind::TooManyPoints,
            ));
        }
        points.push(XycePemPwlPoint::new(first, second));
        source_lines.push(line_number);
    }

    if points.len() < 2 {
        return Err(XycePemLegacyTableParseError::file(
            source_name,
            XycePemLegacyTableParseErrorKind::TooFewPoints,
        ));
    }

    // The parser has already established every constructor invariant.
    XycePemPwlTable::new(points).map_err(|error| match error {
        XycePemPwlTableError::TooFewPoints { .. } => XycePemLegacyTableParseError::file(
            source_name,
            XycePemLegacyTableParseErrorKind::TooFewPoints,
        ),
        XycePemPwlTableError::NonFinitePoint { index, .. } => XycePemLegacyTableParseError::at(
            source_name,
            source_lines[index],
            1,
            XycePemLegacyTableParseErrorKind::NonFiniteFirstValue,
        ),
        XycePemPwlTableError::NonIncreasingAbscissa { index, .. } => {
            XycePemLegacyTableParseError::at(
                source_name,
                source_lines[index],
                1,
                XycePemLegacyTableParseErrorKind::NonIncreasingAbscissa,
            )
        }
    })
}

/// Construction or evaluation failure for the PEM equation kernel.
#[derive(Debug, Clone, PartialEq)]
pub enum XycePemMemristorError {
    InvalidParameter {
        name: &'static str,
        reason: &'static str,
    },
    InvalidTable {
        polarity: &'static str,
        source: XycePemPwlTableError,
    },
    NonFiniteInput {
        name: &'static str,
    },
    NonFiniteEvaluation,
}

impl fmt::Display for XycePemMemristorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidParameter { name, reason } => {
                write!(f, "invalid PEM memristor parameter {name}: {reason}")
            }
            Self::InvalidTable { polarity, source } => {
                write!(f, "invalid PEM {polarity} state table: {source}")
            }
            Self::NonFiniteInput { name } => {
                write!(f, "PEM memristor input {name} must be finite")
            }
            Self::NonFiniteEvaluation => write!(f, "PEM memristor evaluation was not finite"),
        }
    }
}

impl Error for XycePemMemristorError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::InvalidTable { source, .. } => Some(source),
            _ => None,
        }
    }
}

/// Terminal-law values and analytic derivatives at one operating point.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XycePemTerminalLaw {
    pub h: Value,
    pub current: Value,
    /// `dI/d(v_pos-v_neg)`.
    pub conductance: Value,
    /// `dI/dx = h(v)`.
    pub current_state_derivative: Value,
    /// Xyce's stored differential resistance.  It is absent when conductance
    /// is exactly zero because Xyce retains the previous store value then.
    pub incremental_resistance: Option<Value>,
}

/// Dynamic state-drive values and exact derivatives.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XycePemStateDrive {
    pub threshold: Value,
    pub threshold_derivative: Value,
    pub table_value: Value,
    pub table_derivative: Value,
    pub value: Value,
    /// Derivative with respect to `v_pos-v_neg`.
    pub voltage_derivative: Value,
    pub state_derivative: Value,
}

/// Whether the state row represents DC initialization or transient dynamics.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XycePemEvaluationMode {
    DcOperatingPoint,
    Dynamic,
}

/// Complete equation cache for one Newton evaluation.
///
/// Rows and columns of `jacobian` are ordered `(v_pos, v_neg, x)`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct XycePemMemristorCache {
    pub voltage: Value,
    pub terminal: XycePemTerminalLaw,
    /// Dynamic state data, absent for the DC `x - x_inf` gauge equation.
    pub state_drive: Option<XycePemStateDrive>,
    pub residual: [Value; 3],
    pub charge: [Value; 3],
    pub jacobian: [[Value; 3]; 3],
    pub charge_jacobian: [[Value; 3]; 3],
}

/// Validated, solver-independent native PEM memristor equation kernel.
#[derive(Debug, Clone, PartialEq)]
pub struct XycePemMemristor {
    model: XycePemModelParams,
    instance: XycePemInstanceParams,
    positive_table: XycePemPwlTable,
    negative_table: XycePemPwlTable,
}

impl XycePemMemristor {
    pub fn new(
        model: XycePemModelParams,
        instance: XycePemInstanceParams,
        positive_table: XycePemPwlTable,
        negative_table: XycePemPwlTable,
    ) -> Result<Self, XycePemMemristorError> {
        model.validate()?;
        instance.validate()?;
        Ok(Self {
            model,
            instance,
            positive_table,
            negative_table,
        })
    }

    #[inline]
    pub fn model(&self) -> &XycePemModelParams {
        &self.model
    }

    #[inline]
    pub fn instance(&self) -> &XycePemInstanceParams {
        &self.instance
    }

    #[inline]
    pub fn positive_table(&self) -> &XycePemPwlTable {
        &self.positive_table
    }

    #[inline]
    pub fn negative_table(&self) -> &XycePemPwlTable {
        &self.negative_table
    }

    /// State selected during DC operating-point initialization.
    ///
    /// An explicit `XO` always wins.  Otherwise Xyce selects one only for a
    /// strictly positive terminal voltage and zero for equality or negativity.
    pub fn dc_state_target(
        &self,
        v_pos: Value,
        v_neg: Value,
    ) -> Result<Value, XycePemMemristorError> {
        finite_input("v_pos", v_pos)?;
        finite_input("v_neg", v_neg)?;
        Ok(if self.instance.x0_given {
            self.instance.x0
        } else if v_pos > v_neg {
            1.0
        } else {
            0.0
        })
    }

    /// Evaluate the voltage-controlled terminal current law.
    pub fn terminal_law(
        &self,
        voltage: Value,
        x: Value,
    ) -> Result<XycePemTerminalLaw, XycePemMemristorError> {
        finite_input("voltage", voltage)?;
        finite_input("x", x)?;

        let positive_exponential = (voltage / self.model.v1).exp();
        let negative_exponential = (-voltage / self.model.v2).exp();
        let h = self.model.i1 * positive_exponential - self.model.i2 * negative_exponential
            + self.model.g0 * voltage
            - (self.model.i1 - self.model.i2);
        let conductance = x
            * (self.model.i1 / self.model.v1 * positive_exponential
                + self.model.i2 / self.model.v2 * negative_exponential
                + self.model.g0);
        let current = x * h;
        if !h.is_finite() || !conductance.is_finite() || !current.is_finite() {
            return Err(XycePemMemristorError::NonFiniteEvaluation);
        }
        Ok(XycePemTerminalLaw {
            h,
            current,
            conductance,
            current_state_derivative: h,
            incremental_resistance: (conductance != 0.0).then(|| conductance.recip()),
        })
    }

    /// Evaluate `C g(v) f(x)` and its exact analytic derivatives.
    pub fn dynamic_state_drive(
        &self,
        voltage: Value,
        x: Value,
    ) -> Result<XycePemStateDrive, XycePemMemristorError> {
        finite_input("voltage", voltage)?;
        finite_input("x", x)?;

        // Compatibility-critical order: the positive branch wins when the
        // thresholds overlap and both comparisons are true.
        let (threshold, threshold_derivative) = if voltage >= self.model.v_p {
            let exponential = (self.model.d1 * (voltage - self.model.v_p)).exp();
            (exponential - 1.0, self.model.d1 * exponential)
        } else if voltage <= self.model.v_n {
            let exponential = (self.model.d2 * (voltage - self.model.v_n)).exp();
            (exponential - 1.0, self.model.d2 * exponential)
        } else {
            (0.0, 0.0)
        };

        let (coefficient, table_sample) = if voltage >= 0.0 {
            if x > self.positive_table.max_x() {
                (
                    self.model.c1,
                    XycePemPwlSample {
                        value: 0.0,
                        derivative: 0.0,
                    },
                )
            } else {
                (self.model.c1, self.positive_table.evaluate_finite(x))
            }
        } else if x < self.negative_table.min_x() {
            (
                self.model.c2,
                XycePemPwlSample {
                    value: 0.0,
                    derivative: 0.0,
                },
            )
        } else {
            (self.model.c2, self.negative_table.evaluate_finite(x))
        };

        let value = coefficient * threshold * table_sample.value;
        let voltage_derivative = coefficient * threshold_derivative * table_sample.value;
        let state_derivative = coefficient * threshold * table_sample.derivative;
        let result = XycePemStateDrive {
            threshold,
            threshold_derivative,
            table_value: table_sample.value,
            table_derivative: table_sample.derivative,
            value,
            voltage_derivative,
            state_derivative,
        };
        if [
            result.threshold,
            result.threshold_derivative,
            result.table_value,
            result.table_derivative,
            result.value,
            result.voltage_derivative,
            result.state_derivative,
        ]
        .iter()
        .any(|value| !value.is_finite())
        {
            return Err(XycePemMemristorError::NonFiniteEvaluation);
        }
        Ok(result)
    }

    /// Evaluate `F`, `Q`, and their exact analytic Jacobians.
    pub fn evaluate(
        &self,
        v_pos: Value,
        v_neg: Value,
        x: Value,
        mode: XycePemEvaluationMode,
    ) -> Result<XycePemMemristorCache, XycePemMemristorError> {
        finite_input("v_pos", v_pos)?;
        finite_input("v_neg", v_neg)?;
        finite_input("x", x)?;

        let voltage = v_pos - v_neg;
        if !voltage.is_finite() {
            return Err(XycePemMemristorError::NonFiniteEvaluation);
        }
        let terminal = self.terminal_law(voltage, x)?;
        let (state_drive, state_residual, state_jacobian) = match mode {
            XycePemEvaluationMode::DcOperatingPoint => {
                let target = self.dc_state_target(v_pos, v_neg)?;
                (None, x - target, [0.0, 0.0, 1.0])
            }
            XycePemEvaluationMode::Dynamic => {
                let drive = self.dynamic_state_drive(voltage, x)?;
                (
                    Some(drive),
                    drive.value,
                    [
                        drive.voltage_derivative,
                        -drive.voltage_derivative,
                        drive.state_derivative,
                    ],
                )
            }
        };

        let residual = [terminal.current, -terminal.current, state_residual];
        let jacobian = [
            [
                terminal.conductance,
                -terminal.conductance,
                terminal.current_state_derivative,
            ],
            [
                -terminal.conductance,
                terminal.conductance,
                terminal.current_state_derivative,
            ],
            state_jacobian,
        ];
        let charge = [0.0, 0.0, -x];
        let charge_jacobian = [[0.0; 3], [0.0; 3], [0.0, 0.0, -1.0]];

        if residual.iter().any(|value| !value.is_finite())
            || jacobian.iter().flatten().any(|value| !value.is_finite())
        {
            return Err(XycePemMemristorError::NonFiniteEvaluation);
        }
        Ok(XycePemMemristorCache {
            voltage,
            terminal,
            state_drive,
            residual,
            charge,
            jacobian,
            charge_jacobian,
        })
    }
}

fn require(
    condition: bool,
    name: &'static str,
    reason: &'static str,
) -> Result<(), XycePemMemristorError> {
    if condition {
        Ok(())
    } else {
        Err(XycePemMemristorError::InvalidParameter { name, reason })
    }
}

fn finite_input(name: &'static str, value: Value) -> Result<(), XycePemMemristorError> {
    if value.is_finite() {
        Ok(())
    } else {
        Err(XycePemMemristorError::NonFiniteInput { name })
    }
}

fn advance_line_column(
    text: &str,
    byte_offset: usize,
    tracked_offset: &mut usize,
    tracked_line: &mut usize,
    tracked_line_start: &mut usize,
) -> (usize, usize) {
    let bounded = byte_offset.min(text.len());
    debug_assert!(bounded >= *tracked_offset);
    for (relative, &byte) in text.as_bytes()[*tracked_offset..bounded].iter().enumerate() {
        if byte == b'\n' {
            *tracked_line += 1;
            *tracked_line_start = *tracked_offset + relative + 1;
        }
    }
    *tracked_offset = bounded;
    (*tracked_line, bounded - *tracked_line_start + 1)
}

fn skip_ascii_whitespace(text: &str, cursor: &mut usize) {
    while text
        .as_bytes()
        .get(*cursor)
        .is_some_and(u8::is_ascii_whitespace)
    {
        *cursor += 1;
    }
}

fn parse_value_prefix(text: &str, cursor: &mut usize) -> Option<Value> {
    let bytes = text.as_bytes();
    let start = *cursor;
    if matches!(bytes.get(*cursor), Some(b'+' | b'-')) {
        *cursor += 1;
    }

    let integer_start = *cursor;
    while bytes.get(*cursor).is_some_and(u8::is_ascii_digit) {
        *cursor += 1;
    }
    let mut digit_count = *cursor - integer_start;
    if bytes.get(*cursor) == Some(&b'.') {
        *cursor += 1;
        let fraction_start = *cursor;
        while bytes.get(*cursor).is_some_and(u8::is_ascii_digit) {
            *cursor += 1;
        }
        digit_count += *cursor - fraction_start;
    }
    if digit_count == 0 {
        *cursor = start;
        return None;
    }

    if matches!(bytes.get(*cursor), Some(b'e' | b'E')) {
        let exponent_marker = *cursor;
        *cursor += 1;
        if matches!(bytes.get(*cursor), Some(b'+' | b'-')) {
            *cursor += 1;
        }
        let exponent_start = *cursor;
        while bytes.get(*cursor).is_some_and(u8::is_ascii_digit) {
            *cursor += 1;
        }
        if exponent_start == *cursor {
            *cursor = exponent_marker;
        }
    }

    text[start..*cursor].parse().ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn table(points: &[(Value, Value)]) -> XycePemPwlTable {
        XycePemPwlTable::new(
            points
                .iter()
                .map(|&(x, value)| XycePemPwlPoint::new(x, value))
                .collect(),
        )
        .unwrap()
    }

    fn device(model: XycePemModelParams, instance: XycePemInstanceParams) -> XycePemMemristor {
        XycePemMemristor::new(
            model,
            instance,
            table(&[(0.0, 1.0), (0.5, 0.75), (1.0, 0.0)]),
            table(&[(0.0, 0.0), (0.5, 0.5), (1.0, 1.5)]),
        )
        .unwrap()
    }

    fn assert_close(actual: Value, expected: Value, relative: Value, absolute: Value) {
        let tolerance = absolute + relative * actual.abs().max(expected.abs());
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual:.16e}, expected={expected:.16e}, tolerance={tolerance:.3e}"
        );
    }

    #[test]
    fn defaults_match_xyce_7_10_level_four_metadata() {
        assert_eq!(XYCE_PEM_MEMRISTOR_LEVEL, 4);
        assert_eq!(XYCE_PEM_DEFAULT_POSITIVE_TABLE_FILE, "filep.dat");
        assert_eq!(XYCE_PEM_DEFAULT_NEGATIVE_TABLE_FILE, "filem.dat");
        assert_eq!(
            XycePemModelParams::default(),
            XycePemModelParams {
                v1: 1.0,
                v2: 1.0,
                i1: 1.0,
                i2: 1.0,
                g0: 1.0,
                v_p: 1.0e-2,
                v_n: -1.0e-2,
                d1: 1.0,
                d2: 1.0,
                c1: 1.0,
                c2: 1.0,
            }
        );
        assert_eq!(XycePemInstanceParams::default().x0, 0.0);
        assert!(!XycePemInstanceParams::default().x0_given);
    }

    #[test]
    fn validation_rejects_nonfinite_and_zero_voltage_scales() {
        let mut model = XycePemModelParams::default();
        model.v1 = 0.0;
        assert!(matches!(
            model.validate(),
            Err(XycePemMemristorError::InvalidParameter { name: "V1", .. })
        ));
        model = XycePemModelParams::default();
        model.c2 = Value::NAN;
        assert!(matches!(
            model.validate(),
            Err(XycePemMemristorError::InvalidParameter { name: "C2", .. })
        ));
        assert!(
            XycePemInstanceParams {
                x0: Value::INFINITY,
                x0_given: true,
            }
            .validate()
            .is_err()
        );
    }

    #[test]
    fn pwl_table_requires_finite_strictly_increasing_points() {
        assert!(matches!(
            XycePemPwlTable::new(vec![XycePemPwlPoint::new(0.0, 1.0)]),
            Err(XycePemPwlTableError::TooFewPoints { count: 1 })
        ));
        assert!(matches!(
            XycePemPwlTable::new(vec![
                XycePemPwlPoint::new(0.0, 1.0),
                XycePemPwlPoint::new(0.0, 2.0),
            ]),
            Err(XycePemPwlTableError::NonIncreasingAbscissa { index: 1, .. })
        ));
        assert!(matches!(
            XycePemPwlTable::new(vec![
                XycePemPwlPoint::new(0.0, 1.0),
                XycePemPwlPoint::new(1.0, Value::NAN),
            ]),
            Err(XycePemPwlTableError::NonFinitePoint {
                index: 1,
                column: "function value"
            })
        ));
    }

    #[test]
    fn pwl_interpolates_and_extrapolates_with_right_derivative_at_knots() {
        let data = table(&[(0.0, 1.0), (1.0, 3.0), (3.0, 4.0)]);
        assert_eq!(
            data.evaluate(-1.0).unwrap(),
            XycePemPwlSample {
                value: -1.0,
                derivative: 2.0
            }
        );
        assert_eq!(
            data.evaluate(1.0).unwrap(),
            XycePemPwlSample {
                value: 3.0,
                derivative: 0.5
            }
        );
        assert_eq!(
            data.evaluate(5.0).unwrap(),
            XycePemPwlSample {
                value: 5.0,
                derivative: 0.5
            }
        );
    }

    #[test]
    fn legacy_parser_handles_csv_and_column_one_comments() {
        let data = parse_xyce_7_10_legacy_two_column_table(
            "positive.csv",
            "# x,f(x)\n0.0,1.0\n0.5, 0.25\n1.0,0.0\n",
        )
        .unwrap();
        assert_eq!(
            data.points(),
            &[
                XycePemPwlPoint::new(0.0, 1.0),
                XycePemPwlPoint::new(0.5, 0.25),
                XycePemPwlPoint::new(1.0, 0.0),
            ]
        );
    }

    #[test]
    fn legacy_parser_reproduces_whitespace_separator_character_quirk() {
        let data = parse_xyce_7_10_legacy_two_column_table(
            "legacy.dat",
            "9.33E-02 6.30E-01\n1.00E-01 -7.25E-01\n",
        )
        .unwrap();
        assert_close(data.points()[0].value, 0.030, 1e-15, 1e-15);
        // The minus sign is consumed as the mandatory separator character.
        assert_close(data.points()[1].value, 0.725, 1e-15, 1e-15);
    }

    #[test]
    fn legacy_parser_extracts_records_as_a_continuous_stream() {
        let data = parse_xyce_7_10_legacy_two_column_table(
            "stream.csv",
            "# header\n0.0,1.0 0.5,\n0.25 1.0,0.0\n",
        )
        .expect("records may share or span physical lines");
        assert_eq!(
            data.points(),
            &[
                XycePemPwlPoint::new(0.0, 1.0),
                XycePemPwlPoint::new(0.5, 0.25),
                XycePemPwlPoint::new(1.0, 0.0),
            ]
        );
    }

    #[test]
    fn bounded_legacy_parser_rejects_before_allocating_an_extra_point() {
        let error =
            parse_xyce_7_10_legacy_two_column_table_bounded("bounded.csv", "0,0\n1,1\n2,2\n", 2)
                .expect_err("third point exceeds the hard budget");
        assert_eq!(error.line(), Some(3));
        assert_eq!(error.column(), Some(1));
        assert_eq!(
            error.kind(),
            XycePemLegacyTableParseErrorKind::TooManyPoints
        );
    }

    #[test]
    fn legacy_parser_reports_stable_file_line_and_column_errors() {
        let error =
            parse_xyce_7_10_legacy_two_column_table("bad.dat", "# header\n0.0,1.0\n0.5,broken\n")
                .unwrap_err();
        assert_eq!(error.source_name(), "bad.dat");
        assert_eq!(error.line(), Some(3));
        assert_eq!(error.column(), Some(5));
        assert_eq!(
            error.kind(),
            XycePemLegacyTableParseErrorKind::MissingSecondValue
        );
        assert_eq!(
            error.to_string(),
            "bad.dat:3:5: missing second numeric value"
        );

        let error = parse_xyce_7_10_legacy_two_column_table("empty.csv", "# only\n").unwrap_err();
        assert_eq!(error.line(), None);
        assert_eq!(error.kind(), XycePemLegacyTableParseErrorKind::TooFewPoints);
    }

    #[test]
    fn terminal_law_matches_exact_current_conductance_and_resistance() {
        let model = XycePemModelParams {
            v1: 0.7,
            v2: 1.3,
            i1: 2.0e-4,
            i2: 5.0e-5,
            g0: 8.0e-4,
            ..XycePemModelParams::default()
        };
        let device = device(model, XycePemInstanceParams::default());
        let v = 0.31;
        let x = 0.42;
        let law = device.terminal_law(v, x).unwrap();
        let h = model.i1 * (v / model.v1).exp() - model.i2 * (-v / model.v2).exp() + model.g0 * v
            - (model.i1 - model.i2);
        let conductance = x
            * (model.i1 / model.v1 * (v / model.v1).exp()
                + model.i2 / model.v2 * (-v / model.v2).exp()
                + model.g0);
        assert_close(law.current, x * h, 1e-14, 1e-16);
        assert_close(law.conductance, conductance, 1e-14, 1e-16);
        assert_close(law.current_state_derivative, h, 1e-14, 1e-16);
        assert_close(
            law.incremental_resistance.unwrap(),
            conductance.recip(),
            1e-14,
            1e-14,
        );
        assert_eq!(
            device.terminal_law(v, 0.0).unwrap().incremental_resistance,
            None
        );
    }

    #[test]
    fn threshold_branch_order_and_zero_voltage_table_selection_match_xyce() {
        let model = XycePemModelParams {
            v_p: -0.5,
            v_n: 0.5,
            d1: 2.0,
            d2: -7.0,
            c1: 3.0,
            c2: 11.0,
            ..XycePemModelParams::default()
        };
        let device = device(model, XycePemInstanceParams::default());
        let drive = device.dynamic_state_drive(0.0, 0.25).unwrap();
        let expected_threshold = (2.0_f64 * 0.5).exp() - 1.0;
        assert_close(drive.threshold, expected_threshold, 1e-14, 1e-14);
        assert_close(drive.table_value, 0.875, 1e-14, 1e-14);
        assert_close(drive.value, 3.0 * expected_threshold * 0.875, 1e-14, 1e-14);
    }

    #[test]
    fn state_tables_apply_only_the_xyce_one_sided_clamps() {
        let model = XycePemModelParams {
            v_p: 0.1,
            v_n: -0.1,
            ..XycePemModelParams::default()
        };
        let device = device(model, XycePemInstanceParams::default());

        let positive_clamped = device.dynamic_state_drive(0.2, 1.01).unwrap();
        assert_eq!(positive_clamped.table_value, 0.0);
        assert_eq!(positive_clamped.table_derivative, 0.0);
        let positive_opposite_side = device.dynamic_state_drive(0.2, -0.2).unwrap();
        assert_ne!(positive_opposite_side.table_value, 0.0);
        assert_ne!(positive_opposite_side.table_derivative, 0.0);

        let negative_clamped = device.dynamic_state_drive(-0.2, -0.01).unwrap();
        assert_eq!(negative_clamped.table_value, 0.0);
        assert_eq!(negative_clamped.table_derivative, 0.0);
        let negative_opposite_side = device.dynamic_state_drive(-0.2, 1.2).unwrap();
        assert_ne!(negative_opposite_side.table_value, 0.0);
        assert_ne!(negative_opposite_side.table_derivative, 0.0);
    }

    #[test]
    fn dc_state_policy_honors_explicit_xo_and_strict_voltage_sign() {
        let defaulted = device(
            XycePemModelParams::default(),
            XycePemInstanceParams::default(),
        );
        assert_eq!(defaulted.dc_state_target(0.1, 0.0).unwrap(), 1.0);
        assert_eq!(defaulted.dc_state_target(0.0, 0.0).unwrap(), 0.0);
        assert_eq!(defaulted.dc_state_target(-0.1, 0.0).unwrap(), 0.0);

        let explicit = device(
            XycePemModelParams::default(),
            XycePemInstanceParams {
                x0: 0.11,
                x0_given: true,
            },
        );
        assert_eq!(explicit.dc_state_target(-1.0, 1.0).unwrap(), 0.11);
    }

    #[test]
    fn dynamic_dae_jacobian_matches_derivatives_and_xyce_historical_source_form() {
        let model = XycePemModelParams {
            v1: 0.8,
            v2: 1.2,
            i1: 2.0e-4,
            i2: 7.0e-5,
            g0: 4.0e-4,
            v_p: 0.15,
            v_n: -0.2,
            d1: 3.0,
            d2: 2.5,
            c1: 0.7,
            c2: 1.3,
        };
        let device = device(model, XycePemInstanceParams::default());
        for point in [[0.42, 0.0, 0.3], [-0.45, 0.0, 0.7]] {
            let cache = device
                .evaluate(point[0], point[1], point[2], XycePemEvaluationMode::Dynamic)
                .unwrap();
            for column in 0..3 {
                let step = 2.0e-7;
                let mut plus = point;
                let mut minus = point;
                plus[column] += step;
                minus[column] -= step;
                let f_plus = device
                    .evaluate(plus[0], plus[1], plus[2], XycePemEvaluationMode::Dynamic)
                    .unwrap();
                let f_minus = device
                    .evaluate(minus[0], minus[1], minus[2], XycePemEvaluationMode::Dynamic)
                    .unwrap();
                for row in 0..3 {
                    if row == 1 && column == 2 {
                        continue;
                    }
                    let finite_difference =
                        (f_plus.residual[row] - f_minus.residual[row]) / (2.0 * step);
                    assert_close(
                        cache.jacobian[row][column],
                        finite_difference,
                        2.0e-7,
                        2.0e-10,
                    );
                }
            }
            assert_eq!(cache.charge, [0.0, 0.0, -point[2]]);
            assert_eq!(cache.charge_jacobian[2][2], -1.0);
            assert_eq!(cache.jacobian[1][2], cache.jacobian[0][2]);
        }
    }

    #[test]
    fn dc_equation_has_exact_gauge_row_and_retains_terminal_law() {
        let device = device(
            XycePemModelParams::default(),
            XycePemInstanceParams {
                x0: 0.11,
                x0_given: true,
            },
        );
        let cache = device
            .evaluate(0.2, 0.0, 0.3, XycePemEvaluationMode::DcOperatingPoint)
            .unwrap();
        assert_eq!(cache.state_drive, None);
        assert_close(cache.residual[2], 0.19, 1e-14, 1e-14);
        assert_eq!(cache.jacobian[2], [0.0, 0.0, 1.0]);
        assert_eq!(cache.residual[1], -cache.residual[0]);
        assert_eq!(cache.charge[2], -0.3);
    }
}

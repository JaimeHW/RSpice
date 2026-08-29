//! Xyce-compatible standard `.prn` serialization.
//!
//! The serializer accepts already projected real-valued tables. It owns only
//! byte layout: the typed `.PRINT` request supplies delimiter, precision, and
//! width, while typed simulation options supply header/footer policy. A
//! sequence retains each table's local Index column instead of merging or
//! renumbering independently executed parameter-sweep runs.

use crate::Value;
use thiserror::Error;

const DEFAULT_SCIENTIFIC_PRECISION: i32 = 8;
const DEFAULT_FIELD_WIDTH: i32 = 17;

/// Low-level delimiter view consumed by the PRN byte serializer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XycePrnDelimiter<'a> {
    /// Xyce's fixed-width, whitespace-delimited table layout.
    Whitespace,
    /// An unpadded field separator, including tab and authored custom text.
    Separated(&'a str),
}

impl XycePrnDelimiter<'_> {
    fn separator(&self) -> &str {
        match self {
            Self::Whitespace => " ",
            Self::Separated(separator) => separator,
        }
    }

    fn is_padded(self) -> bool {
        matches!(self, Self::Whitespace)
    }
}

/// Adapter implemented by typed delimiter owners above the byte-I/O layer.
pub trait XycePrnDelimiterSource {
    /// Borrow the exact delimiter semantics needed by the serializer.
    fn xyce_prn_delimiter(&self) -> XycePrnDelimiter<'_>;
}

impl XycePrnDelimiterSource for XycePrnDelimiter<'_> {
    fn xyce_prn_delimiter(&self) -> XycePrnDelimiter<'_> {
        *self
    }
}

/// Minimal typed `.PRINT` contract consumed by the PRN byte serializer.
///
/// Netlist parsing owns the source-level request. This lower-layer trait keeps
/// serialization independent of that AST while preserving direct calls with
/// the public netlist request type.
pub trait XycePrnRequest {
    /// Whether this is a `.PRINT` request.
    fn xyce_prn_is_print_request(&self) -> bool;
    /// Effective delimiter, absent only for a non-print request.
    fn xyce_prn_delimiter(&self) -> Option<XycePrnDelimiter<'_>>;
    /// Authored signed scientific precision.
    fn xyce_prn_precision(&self) -> Option<i32>;
    /// Authored signed field width.
    fn xyce_prn_width(&self) -> Option<i32>;
}

/// Minimal output-policy contract consumed by the PRN byte serializer.
pub trait XycePrnOutputOptions {
    /// Optional header policy; absence means enabled.
    fn xyce_prn_print_header(&self) -> Option<bool>;
    /// Optional footer policy; absence means enabled.
    fn xyce_prn_print_footer(&self) -> Option<bool>;
}

#[derive(Debug, Clone, Copy)]
struct XycePrnFormat<'a> {
    is_print_request: bool,
    delimiter: Option<XycePrnDelimiter<'a>>,
    precision: Option<i32>,
    width: Option<i32>,
    print_header: Option<bool>,
    print_footer: Option<bool>,
}

/// One projected real-valued Xyce STD `.prn` table.
#[derive(Debug, Clone, PartialEq)]
pub struct XycePrnTable {
    /// Ordered output-column names, including `Index` when the stream carries
    /// Xyce's standard local row index.
    pub columns: Vec<String>,
    /// Real-valued rows in exactly the same order and arity as `columns`.
    pub rows: Vec<Vec<Value>>,
}

/// Terminal marker associated with a rendered Xyce output stream.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XycePrnFooter {
    /// No terminal marker, used when another independently rendered table is
    /// concatenated immediately afterward.
    None,
    /// `End of Xyce(TM) Simulation`.
    Simulation,
    /// `End of Xyce(TM) Parameter Sweep`.
    ParameterSweep,
}

/// Scientific exponent spelling selected by the shared bounded layout engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XycePrnScientificStyle {
    /// C++/Xyce spelling with an explicit sign and at least two digits
    /// (`e+00`, `e-09`).
    Canonical,
    /// Legacy RSpice comparison spelling (`e0`, `e-9`). This is not Xyce
    /// wire format and must not be used for export, persistence, or
    /// interchange.
    LegacyRspiceComparison,
}

/// Caller-owned resource envelope for one serialization operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct XycePrnLimits {
    /// Maximum aggregate row count across all tables in one stream.
    pub max_rows: usize,
    /// Maximum UTF-8 byte length of the complete serialized stream.
    pub max_output_bytes: usize,
}

impl XycePrnLimits {
    #[must_use]
    pub const fn new(max_rows: usize, max_output_bytes: usize) -> Self {
        Self {
            max_rows,
            max_output_bytes,
        }
    }
}

/// Fail-closed errors from Xyce STD `.prn` serialization.
#[derive(Debug, Error)]
pub enum XycePrnError {
    #[error("standard PRN rendering requires a typed .PRINT request")]
    NotPrintRequest,
    #[error("typed .PRINT request lost its delimiter")]
    MissingDelimiter,
    #[error("Xyce custom PRN delimiter must be nonempty")]
    EmptyCustomDelimiter,
    #[error("standard PRN sequence requires at least one table")]
    EmptySequence,
    #[error("standard PRN output requires at least one column")]
    EmptySchema,
    #[error("standard PRN sequence table {table} has a different schema")]
    SchemaMismatch { table: usize },
    #[error("standard PRN sequence has {rows} rows, exceeding the {maximum}-row envelope")]
    RowLimit { rows: usize, maximum: usize },
    #[error("standard PRN sequence row count overflowed usize")]
    RowCountOverflow,
    #[error("standard PRN table {table} row {row} has {fields} fields for {columns} columns")]
    RowShape {
        table: usize,
        row: usize,
        fields: usize,
        columns: usize,
    },
    #[error("standard PRN table {table} row {row} has invalid Index value {value}")]
    InvalidIndex {
        table: usize,
        row: usize,
        value: Value,
    },
    #[error(
        "standard PRN table {table} row {row} column {column} cannot serialize non-finite value {value}"
    )]
    NonFinite {
        table: usize,
        row: usize,
        column: usize,
        value: Value,
    },
    #[error("scientific .prn output cannot serialize non-finite value {0}")]
    NonFiniteValue(Value),
    #[error("scientific .prn precision must be nonnegative, got {0}")]
    NegativePrecision(i32),
    #[error("standard PRN effective width overflowed")]
    WidthOverflow,
    #[error("standard PRN precision length overflowed usize")]
    PrecisionLengthOverflow,
    #[error("standard PRN precision {precision} exceeds the {maximum}-byte output envelope")]
    PrecisionLimit { precision: usize, maximum: usize },
    #[error("standard PRN width {width} exceeds the {maximum}-byte output envelope")]
    WidthLimit { width: usize, maximum: usize },
    #[error("standard PRN output length overflowed usize")]
    OutputLengthOverflow,
    #[error("standard PRN output exceeds the {maximum}-byte envelope")]
    OutputLimit { maximum: usize },
    #[error("scientific .prn formatter omitted its exponent")]
    MissingExponent,
    #[error("scientific .prn exponent is invalid: {0}")]
    InvalidExponent(#[source] std::num::ParseIntError),
}

fn push_bounded(output: &mut String, fragment: &str, maximum: usize) -> Result<(), XycePrnError> {
    let new_len = output
        .len()
        .checked_add(fragment.len())
        .ok_or(XycePrnError::OutputLengthOverflow)?;
    if new_len > maximum {
        return Err(XycePrnError::OutputLimit { maximum });
    }
    output.push_str(fragment);
    Ok(())
}

/// Format one finite scalar with the selected bounded exponent spelling.
///
/// `max_output_bytes` bounds the token itself. The function proves the worst
/// IEEE-754 token length for `precision` fits that ceiling before invoking
/// Rust's precision-sensitive formatter, so an untrusted precision cannot
/// cause an unbounded intermediate allocation. Use
/// [`XycePrnScientificStyle::Canonical`] for Xyce's C++ spelling. The legacy
/// comparison style exists only to preserve established in-process numerical
/// relations; it is not Xyce wire format.
pub fn format_xyce_prn_scientific(
    value: Value,
    precision: usize,
    max_output_bytes: usize,
    style: XycePrnScientificStyle,
) -> Result<String, XycePrnError> {
    if !value.is_finite() {
        return Err(XycePrnError::NonFiniteValue(value));
    }
    let maximum_serialized_value = precision
        .checked_add(9)
        .ok_or(XycePrnError::PrecisionLengthOverflow)?;
    if maximum_serialized_value > max_output_bytes {
        return Err(XycePrnError::PrecisionLimit {
            precision,
            maximum: max_output_bytes,
        });
    }
    let raw = format!("{value:.precision$e}", precision = precision);
    if style == XycePrnScientificStyle::LegacyRspiceComparison {
        return Ok(raw);
    }
    let (mantissa, exponent) = raw.rsplit_once('e').ok_or(XycePrnError::MissingExponent)?;
    let exponent = exponent
        .parse::<i32>()
        .map_err(XycePrnError::InvalidExponent)?;
    // C++ iostream scientific formatting always includes an exponent sign
    // and at least two exponent digits. Rust omits both; normalize to Xyce's
    // byte spelling (while naturally retaining a three-digit IEEE exponent).
    Ok(format!("{mantissa}e{exponent:+03}"))
}

fn push_field_separator(
    output: &mut String,
    separator: &str,
    field_index: usize,
    maximum: usize,
) -> Result<(), XycePrnError> {
    if field_index != 0 {
        push_bounded(output, separator, maximum)?;
    }
    Ok(())
}

/// Serialize one table with RSpice's legacy compact comparison layout.
///
/// This deliberately emits compact Rust exponents and no fixed-width padding,
/// so it is not Xyce wire format and must not be used for export, persistence,
/// or interchange. It exists only for established in-process conformance
/// relations. `limits` is mandatory so callers cannot restore an unbounded
/// formatter.
pub fn serialize_legacy_compact_prn_for_comparison<D: XycePrnDelimiterSource + ?Sized>(
    table: &XycePrnTable,
    delimiter: &D,
    limits: XycePrnLimits,
) -> Result<String, XycePrnError> {
    // Whitespace selects authored WIDTH in the sequence API. Represent one
    // ASCII space as a custom separator to retain this convenience function's
    // deliberately compact layout while sharing the production engine.
    let compact_delimiter = match delimiter.xyce_prn_delimiter() {
        XycePrnDelimiter::Whitespace => XycePrnDelimiter::Separated(" "),
        other => other,
    };
    serialize_xyce_prn_sequence_with_style(
        std::slice::from_ref(table),
        XycePrnFormat {
            is_print_request: true,
            delimiter: Some(compact_delimiter),
            precision: None,
            width: None,
            print_header: None,
            print_footer: None,
        },
        XycePrnFooter::Simulation,
        limits,
        XycePrnScientificStyle::LegacyRspiceComparison,
    )
}

/// Serialize one or more projected tables as one Xyce STD `.prn` stream.
///
/// This function never renumbers table-local Index columns. It validates all
/// row shapes and numeric values and checks the output byte ceiling before
/// every append. Scientific precision is bounded by `max_output_bytes` before
/// Rust's formatter is invoked, so even an extreme authored precision cannot
/// trigger an unbounded intermediate allocation.
pub fn serialize_xyce_prn_sequence<R, O>(
    tables: &[XycePrnTable],
    request: &R,
    options: &O,
    footer: XycePrnFooter,
    limits: XycePrnLimits,
) -> Result<String, XycePrnError>
where
    R: XycePrnRequest + ?Sized,
    O: XycePrnOutputOptions + ?Sized,
{
    serialize_xyce_prn_sequence_with_style(
        tables,
        XycePrnFormat {
            is_print_request: request.xyce_prn_is_print_request(),
            delimiter: request.xyce_prn_delimiter(),
            precision: request.xyce_prn_precision(),
            width: request.xyce_prn_width(),
            print_header: options.xyce_prn_print_header(),
            print_footer: options.xyce_prn_print_footer(),
        },
        footer,
        limits,
        XycePrnScientificStyle::Canonical,
    )
}

fn serialize_xyce_prn_sequence_with_style(
    tables: &[XycePrnTable],
    format: XycePrnFormat<'_>,
    footer: XycePrnFooter,
    limits: XycePrnLimits,
    scientific_style: XycePrnScientificStyle,
) -> Result<String, XycePrnError> {
    if !format.is_print_request {
        return Err(XycePrnError::NotPrintRequest);
    }
    let delimiter = format.delimiter.ok_or(XycePrnError::MissingDelimiter)?;
    if matches!(delimiter, XycePrnDelimiter::Separated(value) if value.is_empty()) {
        return Err(XycePrnError::EmptyCustomDelimiter);
    }
    let [first, ..] = tables else {
        return Err(XycePrnError::EmptySequence);
    };
    if first.columns.is_empty() {
        return Err(XycePrnError::EmptySchema);
    }

    let precision_raw = format.precision.unwrap_or(DEFAULT_SCIENTIFIC_PRECISION);
    let precision = usize::try_from(precision_raw)
        .map_err(|_| XycePrnError::NegativePrecision(precision_raw))?;
    let width_raw = format.width.unwrap_or(DEFAULT_FIELD_WIDTH);
    let effective_width = if i64::from(width_raw) - i64::from(precision_raw) < 9 {
        i64::from(precision_raw)
            .checked_add(9)
            .ok_or(XycePrnError::WidthOverflow)?
    } else {
        i64::from(width_raw)
    };
    let width = usize::try_from(effective_width).map_err(|_| XycePrnError::WidthOverflow)?;
    // `-1.234e+308` needs precision + 9 bytes. Check before formatting.
    let maximum_serialized_value = precision
        .checked_add(9)
        .ok_or(XycePrnError::PrecisionLengthOverflow)?;
    if maximum_serialized_value > limits.max_output_bytes {
        return Err(XycePrnError::PrecisionLimit {
            precision,
            maximum: limits.max_output_bytes,
        });
    }
    if delimiter.is_padded() && width > limits.max_output_bytes {
        return Err(XycePrnError::WidthLimit {
            width,
            maximum: limits.max_output_bytes,
        });
    }

    let mut total_rows = 0usize;
    for (table_index, table) in tables.iter().enumerate() {
        if table.columns != first.columns {
            return Err(XycePrnError::SchemaMismatch { table: table_index });
        }
        total_rows = total_rows
            .checked_add(table.rows.len())
            .ok_or(XycePrnError::RowCountOverflow)?;
    }
    if total_rows > limits.max_rows {
        return Err(XycePrnError::RowLimit {
            rows: total_rows,
            maximum: limits.max_rows,
        });
    }

    let separator = delimiter.separator();
    let padded = delimiter.is_padded();
    let mut output = String::new();
    if format.print_header.unwrap_or(true) {
        for (column_index, name) in first.columns.iter().enumerate() {
            let name = if column_index == 0 && name.eq_ignore_ascii_case("Index") {
                "Index"
            } else {
                name.as_str()
            };
            push_field_separator(
                &mut output,
                separator,
                column_index,
                limits.max_output_bytes,
            )?;
            if !padded || width <= name.len() {
                push_bounded(&mut output, name, limits.max_output_bytes)?;
            } else if column_index == 0 && name.eq_ignore_ascii_case("Index") {
                push_bounded(&mut output, &format!("{name:<5}"), limits.max_output_bytes)?;
            } else {
                let left = (width - name.len()) / 2;
                let right = width - name.len() - left;
                push_bounded(&mut output, &" ".repeat(left), limits.max_output_bytes)?;
                push_bounded(&mut output, name, limits.max_output_bytes)?;
                push_bounded(&mut output, &" ".repeat(right), limits.max_output_bytes)?;
            }
        }
        push_bounded(&mut output, "\n", limits.max_output_bytes)?;
    }

    for (table_index, table) in tables.iter().enumerate() {
        for (row_index, row) in table.rows.iter().enumerate() {
            if row.len() != table.columns.len() {
                return Err(XycePrnError::RowShape {
                    table: table_index,
                    row: row_index,
                    fields: row.len(),
                    columns: table.columns.len(),
                });
            }
            for (column_index, value) in row.iter().copied().enumerate() {
                push_field_separator(
                    &mut output,
                    separator,
                    column_index,
                    limits.max_output_bytes,
                )?;
                if column_index == 0 && table.columns[0].eq_ignore_ascii_case("Index") {
                    if !value.is_finite() || value < 0.0 || value.fract() != 0.0 {
                        return Err(XycePrnError::InvalidIndex {
                            table: table_index,
                            row: row_index,
                            value,
                        });
                    }
                    let field = if padded {
                        format!("{value:<5.0}")
                    } else {
                        format!("{value:.0}")
                    };
                    push_bounded(&mut output, &field, limits.max_output_bytes)?;
                } else {
                    if !value.is_finite() {
                        return Err(XycePrnError::NonFinite {
                            table: table_index,
                            row: row_index,
                            column: column_index,
                            value,
                        });
                    }
                    let serialized = format_xyce_prn_scientific(
                        value,
                        precision,
                        limits.max_output_bytes,
                        scientific_style,
                    )?;
                    if padded && width > serialized.len() {
                        push_bounded(
                            &mut output,
                            &" ".repeat(width - serialized.len()),
                            limits.max_output_bytes,
                        )?;
                    }
                    push_bounded(&mut output, &serialized, limits.max_output_bytes)?;
                }
            }
            push_bounded(&mut output, "\n", limits.max_output_bytes)?;
        }
    }

    if format.print_footer.unwrap_or(true) {
        let marker = match footer {
            XycePrnFooter::None => None,
            XycePrnFooter::Simulation => Some("End of Xyce(TM) Simulation\n"),
            XycePrnFooter::ParameterSweep => Some("End of Xyce(TM) Parameter Sweep\n"),
        };
        if let Some(marker) = marker {
            push_bounded(&mut output, marker, limits.max_output_bytes)?;
        }
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct TestRequest {
        is_print: bool,
        delimiter: Option<XycePrnDelimiter<'static>>,
        print_precision: Option<i32>,
        print_width: Option<i32>,
    }

    impl XycePrnRequest for TestRequest {
        fn xyce_prn_is_print_request(&self) -> bool {
            self.is_print
        }

        fn xyce_prn_delimiter(&self) -> Option<XycePrnDelimiter<'_>> {
            self.delimiter
        }

        fn xyce_prn_precision(&self) -> Option<i32> {
            self.print_precision
        }

        fn xyce_prn_width(&self) -> Option<i32> {
            self.print_width
        }
    }

    #[derive(Debug, Clone, Default)]
    struct TestOptions {
        output_print_header: Option<bool>,
        output_print_footer: Option<bool>,
    }

    impl XycePrnOutputOptions for TestOptions {
        fn xyce_prn_print_header(&self) -> Option<bool> {
            self.output_print_header
        }

        fn xyce_prn_print_footer(&self) -> Option<bool> {
            self.output_print_footer
        }
    }

    fn request() -> (TestRequest, TestOptions) {
        (
            TestRequest {
                is_print: true,
                delimiter: Some(XycePrnDelimiter::Whitespace),
                print_precision: Some(12),
                print_width: Some(21),
            },
            TestOptions::default(),
        )
    }

    fn table() -> XycePrnTable {
        XycePrnTable {
            columns: vec!["Index".into(), "TIME".into(), "V(A)".into()],
            rows: vec![vec![0.0, 0.0, 1.0], vec![1.0, 1.0e-9, 1.0]],
        }
    }

    #[test]
    fn exact_xyce_sequence_layout_preserves_local_indices() {
        let (request, options) = request();
        for spelling in ["Index", "INDEX", "index", "InDeX"] {
            let mut first = table();
            first.columns[0] = spelling.to_string();
            let second = first.clone();
            let text = serialize_xyce_prn_sequence(
                &[first, second],
                &request,
                &options,
                XycePrnFooter::ParameterSweep,
                XycePrnLimits::new(4, 10_000),
            )
            .unwrap();
            assert_eq!(
                text,
                "Index         TIME                  V(A)         \n0        0.000000000000e+00    1.000000000000e+00\n1        1.000000000000e-09    1.000000000000e+00\n0        0.000000000000e+00    1.000000000000e+00\n1        1.000000000000e-09    1.000000000000e+00\nEnd of Xyce(TM) Parameter Sweep\n"
            );
        }
    }

    #[test]
    fn compact_delimiter_layout_canonicalizes_index_header() {
        for spelling in ["INDEX", "index"] {
            for (delimiter, expected_header, expected_row) in [
                (
                    XycePrnDelimiter::Whitespace,
                    "Index TIME V(A)",
                    "0 0.00000000e0 1.00000000e0",
                ),
                (
                    XycePrnDelimiter::Separated(","),
                    "Index,TIME,V(A)",
                    "0,0.00000000e0,1.00000000e0",
                ),
                (
                    XycePrnDelimiter::Separated("|"),
                    "Index|TIME|V(A)",
                    "0|0.00000000e0|1.00000000e0",
                ),
            ] {
                let mut table = table();
                table.columns[0] = spelling.to_string();
                let text = serialize_legacy_compact_prn_for_comparison(
                    &table,
                    &delimiter,
                    XycePrnLimits::new(2, 10_000),
                )
                .unwrap();
                assert_eq!(text.lines().next(), Some(expected_header));
                assert_eq!(text.lines().nth(1), Some(expected_row));
                assert!(text.ends_with("End of Xyce(TM) Simulation\n"));
            }
        }
    }

    #[test]
    fn authored_header_footer_and_signed_layout_are_exact() {
        let (mut request, mut options) = request();
        request.print_precision = Some(17);
        request.print_width = Some(-1);
        options.output_print_header = Some(false);
        options.output_print_footer = Some(false);
        let text = serialize_xyce_prn_sequence(
            &[table()],
            &request,
            &options,
            XycePrnFooter::Simulation,
            XycePrnLimits::new(2, 10_000),
        )
        .unwrap();
        assert!(!text.contains("Index"));
        assert!(!text.contains("End of Xyce"));
        assert!(text.contains("0.00000000000000000e+00"));
    }

    #[test]
    fn schema_numeric_and_resource_failures_are_typed() {
        let (mut request, options) = request();
        let mut invalid = table();
        invalid.rows[0][1] = Value::NAN;
        assert!(matches!(
            serialize_xyce_prn_sequence(
                &[invalid],
                &request,
                &options,
                XycePrnFooter::None,
                XycePrnLimits::new(2, 10_000),
            ),
            Err(XycePrnError::NonFinite { .. })
        ));
        assert!(matches!(
            serialize_xyce_prn_sequence(
                &[table(), table()],
                &request,
                &options,
                XycePrnFooter::None,
                XycePrnLimits::new(3, 10_000),
            ),
            Err(XycePrnError::RowLimit { .. })
        ));
        request.print_precision = Some(10_000);
        assert!(matches!(
            serialize_xyce_prn_sequence(
                &[table()],
                &request,
                &options,
                XycePrnFooter::None,
                XycePrnLimits::new(2, 1_000),
            ),
            Err(XycePrnError::PrecisionLimit { .. })
        ));
        assert_eq!(
            format_xyce_prn_scientific(1.0e-9, 12, 21, XycePrnScientificStyle::Canonical,).unwrap(),
            "1.000000000000e-09"
        );
        assert!(matches!(
            format_xyce_prn_scientific(
                1.0,
                usize::MAX,
                usize::MAX,
                XycePrnScientificStyle::Canonical,
            ),
            Err(XycePrnError::PrecisionLengthOverflow)
        ));
        assert_eq!(
            format_xyce_prn_scientific(
                1.0e-9,
                8,
                17,
                XycePrnScientificStyle::LegacyRspiceComparison,
            )
            .unwrap(),
            "1.00000000e-9"
        );
    }
}

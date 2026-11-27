//! MAT-file Level 5, written by hand.
//!
//! The layout is the one MathWorks publishes in *MAT-File Format*. Three of
//! its sections are the whole of what is implemented here: *Level 5 MAT-File
//! Header Format* for the 128-byte preamble, *The Data Element* for the
//! tag-then-payload rule every element obeys, and *Numeric Array* — array
//! flags, dimensions, name, real part, optional imaginary part — for what
//! one variable looks like.
//!
//! # What is written
//!
//! One `miMATRIX` element per variable, each an `mxDOUBLE_CLASS` array of N
//! rows and one column, real or complex, named. Nothing else: no cell array,
//! no structure, no character matrix, no sparse array, and no `miCOMPRESSED`
//! wrapper. Compression is optional in the spec and a plain file is the one
//! every reader opens without a decompressor in front of it, so RSpice writes
//! plain files.
//!
//! Every element is written in the **long tag form** — four bytes of type,
//! four bytes of size — including the ones short enough for the optional
//! small-element form that *The Data Element* describes. The small form packs
//! the byte count into the upper half of the type word, which is the only
//! field in this layout whose meaning depends on reading it at the right
//! width. One shape everywhere costs at most four bytes per variable name and
//! leaves nothing to get wrong.
//!
//! # Determinism
//!
//! Nothing here reads a clock, a path, or an environment: the caller states
//! the header text. The same variables and the same text are the same bytes.

use std::collections::HashSet;
use std::fmt;
use std::io::{self, Write};

pub mod publication;
pub mod reader;
#[cfg(feature = "result-waveform")]
pub mod result;

/// *Level 5 MAT-File Header Format*: 116 bytes of descriptive text, 8 bytes
/// of subsystem-data offset, 2 bytes of version and 2 of endian indicator.
pub const HEADER_BYTES: usize = 128;

/// The descriptive text field of that header.
pub const HEADER_TEXT_BYTES: usize = 116;

/// The prefix MATLAB itself writes, and the signature RSpice's own importer
/// identifies a `.mat` file by. A file that does not open with it is not one
/// this product could read back.
pub const HEADER_SIGNATURE: &str = "MATLAB 5.0 MAT-file";

/// `namelengthmax`: the longest variable name MATLAB accepts.
pub const MAX_NAME_CHARS: usize = 63;

// *Data Types*. Only the five this layout uses are named.
const MI_INT8: u32 = 1;
const MI_INT32: u32 = 5;
const MI_UINT32: u32 = 6;
const MI_DOUBLE: u32 = 9;
const MI_MATRIX: u32 = 14;

// *Array Types (Classes)* and the array-flags bits above them.
const MX_STRUCT_CLASS: u32 = 2;
const MX_DOUBLE_CLASS: u32 = 6;
const MX_COMPLEX: u32 = 0x0800;

/// Every data element is padded so the next one starts on an 8-byte boundary.
const ALIGNMENT: usize = 8;

/// One variable: a column of doubles under a MATLAB name.
#[derive(Debug, Clone, PartialEq)]
pub struct MatVariable {
    pub name: String,
    pub real: Vec<f64>,
    /// `Some` makes the array complex. It must be exactly as long as `real`.
    pub imag: Option<Vec<f64>>,
}

/// What this writer refuses rather than write.
///
/// Each one is a file MATLAB would either reject or silently misread, which
/// is worse: a duplicate name loads as one variable, not two.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatWriteError {
    /// The header text does not begin with [`HEADER_SIGNATURE`].
    HeaderSignature,
    /// The header text is longer than the field that holds it.
    HeaderTextTooLong(usize),
    /// A name that is not a MATLAB identifier: a letter, then letters, digits
    /// and underscores.
    NotAnIdentifier(String),
    /// A name past `namelengthmax`.
    NameTooLong(String, usize),
    /// Two variables under one name. MATLAB would keep whichever it read
    /// last and never say so.
    DuplicateName(String),
    /// A complex array whose two halves are different lengths.
    UnbalancedComplex {
        name: String,
        real: usize,
        imag: usize,
    },
    /// More rows than the dimensions field, which is `miINT32`, can state.
    TooManyRows(String, usize),
    /// A variable of more bytes than its tag, which is `u32`, can state.
    TooLarge(String, u64),
    /// The destination refused a write.
    Write(String),
}

impl fmt::Display for MatWriteError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HeaderSignature => write!(
                formatter,
                "the header text must begin with '{HEADER_SIGNATURE}'"
            ),
            Self::HeaderTextTooLong(bytes) => write!(
                formatter,
                "the header text is {bytes} bytes; the field holds {HEADER_TEXT_BYTES}"
            ),
            Self::NotAnIdentifier(name) => write!(
                formatter,
                "'{name}' is not a MATLAB identifier: a name is a letter followed by letters, digits and underscores"
            ),
            Self::NameTooLong(name, characters) => write!(
                formatter,
                "'{name}' is {characters} characters; MATLAB accepts {MAX_NAME_CHARS}"
            ),
            Self::DuplicateName(name) => write!(
                formatter,
                "two variables are named '{name}'; MATLAB would load one of them"
            ),
            Self::UnbalancedComplex { name, real, imag } => write!(
                formatter,
                "'{name}' has {real} real values and {imag} imaginary ones"
            ),
            Self::TooManyRows(name, rows) => write!(
                formatter,
                "'{name}' has {rows} rows; a MAT dimension is a signed 32-bit count"
            ),
            Self::TooLarge(name, bytes) => write!(
                formatter,
                "'{name}' takes {bytes} bytes; a MAT v5 variable holds at most 4 GiB. Export \
                 HDF5, which has no such limit"
            ),
            Self::Write(reason) => write!(formatter, "the MAT file could not be written: {reason}"),
        }
    }
}

/// Whether `name` is a name MATLAB will accept back: not a reserved word either.
///
/// Length is checked separately so the refusal can say which rule was broken.
pub fn is_matlab_identifier(name: &str) -> bool {
    let mut characters = name.chars();
    characters
        .next()
        .is_some_and(|first| first.is_ascii_alphabetic())
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
        && !publication::MATLAB_KEYWORDS.contains(&name)
}

/// Serialize a Level 5 MAT-file.
pub fn write_mat_v5(
    header_text: &str,
    variables: &[MatVariable],
) -> Result<Vec<u8>, MatWriteError> {
    let rows = validate(header_text, variables)?;
    let mut bytes = Vec::new();
    write_header(&mut bytes, header_text).expect("writing to memory cannot fail");
    for (variable, rows) in variables.iter().zip(rows) {
        write_column_start(&mut bytes, &variable.name, rows, variable.imag.is_some())
            .expect("writing to memory cannot fail");
        write_values(&mut bytes, &variable.real);
        if let Some(imag) = &variable.imag {
            write_part_tag(&mut bytes, rows).expect("writing to memory cannot fail");
            write_values(&mut bytes, imag);
        }
    }
    Ok(bytes)
}

/// A dimension is written as `miINT32`, so a row count has to fit one.
pub(crate) fn row_dimension(name: &str, rows: usize) -> Result<i32, MatWriteError> {
    i32::try_from(rows).map_err(|_| MatWriteError::TooManyRows(name.to_owned(), rows))
}

/// Check every rule this writer refuses on, and return the row dimension of
/// each variable so the element writer never has to cast one itself.
fn validate(header_text: &str, variables: &[MatVariable]) -> Result<Vec<i32>, MatWriteError> {
    if !header_text.starts_with(HEADER_SIGNATURE) {
        return Err(MatWriteError::HeaderSignature);
    }
    if header_text.len() > HEADER_TEXT_BYTES {
        return Err(MatWriteError::HeaderTextTooLong(header_text.len()));
    }
    // MATLAB's own uniqueness rule, which is case-sensitive. A caller that
    // needs a stricter one — RSpice's importer refuses two signals whose
    // names differ only in case — applies it before handing variables here.
    let mut names = HashSet::with_capacity(variables.len());
    let mut rows = Vec::with_capacity(variables.len());
    for variable in variables {
        let characters = variable.name.chars().count();
        if characters > MAX_NAME_CHARS {
            return Err(MatWriteError::NameTooLong(
                variable.name.clone(),
                characters,
            ));
        }
        if !is_matlab_identifier(&variable.name) {
            return Err(MatWriteError::NotAnIdentifier(variable.name.clone()));
        }
        if !names.insert(variable.name.as_str()) {
            return Err(MatWriteError::DuplicateName(variable.name.clone()));
        }
        if let Some(imag) = &variable.imag
            && imag.len() != variable.real.len()
        {
            return Err(MatWriteError::UnbalancedComplex {
                name: variable.name.clone(),
                real: variable.real.len(),
                imag: imag.len(),
            });
        }
        rows.push(row_dimension(&variable.name, variable.real.len())?);
        column_size(&variable.name, variable.real.len(), variable.imag.is_some())?;
    }
    Ok(rows)
}

/// *Level 5 MAT-File Header Format*, for a header text no longer than its field.
pub(crate) fn write_header(out: &mut dyn Write, header_text: &str) -> io::Result<()> {
    let mut bytes = Vec::with_capacity(HEADER_BYTES);
    let text = header_text.as_bytes();
    bytes.extend_from_slice(text);
    // The text field is space-padded, as MATLAB writes it.
    bytes.resize(bytes.len() + (HEADER_TEXT_BYTES - text.len()), b' ');
    // Subsystem data offset. All zeros says the file has no subsystem data,
    // which is the only kind this writer produces.
    bytes.extend_from_slice(&[0_u8; 8]);
    // Version 0x0100, then the endian indicator: the characters M and I as a
    // 16-bit value, which on a little-endian file are the bytes 'I', 'M'.
    bytes.extend_from_slice(&0x0100_u16.to_le_bytes());
    bytes.extend_from_slice(b"IM");
    debug_assert_eq!(bytes.len(), HEADER_BYTES);
    out.write_all(&bytes)
}

/// Bytes an element's payload is padded by to reach the next 8-byte boundary.
const fn padding(length: u64) -> u64 {
    (ALIGNMENT as u64 - length % ALIGNMENT as u64) % ALIGNMENT as u64
}

/// The bytes of one numeric column's element after its tag: array flags, dimensions, name, the
/// real part and, when complex, the imaginary part.
fn column_payload_bytes(name: &str, rows: usize, complex: bool) -> u64 {
    let name = name.len() as u64;
    let part = ALIGNMENT as u64 + rows as u64 * 8;
    // Array flags and dimensions are two 8-byte subelements each.
    16 + 16 + (ALIGNMENT as u64 + name + padding(name)) + if complex { 2 * part } else { part }
}

/// The size a column `name` of `rows` values states in its tag, refused when it is more than
/// the `u32` the tag holds.
pub(crate) fn column_size(name: &str, rows: usize, complex: bool) -> Result<u64, MatWriteError> {
    let bytes = column_payload_bytes(name, rows, complex);
    if bytes > u64::from(u32::MAX) {
        return Err(MatWriteError::TooLarge(name.to_owned(), bytes));
    }
    Ok(bytes)
}

/// *The Data Element*, long form: type then byte count, both `u32`.
fn write_tag(out: &mut dyn Write, data_type: u32, byte_size: u64) -> io::Result<()> {
    let size = u32::try_from(byte_size)
        .map_err(|_| io::Error::other("a MAT element is larger than its tag can state"))?;
    out.write_all(&data_type.to_le_bytes())?;
    out.write_all(&size.to_le_bytes())
}

/// The array flags, dimensions and name of an array of `rows` x `columns` of `class`.
fn write_array_head(
    out: &mut dyn Write,
    class: u32,
    rows: i32,
    columns: i32,
    name: &str,
) -> io::Result<()> {
    write_tag(out, MI_UINT32, 8)?;
    out.write_all(&class.to_le_bytes())?;
    // nzmax is for sparse arrays only.
    out.write_all(&0_u32.to_le_bytes())?;
    write_tag(out, MI_INT32, 8)?;
    out.write_all(&rows.to_le_bytes())?;
    out.write_all(&columns.to_le_bytes())?;
    let name = name.as_bytes();
    write_tag(out, MI_INT8, name.len() as u64)?;
    out.write_all(name)?;
    out.write_all(&[0; ALIGNMENT][..padding(name.len() as u64) as usize])
}

/// One numeric column's `miMATRIX` up to its real values: the element's tag, array flags,
/// dimensions, name and the real part's tag. Its `rows` values follow as little-endian `f64`,
/// then, when complex, [`write_part_tag`] and the imaginary values.
///
/// A column vector: one value per row, one column. Two dimensions is the fewest the spec
/// allows, and the shape every MATLAB reader treats as a signal rather than a matrix.
pub(crate) fn write_column_start(
    out: &mut dyn Write,
    name: &str,
    rows: i32,
    complex: bool,
) -> io::Result<()> {
    let count = usize::try_from(rows).unwrap_or_default();
    write_tag(out, MI_MATRIX, column_payload_bytes(name, count, complex))?;
    let class = MX_DOUBLE_CLASS | if complex { MX_COMPLEX } else { 0 };
    write_array_head(out, class, rows, 1, name)?;
    write_part_tag(out, rows)
}

/// The tag of one part of a column of `rows` values: every double is eight bytes, so the part
/// is always aligned.
pub(crate) fn write_part_tag(out: &mut dyn Write, rows: i32) -> io::Result<()> {
    write_tag(out, MI_DOUBLE, u64::try_from(rows).unwrap_or_default() * 8)
}

fn write_values(bytes: &mut Vec<u8>, values: &[f64]) {
    for value in values {
        bytes.extend_from_slice(&value.to_le_bytes());
    }
}

/// The width each field name of a structure takes: `namelengthmax` and its terminating NUL.
const FIELD_NAME_BYTES: u64 = MAX_NAME_CHARS as u64 + 1;

/// The size a structure `name` of numeric column `fields` (name, rows, complex) states in its
/// tag, refused when it is more than the `u32` the tag holds.
pub(crate) fn struct_size(name: &str, fields: &[(&str, usize, bool)]) -> Result<u64, MatWriteError> {
    let name_bytes = name.len() as u64;
    let names = fields.len() as u64 * FIELD_NAME_BYTES;
    let bytes = 16
        + 16
        + (ALIGNMENT as u64 + name_bytes + padding(name_bytes))
        + 16
        + (ALIGNMENT as u64 + names + padding(names))
        + fields
            .iter()
            .map(|(_, rows, complex)| ALIGNMENT as u64 + column_payload_bytes("", *rows, *complex))
            .sum::<u64>();
    if bytes > u64::from(u32::MAX) {
        return Err(MatWriteError::TooLarge(name.to_owned(), bytes));
    }
    Ok(bytes)
}

/// One 1x1 structure's `miMATRIX` up to its fields: the element's tag, array flags, dimensions,
/// name, field name length and field names (*MAT-File Format*, structure arrays). Each field
/// follows as a column ([`write_column_start`] with an empty name, then its values) in the
/// order `fields` names them.
pub(crate) fn write_struct_start(
    out: &mut dyn Write,
    name: &str,
    fields: &[(&str, usize, bool)],
) -> Result<(), MatWriteError> {
    let size = struct_size(name, fields)?;
    let write = |out: &mut dyn Write| -> io::Result<()> {
        write_tag(out, MI_MATRIX, size)?;
        write_array_head(out, MX_STRUCT_CLASS, 1, 1, name)?;
        write_tag(out, MI_INT32, 4)?;
        out.write_all(&(FIELD_NAME_BYTES as i32).to_le_bytes())?;
        out.write_all(&[0; 4])?;
        let names = fields.len() as u64 * FIELD_NAME_BYTES;
        write_tag(out, MI_INT8, names)?;
        for (field, ..) in fields {
            let mut cell = field.as_bytes().to_vec();
            cell.resize(FIELD_NAME_BYTES as usize, 0);
            out.write_all(&cell)?;
        }
        out.write_all(&[0; ALIGNMENT][..padding(names) as usize])
    };
    write(out).map_err(|error| MatWriteError::Write(error.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEXT: &str = "MATLAB 5.0 MAT-file, Platform: RSpice, Created on: 2026-01-02T03:04:05Z";

    fn real(name: &str, values: &[f64]) -> MatVariable {
        MatVariable {
            name: name.to_owned(),
            real: values.to_vec(),
            imag: None,
        }
    }

    #[test]
    fn the_header_is_the_128_bytes_the_specification_lays_out() {
        let bytes = write_mat_v5(TEXT, &[real("time", &[0.0, 1.0])]).expect("writes");
        // Level 5 MAT-File Header Format: 116 bytes of text, space-padded.
        assert_eq!(&bytes[..TEXT.len()], TEXT.as_bytes());
        assert!(
            bytes[TEXT.len()..HEADER_TEXT_BYTES]
                .iter()
                .all(|byte| *byte == b' ')
        );
        // Subsystem data offset: unused, so all zeros.
        assert_eq!(&bytes[HEADER_TEXT_BYTES..124], &[0_u8; 8]);
        // Version 0x0100, little-endian.
        assert_eq!(&bytes[124..126], &[0x00, 0x01]);
        // Endian indicator: 'M' and 'I' as a 16-bit value, little-endian.
        assert_eq!(&bytes[126..128], b"IM");
    }

    #[test]
    fn a_two_sample_variable_is_the_bytes_the_element_layout_names() {
        let bytes = write_mat_v5(TEXT, &[real("time", &[0.0, 1.0])]).expect("writes");
        let element = &bytes[HEADER_BYTES..];
        // The Data Element, long form: miMATRIX and its payload size.
        assert_eq!(&element[0..4], &14_u32.to_le_bytes());
        // 16 of array flags, 16 of dimensions, 16 of padded name, 24 of data.
        assert_eq!(&element[4..8], &72_u32.to_le_bytes());
        // Array flags: miUINT32, eight bytes, mxDOUBLE_CLASS, nzmax zero.
        assert_eq!(&element[8..12], &6_u32.to_le_bytes());
        assert_eq!(&element[12..16], &8_u32.to_le_bytes());
        assert_eq!(&element[16..20], &6_u32.to_le_bytes());
        assert_eq!(&element[20..24], &0_u32.to_le_bytes());
        // Dimensions: miINT32, eight bytes, two rows by one column.
        assert_eq!(&element[24..28], &5_u32.to_le_bytes());
        assert_eq!(&element[28..32], &8_u32.to_le_bytes());
        assert_eq!(&element[32..36], &2_i32.to_le_bytes());
        assert_eq!(&element[36..40], &1_i32.to_le_bytes());
        // Array name: miINT8, four characters, padded to eight bytes.
        assert_eq!(&element[40..44], &1_u32.to_le_bytes());
        assert_eq!(&element[44..48], &4_u32.to_le_bytes());
        assert_eq!(&element[48..52], b"time");
        assert_eq!(&element[52..56], &[0, 0, 0, 0]);
        // Real part: miDOUBLE, sixteen bytes, two little-endian doubles.
        assert_eq!(&element[56..60], &9_u32.to_le_bytes());
        assert_eq!(&element[60..64], &16_u32.to_le_bytes());
        assert_eq!(&element[64..72], &0.0_f64.to_le_bytes());
        assert_eq!(&element[72..80], &1.0_f64.to_le_bytes());
        assert_eq!(element.len(), 80);
    }

    #[test]
    fn a_name_that_needs_no_padding_is_not_padded() {
        // Eight characters: the name subelement lands exactly on the
        // boundary, so nothing is added after it.
        let bytes = write_mat_v5(TEXT, &[real("abcdefgh", &[1.0])]).expect("writes");
        let element = &bytes[HEADER_BYTES..];
        assert_eq!(&element[44..48], &8_u32.to_le_bytes());
        assert_eq!(&element[48..56], b"abcdefgh");
        assert_eq!(&element[56..60], &9_u32.to_le_bytes());
    }

    #[test]
    fn a_complex_variable_sets_the_flag_and_writes_both_halves() {
        let variable = MatVariable {
            name: "v".to_owned(),
            real: vec![1.0, 2.0],
            imag: Some(vec![-1.0, -2.0]),
        };
        let bytes = write_mat_v5(TEXT, &[variable]).expect("writes");
        let element = &bytes[HEADER_BYTES..];
        // mxCOMPLEX above mxDOUBLE_CLASS.
        assert_eq!(&element[16..20], &(0x0800_u32 | 6).to_le_bytes());
        // A one-character name still occupies a whole padded subelement.
        assert_eq!(&element[44..48], &1_u32.to_le_bytes());
        assert_eq!(&element[48..49], b"v");
        // Real part, then imaginary part, each its own miDOUBLE subelement.
        assert_eq!(&element[56..60], &9_u32.to_le_bytes());
        assert_eq!(&element[60..64], &16_u32.to_le_bytes());
        assert_eq!(&element[64..72], &1.0_f64.to_le_bytes());
        assert_eq!(&element[72..80], &2.0_f64.to_le_bytes());
        assert_eq!(&element[80..84], &9_u32.to_le_bytes());
        assert_eq!(&element[88..96], &(-1.0_f64).to_le_bytes());
        assert_eq!(&element[96..104], &(-2.0_f64).to_le_bytes());
        assert_eq!(element.len(), 104);
    }

    #[test]
    fn every_element_starts_on_an_eight_byte_boundary() {
        let bytes = write_mat_v5(
            TEXT,
            &[
                real("a", &[1.0]),
                real("abcde", &[1.0, 2.0]),
                real("abcdefghijkl", &[1.0]),
            ],
        )
        .expect("writes");
        assert_eq!(bytes.len() % ALIGNMENT, 0);
        let mut offset = HEADER_BYTES;
        while offset < bytes.len() {
            assert_eq!(offset % ALIGNMENT, 0, "element at {offset}");
            let size = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap());
            offset += ALIGNMENT + size as usize;
        }
        assert_eq!(offset, bytes.len());
    }

    #[test]
    fn the_same_variables_write_the_same_bytes_twice() {
        let variables = [real("time", &[0.0, 1.0]), real("v", &[2.0, 3.0])];
        assert_eq!(
            write_mat_v5(TEXT, &variables).expect("writes"),
            write_mat_v5(TEXT, &variables).expect("writes")
        );
    }

    #[test]
    fn a_header_text_without_the_signature_is_refused() {
        let error = write_mat_v5("RSpice waveforms", &[real("time", &[0.0])])
            .expect_err("the importer identifies a .mat file by its signature");
        assert_eq!(error, MatWriteError::HeaderSignature);
        assert!(error.to_string().contains("MATLAB 5.0 MAT-file"));
    }

    #[test]
    fn a_header_text_past_the_field_is_refused_rather_than_cut() {
        let text = format!("{HEADER_SIGNATURE}{}", "x".repeat(HEADER_TEXT_BYTES));
        let error =
            write_mat_v5(&text, &[real("time", &[0.0])]).expect_err("the field holds 116 bytes");
        assert_eq!(
            error,
            MatWriteError::HeaderTextTooLong(HEADER_SIGNATURE.len() + HEADER_TEXT_BYTES)
        );
    }

    #[test]
    fn a_name_that_is_not_an_identifier_is_refused() {
        for name in ["", "1st", "_leading", "V(out)", "with space", "sig-nal"] {
            let error = write_mat_v5(TEXT, &[real(name, &[0.0])])
                .expect_err("MATLAB would not accept this name");
            assert_eq!(error, MatWriteError::NotAnIdentifier(name.to_owned()));
        }
        for name in ["a", "V_out_", "x1", "A_b_9"] {
            assert!(is_matlab_identifier(name), "{name}");
        }
    }

    #[test]
    fn a_name_past_namelengthmax_is_refused() {
        let name = "v".repeat(MAX_NAME_CHARS + 1);
        let error =
            write_mat_v5(TEXT, &[real(&name, &[0.0])]).expect_err("MATLAB accepts 63 characters");
        assert_eq!(error, MatWriteError::NameTooLong(name, MAX_NAME_CHARS + 1));
        assert!(write_mat_v5(TEXT, &[real(&"v".repeat(MAX_NAME_CHARS), &[0.0])]).is_ok());
    }

    #[test]
    fn two_variables_under_one_name_are_refused_rather_than_silently_merged() {
        let error = write_mat_v5(TEXT, &[real("v", &[0.0]), real("v", &[1.0])])
            .expect_err("MATLAB would load one of them");
        assert_eq!(error, MatWriteError::DuplicateName("v".to_owned()));
        // MATLAB's own rule is case-sensitive, so these are two variables.
        assert!(write_mat_v5(TEXT, &[real("v", &[0.0]), real("V", &[1.0])]).is_ok());
    }

    #[test]
    fn a_row_count_past_a_signed_32_bit_dimension_is_refused() {
        // The variable this refuses would be 17 GB of doubles, so the rule is
        // exercised on the dimension itself rather than on a written file.
        assert_eq!(row_dimension("v", 1_000_000), Ok(1_000_000));
        assert_eq!(row_dimension("v", i32::MAX as usize), Ok(i32::MAX));
        let rows = i32::MAX as usize + 1;
        assert_eq!(
            row_dimension("v", rows),
            Err(MatWriteError::TooManyRows("v".to_owned(), rows))
        );
    }

    #[test]
    fn a_complex_variable_with_unequal_halves_is_refused() {
        let variable = MatVariable {
            name: "v".to_owned(),
            real: vec![1.0, 2.0],
            imag: Some(vec![-1.0]),
        };
        let error = write_mat_v5(TEXT, &[variable]).expect_err("a complex array has two halves");
        assert_eq!(
            error,
            MatWriteError::UnbalancedComplex {
                name: "v".to_owned(),
                real: 2,
                imag: 1,
            }
        );
    }
}

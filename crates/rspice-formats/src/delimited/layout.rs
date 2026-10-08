//! Optional final record for ambiguous real/complex waveform column names.
//!
//! Numeric tables normally infer adjacent `Re(x)`/`Im(x)` columns as complex.
//! When those labels instead name independent real signals, a rectangular
//! layout record after the samples preserves their representation without
//! changing either the header labels or the numeric cells. Unmarked files keep
//! their historical inference rules. Consumers displaying individual physical
//! columns can validate and omit the record without combining any samples.

/// Coordinate cell of a version-one layout record; all other cells name kinds.
pub const RECORD_MARKER: &str = "# RSpiceTableLayoutV1";

#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ColumnKind {
    Real,
    ComplexReal,
    ComplexImag,
}

impl ColumnKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Real => "real",
            Self::ComplexReal => "complex_real",
            Self::ComplexImag => "complex_imag",
        }
    }
}

/// Include unknown versions so callers refuse rather than ignore them.
pub fn is_layout_record(first_cell: &str) -> bool {
    first_cell.starts_with("# RSpiceTableLayout")
}

/// The signal name of a matching adjacent pair, preserving its exact spelling.
pub fn complex_pair_name<'a>(real: &'a str, imag: &str) -> Option<&'a str> {
    let name = real.strip_prefix("Re(")?.strip_suffix(')')?;
    (imag.strip_prefix("Im(")?.strip_suffix(')')? == name).then_some(name)
}

/// Infer the traditional layout from signal headers (excluding the coordinate).
pub fn infer_layout(headers: &[impl AsRef<str>]) -> Vec<ColumnKind> {
    let mut kinds = vec![ColumnKind::Real; headers.len()];
    let mut index = 0;
    while index + 1 < headers.len() {
        if complex_pair_name(headers[index].as_ref(), headers[index + 1].as_ref()).is_some() {
            kinds[index] = ColumnKind::ComplexReal;
            kinds[index + 1] = ColumnKind::ComplexImag;
            index += 2;
        } else {
            index += 1;
        }
    }
    kinds
}

/// Decode an optional layout record against signal headers, excluding the
/// coordinate header. `fields` includes the marker in the coordinate position.
/// Callers must also enforce that a layout record is unique and final.
pub fn parse_layout_record(
    headers: &[impl AsRef<str>],
    fields: &[impl AsRef<str>],
) -> Result<Option<Vec<ColumnKind>>, String> {
    let Some(marker) = fields
        .first()
        .map(AsRef::as_ref)
        .filter(|s| is_layout_record(s))
    else {
        return Ok(None);
    };
    if marker != RECORD_MARKER {
        return Err(format!("unsupported table layout marker {marker:?}"));
    }
    if fields.len() != headers.len() + 1 {
        return Err(format!(
            "table layout has {} columns; expected {} columns",
            fields.len(),
            headers.len() + 1
        ));
    }
    let kinds = fields[1..]
        .iter()
        .map(|field| match field.as_ref() {
            "real" => Ok(ColumnKind::Real),
            "complex_real" => Ok(ColumnKind::ComplexReal),
            "complex_imag" => Ok(ColumnKind::ComplexImag),
            other => Err(format!("unknown table column kind {other:?}")),
        })
        .collect::<Result<Vec<_>, _>>()?;
    validate_layout(headers, &kinds)?;
    Ok(Some(kinds))
}

pub(super) fn validate_layout(
    headers: &[impl AsRef<str>],
    kinds: &[ColumnKind],
) -> Result<(), String> {
    if headers.len() != kinds.len() {
        return Err("table layout column count does not match its headers".into());
    }
    let mut index = 0;
    while index < kinds.len() {
        match kinds[index] {
            ColumnKind::Real => index += 1,
            ColumnKind::ComplexReal
                if kinds.get(index + 1) == Some(&ColumnKind::ComplexImag)
                    && complex_pair_name(headers[index].as_ref(), headers[index + 1].as_ref())
                        .is_some() =>
            {
                index += 2;
            }
            _ => {
                return Err(format!(
                    "table layout column {} must belong to an adjacent complex_real/complex_imag pair with matching Re(...)/Im(...) headers",
                    index + 2
                ));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_layout_overrides_only_the_inferred_representation() {
        let headers = ["Re(x)", "Im(x)", "Re(Re(a,\"b\"))", "Im(Re(a,\"b\"))"];
        let legacy = [
            ColumnKind::ComplexReal,
            ColumnKind::ComplexImag,
            ColumnKind::ComplexReal,
            ColumnKind::ComplexImag,
        ];
        assert_eq!(infer_layout(&headers), legacy);
        let fields = [
            RECORD_MARKER,
            "real",
            "real",
            "complex_real",
            "complex_imag",
        ];
        assert_eq!(
            parse_layout_record(&headers, &fields).unwrap().unwrap(),
            [
                ColumnKind::Real,
                ColumnKind::Real,
                ColumnKind::ComplexReal,
                ColumnKind::ComplexImag
            ]
        );
        assert_eq!(
            parse_layout_record(&headers, &["0", "1", "2", "3", "4"]).unwrap(),
            None
        );
        assert_eq!(
            infer_layout(&["Re(x)", "Im(X)", "re(z)", "im(z)"]),
            vec![ColumnKind::Real; 4]
        );
    }

    #[test]
    fn declarations_require_valid_versions_kinds_widths_and_matching_names() {
        for fields in [
            vec!["# RSpiceTableLayoutV2", "real", "real"],
            vec![RECORD_MARKER, "real"],
            vec![RECORD_MARKER, "real", "real", "real"],
            vec![RECORD_MARKER, "unknown", "real"],
            vec![RECORD_MARKER, "complex_imag", "complex_real"],
            vec![RECORD_MARKER, "complex_real", "real"],
        ] {
            assert!(
                parse_layout_record(&["Re(x)", "Im(x)"], &fields).is_err(),
                "{fields:?}"
            );
        }
        for headers in [["Re(x)", "Im(y)"], ["x", "y"]] {
            assert!(
                parse_layout_record(&headers, &[RECORD_MARKER, "complex_real", "complex_imag"])
                    .is_err()
            );
        }
    }
}

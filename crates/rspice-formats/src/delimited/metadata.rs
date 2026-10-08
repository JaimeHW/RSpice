//! Versioned quantity metadata for numeric CSV/TSV tables.
//!
//! The final record starts with a plain comment marker. Its second cell is
//! compact JSON; remaining cells are empty, retaining the table's rectangular
//! shape (a coordinate-only table uses two cells). Numeric cells are unchanged.
//! Older readers refuse the new layout version instead of losing its units.

use super::layout::{ColumnKind, parse_layout_record, validate_layout};
use serde::{Deserialize, Serialize};

pub const RECORD_MARKER: &str = "# RSpiceTableLayoutV2";

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ColumnMetadata {
    pub name: String,
    pub kind: ColumnKind,
    pub quantity: Option<String>,
    pub unit: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TableMetadata {
    pub analysis: Option<String>,
    pub title: Option<String>,
    /// Coordinate first, followed by physical signal columns in header order.
    pub columns: Vec<ColumnMetadata>,
}

#[derive(Debug)]
pub struct TableLayout {
    /// Signal representations, excluding the real coordinate.
    pub kinds: Vec<ColumnKind>,
    pub metadata: Option<TableMetadata>,
}

impl TableMetadata {
    pub fn validate(&self, headers: &[impl AsRef<str>]) -> Result<(), String> {
        if headers.is_empty() || headers.len() != self.columns.len() {
            return Err("table metadata column count does not match its headers".into());
        }
        for (index, (header, column)) in headers.iter().zip(&self.columns).enumerate() {
            if header.as_ref() != column.name {
                return Err(format!(
                    "table metadata column {} does not match its header",
                    index + 1
                ));
            }
            if let Some(unit) = &column.unit
                && (unit.trim().is_empty() || unit.chars().any(char::is_control))
            {
                return Err(format!(
                    "table column {} unit must be nonempty and control-free",
                    index + 1
                ));
            }
        }
        if self.columns[0].kind != ColumnKind::Real {
            return Err("table coordinate must be real".into());
        }
        let kinds = self.columns[1..]
            .iter()
            .map(|column| column.kind)
            .collect::<Vec<_>>();
        validate_layout(&headers[1..], &kinds)?;
        for pair in self.columns[1..].windows(2) {
            if pair[0].kind == ColumnKind::ComplexReal
                && (pair[0].unit != pair[1].unit || pair[0].quantity != pair[1].quantity)
            {
                return Err(
                    "complex table components must declare matching quantities and units".into(),
                );
            }
        }
        Ok(())
    }

    /// Prepare all metadata before a writer publishes any output.
    pub fn record(&self, headers: &[impl AsRef<str>]) -> Result<Vec<String>, String> {
        self.validate(headers)?;
        let mut fields = vec![String::new(); headers.len().max(2)];
        fields[0] = RECORD_MARKER.to_owned();
        fields[1] = serde_json::to_string(self).map_err(|error| error.to_string())?;
        Ok(fields)
    }
}

/// Read either historical layout-only metadata or the quantity-aware version.
/// Callers enforce that the record is unique, final, and does not count as data.
pub fn parse_table_record(
    headers: &[impl AsRef<str>],
    fields: &[impl AsRef<str>],
) -> Result<Option<TableLayout>, String> {
    if fields.first().map(AsRef::as_ref) != Some(RECORD_MARKER) {
        return parse_layout_record(headers.get(1..).unwrap_or_default(), fields).map(|kinds| {
            kinds.map(|kinds| TableLayout {
                kinds,
                metadata: None,
            })
        });
    }
    if fields.len() != headers.len().max(2)
        || fields[2..].iter().any(|field| !field.as_ref().is_empty())
    {
        return Err("table metadata record must have the header width, a JSON payload, and empty trailing cells".into());
    }
    // Count declarations before allocating their strings. IgnoredAny has zero
    // size, so this Vec tracks only a length, even for an oversized array.
    // The complete decode below retains the strict schema/duplicate checks.
    #[derive(Deserialize)]
    struct ColumnCount {
        columns: Vec<serde::de::IgnoredAny>,
    }
    let count: ColumnCount = serde_json::from_str(fields[1].as_ref())
        .map_err(|error| format!("invalid table metadata: {error}"))?;
    if count.columns.len() != headers.len() {
        return Err("table metadata column count does not match its headers".into());
    }
    let metadata: TableMetadata = serde_json::from_str(fields[1].as_ref())
        .map_err(|error| format!("invalid table metadata: {error}"))?;
    metadata.validate(headers)?;
    Ok(Some(TableLayout {
        kinds: metadata.columns[1..]
            .iter()
            .map(|column| column.kind)
            .collect(),
        metadata: Some(metadata),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture() -> TableMetadata {
        TableMetadata {
            analysis: Some("ac1".into()),
            title: Some("Exact units".into()),
            columns: vec![
                ColumnMetadata {
                    name: "frequency".into(),
                    kind: ColumnKind::Real,
                    quantity: Some("frequency".into()),
                    unit: Some("MHz".into()),
                },
                ColumnMetadata {
                    name: "Re(gain)".into(),
                    kind: ColumnKind::ComplexReal,
                    quantity: Some("value".into()),
                    unit: Some("V/sqrt(Hz)".into()),
                },
                ColumnMetadata {
                    name: "Im(gain)".into(),
                    kind: ColumnKind::ComplexImag,
                    quantity: Some("value".into()),
                    unit: Some("V/sqrt(Hz)".into()),
                },
            ],
        }
    }

    #[test]
    fn quantity_metadata_preserves_exact_symbols_and_binds_physical_columns() {
        let headers = ["frequency", "Re(gain)", "Im(gain)"];
        let metadata = fixture();
        let record = metadata.record(&headers).unwrap();
        assert_eq!(record.len(), headers.len());
        let layout = parse_table_record(&headers, &record).unwrap().unwrap();
        assert_eq!(
            layout.kinds,
            [ColumnKind::ComplexReal, ColumnKind::ComplexImag]
        );
        assert_eq!(layout.metadata, Some(metadata.clone()));
        for index in 0..metadata.columns.len() {
            let mut changed = metadata.clone();
            changed.columns[index].name.push('x');
            assert!(changed.record(&headers).is_err());
        }
        for unit in [None, Some("A".into()), Some("".into()), Some("V\n".into())] {
            let mut changed = metadata.clone();
            changed.columns[2].unit = unit;
            assert!(changed.record(&headers).is_err());
        }
        let mut invalid = record;
        invalid[2] = "extra".into();
        assert!(parse_table_record(&headers, &invalid).is_err());
    }

    #[test]
    fn older_layouts_and_coordinate_only_tables_remain_explicit() {
        let old = parse_table_record(
            &["time", "x"],
            &[super::super::layout::RECORD_MARKER, "real"],
        )
        .unwrap()
        .unwrap();
        assert_eq!(old.kinds, [ColumnKind::Real]);
        assert!(old.metadata.is_none());
        let mut metadata = fixture();
        metadata.columns.truncate(1);
        let record = metadata.record(&["frequency"]).unwrap();
        assert_eq!(record.len(), 2);
        assert_eq!(
            parse_table_record(&["frequency"], &record)
                .unwrap()
                .unwrap()
                .metadata,
            Some(metadata)
        );
        for fields in [
            vec![RECORD_MARKER],
            vec![RECORD_MARKER, "{}"],
            vec!["# RSpiceTableLayoutV3", "{}"],
        ] {
            assert!(parse_table_record(&["time"], &fields).is_err());
        }
    }

    #[test]
    fn metadata_count_is_checked_before_column_declarations_are_materialized() {
        let payload = format!("{{\"columns\":[{}]}}", vec!["null"; 10_000].join(","));
        let error = parse_table_record(&["time", "x"], &[RECORD_MARKER, &payload]).unwrap_err();
        assert!(error.contains("column count"), "{error}");
    }
}

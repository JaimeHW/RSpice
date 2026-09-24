//! Arrow IPC and Parquet codecs for numeric results and selected tables.

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

/// A selected table view supplied to the Parquet byte writer.
pub trait ParquetTableSource {
    fn column_count(&self) -> usize;
    fn row_count(&self) -> usize;
    fn column_id(&self, column: usize) -> &str;
    fn column_label(&self, column: usize) -> &str;
    fn column_unit(&self, column: usize) -> Option<&str>;
    fn numeric_value(&self, row: usize, column: usize) -> Option<f64>;
    fn display_value(&self, row: usize, column: usize) -> Option<&str>;
}

/// Encode an already-selected table as Parquet, preserving nullable numeric
/// and text columns and caller-supplied provenance metadata.
pub fn encode_parquet_table(
    source: &impl ParquetTableSource,
    metadata: Option<Vec<(String, Option<String>)>>,
) -> Result<Vec<u8>, String> {
    use arrow_array::{ArrayRef, Float64Array, RecordBatch, StringArray};
    use arrow_schema::{DataType, Field, Schema};
    use parquet::arrow::ArrowWriter;
    use parquet::file::metadata::KeyValue;
    use parquet::file::properties::WriterProperties;

    let fields = (0..source.column_count())
        .map(|column| {
            Field::new(
                source.column_id(column),
                if (0..source.row_count()).any(|row| source.numeric_value(row, column).is_some()) {
                    DataType::Float64
                } else {
                    DataType::Utf8
                },
                true,
            )
            .with_metadata(
                [
                    ("label".to_owned(), source.column_label(column).to_owned()),
                    (
                        "unit".to_owned(),
                        source.column_unit(column).unwrap_or_default().to_owned(),
                    ),
                ]
                .into_iter()
                .collect(),
            )
        })
        .collect::<Vec<_>>();
    let schema = Arc::new(Schema::new(fields));
    let arrays = (0..source.column_count())
        .map(|column| {
            if schema
                .field_with_name(source.column_id(column))
                .is_ok_and(|field| field.data_type() == &DataType::Float64)
            {
                Arc::new(Float64Array::from(
                    (0..source.row_count())
                        .map(|row| source.numeric_value(row, column))
                        .collect::<Vec<_>>(),
                )) as ArrayRef
            } else {
                Arc::new(StringArray::from(
                    (0..source.row_count())
                        .map(|row| source.display_value(row, column))
                        .collect::<Vec<_>>(),
                )) as ArrayRef
            }
        })
        .collect::<Vec<_>>();
    let batch = RecordBatch::try_new(schema.clone(), arrays).map_err(|error| error.to_string())?;
    let metadata = metadata.map(|items| {
        items
            .into_iter()
            .map(|(key, value)| KeyValue { key, value })
            .collect()
    });
    let properties = WriterProperties::builder()
        .set_key_value_metadata(metadata)
        .build();
    let mut writer = ArrowWriter::try_new(Vec::new(), schema, Some(properties))
        .map_err(|error| error.to_string())?;
    writer.write(&batch).map_err(|error| error.to_string())?;
    writer.into_inner().map_err(|error| error.to_string())
}

#[derive(Debug, Clone, Copy)]
pub struct ColumnarLimits {
    pub max_columns: usize,
    pub max_rows: usize,
    pub max_values: usize,
}

pub struct DecodedColumnarTable {
    pub metadata: HashMap<String, String>,
    pub columns: Vec<(String, Vec<f64>)>,
}

fn adapter_error(format: &str, detail: impl std::fmt::Display) -> String {
    format!("{format} import: {detail}")
}

pub fn decode_arrow_ipc(
    bytes: &[u8],
    limits: ColumnarLimits,
    format: &str,
) -> Result<DecodedColumnarTable, String> {
    use arrow_ipc::reader::{FileReader, StreamReader};
    let mut batches = Vec::new();
    let file_attempt = FileReader::try_new(Cursor::new(bytes), None);
    let metadata = match file_attempt {
        Ok(reader) => {
            let metadata = reader.schema().metadata().clone();
            for batch in reader {
                batches.push(batch.map_err(|error| {
                    adapter_error(format, format_args!("invalid Arrow record batch: {error}"))
                })?);
            }
            metadata
        }
        Err(file_error) => {
            let reader = StreamReader::try_new(Cursor::new(bytes), None).map_err(|stream_error| {
                adapter_error(
                    format,
                    format_args!(
                        "neither Arrow file nor stream framing is valid (file: {file_error}; stream: {stream_error})"
                    ),
                )
            })?;
            let metadata = reader.schema().metadata().clone();
            for batch in reader {
                batches.push(batch.map_err(|error| {
                    adapter_error(format, format_args!("invalid Arrow stream batch: {error}"))
                })?);
            }
            metadata
        }
    };
    decode_arrow_batches(format, batches, metadata, limits)
}

pub fn decode_parquet(
    bytes: &[u8],
    limits: ColumnarLimits,
    format: &str,
) -> Result<DecodedColumnarTable, String> {
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    let builder = ParquetRecordBatchReaderBuilder::try_new(bytes::Bytes::copy_from_slice(bytes))
        .map_err(|error| {
            adapter_error(format, format_args!("invalid Parquet metadata: {error}"))
        })?;
    let metadata = builder.schema().metadata().clone();
    let reader = builder.with_batch_size(16_384).build().map_err(|error| {
        adapter_error(
            format,
            format_args!("could not create Parquet reader: {error}"),
        )
    })?;
    let mut batches = Vec::new();
    for batch in reader {
        batches.push(batch.map_err(|error| {
            adapter_error(format, format_args!("invalid Parquet row group: {error}"))
        })?);
    }
    decode_arrow_batches(format, batches, metadata, limits)
}

fn decode_arrow_batches(
    format: &str,
    batches: Vec<arrow_array::RecordBatch>,
    metadata: HashMap<String, String>,
    limits: ColumnarLimits,
) -> Result<DecodedColumnarTable, String> {
    let max_columns = limits.max_columns;
    let max_rows = limits.max_rows;
    let max_values = limits.max_values;
    let first = batches
        .first()
        .ok_or_else(|| adapter_error(format, "the table contains no record batches"))?;
    let schema = first.schema();
    if schema.fields().len() < 2 || schema.fields().len() > max_columns.saturating_mul(2) {
        return Err(adapter_error(
            format,
            format_args!(
                "the table has {} fields; expected 2..={}",
                schema.fields().len(),
                max_columns
            ),
        ));
    }
    let row_count = batches
        .iter()
        .try_fold(0_usize, |count, batch| count.checked_add(batch.num_rows()))
        .ok_or_else(|| adapter_error(format, "row count overflow"))?;
    if row_count > max_rows {
        return Err(adapter_error(
            format,
            format_args!("the table has {row_count} rows; the limit is {max_rows}"),
        ));
    }
    let table_values = row_count
        .checked_mul(schema.fields().len())
        .ok_or_else(|| adapter_error(format, "table value count overflow"))?;
    if table_values > max_values {
        return Err(adapter_error(
            format,
            format_args!("the table contains {table_values} values; the limit is {max_values}"),
        ));
    }
    let mut columns = schema
        .fields()
        .iter()
        .map(|field| (field.name().clone(), Vec::with_capacity(row_count)))
        .collect::<Vec<_>>();
    for batch in batches {
        if batch.schema().as_ref() != schema.as_ref() {
            return Err(adapter_error(
                format,
                "record-batch schema changed within the source",
            ));
        }
        for (index, array) in batch.columns().iter().enumerate() {
            let name = columns[index].0.clone();
            let values = numeric_values(format, &name, array.as_ref())?;
            columns[index].1.extend(values);
        }
    }
    Ok(DecodedColumnarTable { metadata, columns })
}

fn numeric_values(
    format: &str,
    name: &str,
    array: &dyn arrow_array::Array,
) -> Result<Vec<f64>, String> {
    use arrow_array::{
        BooleanArray, Float32Array, Float64Array, Int8Array, Int16Array, Int32Array, Int64Array,
        UInt8Array, UInt16Array, UInt32Array, UInt64Array,
    };
    if array.null_count() != 0 {
        return Err(adapter_error(
            format,
            format_args!(
                "column '{name}' contains {} null values",
                array.null_count()
            ),
        ));
    }
    macro_rules! float_values {
        ($ty:ty) => {
            array
                .as_any()
                .downcast_ref::<$ty>()
                .map(|array| array.values().iter().map(|value| *value as f64).collect())
        };
    }
    macro_rules! signed_values {
        ($ty:ty) => {
            array.as_any().downcast_ref::<$ty>().map(|array| {
                array
                    .values()
                    .iter()
                    .map(|value| {
                        crate::numeric::exact_signed_integer(name, *value as i64)
                            .map_err(|detail| adapter_error(format, detail))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
        };
    }
    macro_rules! unsigned_values {
        ($ty:ty) => {
            array.as_any().downcast_ref::<$ty>().map(|array| {
                array
                    .values()
                    .iter()
                    .map(|value| {
                        crate::numeric::exact_unsigned_integer(name, *value as u64)
                            .map_err(|detail| adapter_error(format, detail))
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
        };
    }
    let values = float_values!(Float64Array)
        .or_else(|| float_values!(Float32Array))
        .map(Ok)
        .or_else(|| signed_values!(Int64Array))
        .or_else(|| signed_values!(Int32Array))
        .or_else(|| signed_values!(Int16Array))
        .or_else(|| signed_values!(Int8Array))
        .or_else(|| unsigned_values!(UInt64Array))
        .or_else(|| unsigned_values!(UInt32Array))
        .or_else(|| unsigned_values!(UInt16Array))
        .or_else(|| unsigned_values!(UInt8Array))
        .or_else(|| {
            array.as_any().downcast_ref::<BooleanArray>().map(|array| {
                Ok((0..array.len())
                    .map(|index| if array.value(index) { 1.0 } else { 0.0 })
                    .collect())
            })
        });
    values.ok_or_else(|| {
        adapter_error(
            format,
            format_args!(
                "column '{name}' has unsupported Arrow type {}",
                array.data_type()
            ),
        )
    })?
}

#[cfg(test)]
mod tests {
    use super::{ColumnarLimits, ParquetTableSource, decode_arrow_batches, encode_parquet_table};
    use arrow_array::{Array, ArrayRef, Float64Array, Int64Array, RecordBatch, StringArray};
    use arrow_schema::{DataType, Field, Schema};
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn refuses_rows_over_limit_and_inexact_integer_columns() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("time", DataType::Float64, false),
            Field::new("count", DataType::Int64, false),
        ]));
        let batch = RecordBatch::try_new(
            schema,
            vec![
                Arc::new(Float64Array::from(vec![0.0, 1.0])) as ArrayRef,
                Arc::new(Int64Array::from(vec![1, (1_i64 << 53) + 1])) as ArrayRef,
            ],
        )
        .expect("record batch");
        let limits = ColumnarLimits {
            max_columns: 2,
            max_rows: 1,
            max_values: 4,
        };
        let error = decode_arrow_batches("arrow_ipc", vec![batch.clone()], HashMap::new(), limits)
            .err()
            .expect("row limit");
        assert!(error.contains("the table has 2 rows; the limit is 1"));

        let limits = ColumnarLimits {
            max_rows: 2,
            ..limits
        };
        let error = decode_arrow_batches("arrow_ipc", vec![batch], HashMap::new(), limits)
            .err()
            .expect("precision loss");
        assert!(error.contains("cannot be represented exactly as f64"));
    }

    struct SelectedTable;

    impl ParquetTableSource for SelectedTable {
        fn column_count(&self) -> usize {
            2
        }
        fn row_count(&self) -> usize {
            2
        }
        fn column_id(&self, column: usize) -> &str {
            ["time", "label"][column]
        }
        fn column_label(&self, column: usize) -> &str {
            ["Time", "Label"][column]
        }
        fn column_unit(&self, column: usize) -> Option<&str> {
            (column == 0).then_some("s")
        }
        fn numeric_value(&self, row: usize, column: usize) -> Option<f64> {
            (column == 0).then_some(row as f64)
        }
        fn display_value(&self, row: usize, column: usize) -> Option<&str> {
            (column == 1 && row == 0).then_some("first")
        }
    }

    #[test]
    fn parquet_writer_preserves_numeric_text_and_null_columns() {
        let bytes = encode_parquet_table(
            &SelectedTable,
            Some(vec![(
                "rspice.grid_id".to_owned(),
                Some("fixture".to_owned()),
            )]),
        )
        .expect("Parquet bytes");
        let mut reader = ParquetRecordBatchReaderBuilder::try_new(bytes::Bytes::from(bytes))
            .expect("Parquet metadata")
            .build()
            .expect("Parquet reader");
        let batch = reader.next().expect("batch").expect("batch values");
        assert_eq!(batch.schema().field(0).metadata().get("unit").unwrap(), "s");
        let time = batch
            .column(0)
            .as_any()
            .downcast_ref::<Float64Array>()
            .unwrap();
        assert_eq!(time.values(), &[0.0, 1.0]);
        let label = batch
            .column(1)
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        assert_eq!(label.value(0), "first");
        assert!(label.is_null(1));
    }
}

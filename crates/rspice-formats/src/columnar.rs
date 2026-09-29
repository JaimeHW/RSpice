//! Arrow IPC and Parquet codecs for numeric results and selected tables.

use std::collections::HashMap;
use std::io::Cursor;
use std::sync::Arc;

use crate::table::EngineeringTableSource;

#[derive(Debug)]
pub enum ParquetTableError {
    Arrow(arrow_schema::ArrowError),
    Parquet(parquet::errors::ParquetError),
}

impl std::fmt::Display for ParquetTableError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Arrow(source) => source.fmt(f),
            Self::Parquet(source) => source.fmt(f),
        }
    }
}

impl std::error::Error for ParquetTableError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(match self {
            Self::Arrow(source) => source,
            Self::Parquet(source) => source,
        })
    }
}

/// A columnar decoder failure with the caller's source-format identity.
#[derive(Debug)]
pub struct ColumnarReadError {
    pub format: String,
    pub reason: ColumnarReadFailure,
}

#[derive(Debug)]
pub enum ColumnarReadFailure {
    ArrowFraming {
        file: Box<arrow_schema::ArrowError>,
        stream: Box<arrow_schema::ArrowError>,
    },
    ArrowBatch {
        context: &'static str,
        source: arrow_schema::ArrowError,
    },
    ParquetMetadata(parquet::errors::ParquetError),
    ParquetReader(parquet::errors::ParquetError),
    FieldCount {
        fields: usize,
        max_columns: usize,
    },
    SchemaChanged,
    RowCountOverflow,
    RowLimit {
        rows: usize,
        limit: usize,
    },
    ValueCountOverflow,
    ValueLimit {
        values: usize,
        limit: usize,
    },
    Empty,
    NullValues {
        column: String,
        count: usize,
    },
    UnsupportedType {
        column: String,
        data_type: Box<arrow_schema::DataType>,
    },
    InexactInteger(crate::numeric::ExactIntegerError),
    MissingCoordinate(String),
    AnalysisDomain(crate::UnsupportedWaveformDomain),
    ComplexColumns(crate::numeric::ComplexColumnError),
}

impl std::fmt::Display for ColumnarReadFailure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ArrowFraming { file, stream } => write!(
                f,
                "neither Arrow file nor stream framing is valid (file: {file}; stream: {stream})"
            ),
            Self::ArrowBatch { context, source } => write!(f, "{context}: {source}"),
            Self::ParquetMetadata(source) => write!(f, "invalid Parquet metadata: {source}"),
            Self::ParquetReader(source) => write!(f, "could not create Parquet reader: {source}"),
            Self::FieldCount {
                fields,
                max_columns,
            } => write!(
                f,
                "the table has {fields} fields; expected 2..={max_columns}"
            ),
            Self::SchemaChanged => f.write_str("record-batch schema changed within the source"),
            Self::RowCountOverflow => f.write_str("row count overflow"),
            Self::RowLimit { rows, limit } => {
                write!(f, "the table has {rows} rows; the limit is {limit}")
            }
            Self::ValueCountOverflow => f.write_str("table value count overflow"),
            Self::ValueLimit { values, limit } => write!(
                f,
                "the table contains {values} values; the limit is {limit}"
            ),
            Self::Empty => f.write_str("the table contains no record batches"),
            Self::NullValues { column, count } => {
                write!(f, "column '{column}' contains {count} null values")
            }
            Self::UnsupportedType { column, data_type } => write!(
                f,
                "column '{column}' has unsupported Arrow type {data_type}"
            ),
            Self::InexactInteger(source) => source.fmt(f),
            Self::MissingCoordinate(name) => {
                write!(f, "schema metadata names missing coordinate '{name}'")
            }
            Self::AnalysisDomain(source) => source.fmt(f),
            Self::ComplexColumns(source) => source.fmt(f),
        }
    }
}

impl std::error::Error for ColumnarReadFailure {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ArrowFraming { stream, .. } => Some(stream.as_ref()),
            Self::ArrowBatch { source, .. } => Some(source),
            Self::ParquetMetadata(source) | Self::ParquetReader(source) => Some(source),
            Self::InexactInteger(source) => Some(source),
            Self::AnalysisDomain(source) => Some(source),
            Self::ComplexColumns(source) => Some(source),
            _ => None,
        }
    }
}

impl std::fmt::Display for ColumnarReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} import: {}", self.format, self.reason)
    }
}

impl std::error::Error for ColumnarReadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.reason)
    }
}

/// Encode an already-selected table as Parquet, preserving nullable numeric
/// and text columns and caller-supplied provenance metadata.
pub fn encode_parquet_table(
    source: &impl EngineeringTableSource,
    metadata: Option<Vec<(String, Option<String>)>>,
) -> Result<Vec<u8>, ParquetTableError> {
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
    let batch = RecordBatch::try_new(schema.clone(), arrays).map_err(ParquetTableError::Arrow)?;
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
        .map_err(ParquetTableError::Parquet)?;
    writer.write(&batch).map_err(ParquetTableError::Parquet)?;
    writer.into_inner().map_err(ParquetTableError::Parquet)
}

#[derive(Debug, Clone, Copy)]
pub struct ColumnarLimits {
    pub max_columns: usize,
    pub max_rows: usize,
    pub max_values: usize,
}

struct DecodedColumnarTable {
    metadata: HashMap<String, String>,
    columns: Vec<(String, Vec<f64>)>,
}

fn adapter_error(format: &str, reason: ColumnarReadFailure) -> ColumnarReadError {
    ColumnarReadError {
        format: format.to_owned(),
        reason,
    }
}

/// Decode Arrow waveform columns and their coordinate/domain metadata.
pub fn decode_arrow_ipc(
    bytes: &[u8],
    limits: ColumnarLimits,
    format: &str,
) -> Result<crate::numeric::DecodedNumericDataset, ColumnarReadError> {
    finish_columnar_table(format, decode_arrow_ipc_table(bytes, limits, format)?)
}

/// Decode Parquet waveform columns and their coordinate/domain metadata.
pub fn decode_parquet(
    bytes: &[u8],
    limits: ColumnarLimits,
    format: &str,
) -> Result<crate::numeric::DecodedNumericDataset, ColumnarReadError> {
    finish_columnar_table(format, decode_parquet_table(bytes, limits, format)?)
}

fn finish_columnar_table(
    format: &str,
    table: DecodedColumnarTable,
) -> Result<crate::numeric::DecodedNumericDataset, ColumnarReadError> {
    let DecodedColumnarTable {
        metadata,
        mut columns,
    } = table;
    let coordinate_name = metadata
        .get("rspice.coordinate")
        .cloned()
        .unwrap_or_else(|| columns[0].0.clone());
    let coordinate_index = columns
        .iter()
        .position(|(name, _)| name == &coordinate_name)
        .ok_or_else(|| {
            adapter_error(
                format,
                ColumnarReadFailure::MissingCoordinate(coordinate_name.clone()),
            )
        })?;
    let coordinate = columns.remove(coordinate_index).1;
    let domain = metadata
        .get("rspice.analysis")
        .map(|value| {
            value
                .parse::<crate::WaveformDomain>()
                .map_err(|error| adapter_error(format, ColumnarReadFailure::AnalysisDomain(error)))
        })
        .transpose()?
        .unwrap_or_else(|| crate::WaveformDomain::from_coordinate_name(&coordinate_name));
    let signals = crate::numeric::combine_real_imag_columns(columns)
        .map_err(|error| adapter_error(format, ColumnarReadFailure::ComplexColumns(error)))?;
    Ok(crate::numeric::DecodedNumericDataset {
        domain,
        coordinate_name,
        coordinate,
        signals,
    })
}

fn decode_arrow_ipc_table(
    bytes: &[u8],
    limits: ColumnarLimits,
    format: &str,
) -> Result<DecodedColumnarTable, ColumnarReadError> {
    use arrow_ipc::reader::{FileReader, StreamReader};
    let file_attempt = FileReader::try_new(Cursor::new(bytes), None);
    match file_attempt {
        Ok(reader) => {
            let metadata = reader.schema().metadata().clone();
            decode_arrow_batches(
                format,
                reader,
                metadata,
                limits,
                "invalid Arrow record batch",
            )
        }
        Err(file_error) => {
            let reader =
                StreamReader::try_new(Cursor::new(bytes), None).map_err(|stream_error| {
                    adapter_error(
                        format,
                        ColumnarReadFailure::ArrowFraming {
                            file: Box::new(file_error),
                            stream: Box::new(stream_error),
                        },
                    )
                })?;
            let metadata = reader.schema().metadata().clone();
            decode_arrow_batches(
                format,
                reader,
                metadata,
                limits,
                "invalid Arrow stream batch",
            )
        }
    }
}

fn decode_parquet_table(
    bytes: &[u8],
    limits: ColumnarLimits,
    format: &str,
) -> Result<DecodedColumnarTable, ColumnarReadError> {
    use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
    let builder = ParquetRecordBatchReaderBuilder::try_new(bytes::Bytes::copy_from_slice(bytes))
        .map_err(|error| adapter_error(format, ColumnarReadFailure::ParquetMetadata(error)))?;
    let metadata = builder.schema().metadata().clone();
    let batch_size = 16_384.min(limits.max_rows.saturating_add(1).max(1));
    let reader = builder
        .with_batch_size(batch_size)
        .build()
        .map_err(|error| adapter_error(format, ColumnarReadFailure::ParquetReader(error)))?;
    decode_arrow_batches(
        format,
        reader,
        metadata,
        limits,
        "invalid Parquet row group",
    )
}

fn decode_arrow_batches(
    format: &str,
    batches: impl IntoIterator<Item = Result<arrow_array::RecordBatch, arrow_schema::ArrowError>>,
    metadata: HashMap<String, String>,
    limits: ColumnarLimits,
    batch_error: &'static str,
) -> Result<DecodedColumnarTable, ColumnarReadError> {
    let mut decoded = ColumnarAccumulator::new(format, limits);
    for batch in batches {
        let batch = batch.map_err(|error| {
            adapter_error(
                format,
                ColumnarReadFailure::ArrowBatch {
                    context: batch_error,
                    source: error,
                },
            )
        })?;
        decoded.push(batch)?;
    }
    decoded.finish(metadata)
}

struct ColumnarAccumulator<'a> {
    format: &'a str,
    limits: ColumnarLimits,
    schema: Option<arrow_schema::SchemaRef>,
    rows: usize,
    columns: Vec<(String, Vec<f64>)>,
}

impl<'a> ColumnarAccumulator<'a> {
    fn new(format: &'a str, limits: ColumnarLimits) -> Self {
        Self {
            format,
            limits,
            schema: None,
            rows: 0,
            columns: Vec::new(),
        }
    }

    fn push(&mut self, batch: arrow_array::RecordBatch) -> Result<(), ColumnarReadError> {
        let schema = batch.schema();
        if self.schema.is_none() {
            let fields = schema.fields().len();
            if fields < 2 || fields > self.limits.max_columns.saturating_mul(2) {
                return Err(adapter_error(
                    self.format,
                    ColumnarReadFailure::FieldCount {
                        fields,
                        max_columns: self.limits.max_columns,
                    },
                ));
            }
            self.columns = schema
                .fields()
                .iter()
                .map(|field| (field.name().clone(), Vec::new()))
                .collect();
            self.schema = Some(schema.clone());
        }
        if self.schema.as_ref() != Some(&schema) {
            return Err(adapter_error(
                self.format,
                ColumnarReadFailure::SchemaChanged,
            ));
        }
        let rows = self
            .rows
            .checked_add(batch.num_rows())
            .ok_or_else(|| adapter_error(self.format, ColumnarReadFailure::RowCountOverflow))?;
        if rows > self.limits.max_rows {
            return Err(adapter_error(
                self.format,
                ColumnarReadFailure::RowLimit {
                    rows,
                    limit: self.limits.max_rows,
                },
            ));
        }
        let values = rows
            .checked_mul(schema.fields().len())
            .ok_or_else(|| adapter_error(self.format, ColumnarReadFailure::ValueCountOverflow))?;
        if values > self.limits.max_values {
            return Err(adapter_error(
                self.format,
                ColumnarReadFailure::ValueLimit {
                    values,
                    limit: self.limits.max_values,
                },
            ));
        }
        for (index, array) in batch.columns().iter().enumerate() {
            let (name, column) = &mut self.columns[index];
            column.extend(numeric_values(self.format, name, array.as_ref())?);
        }
        self.rows = rows;
        Ok(())
    }

    fn finish(
        self,
        metadata: HashMap<String, String>,
    ) -> Result<DecodedColumnarTable, ColumnarReadError> {
        if self.schema.is_none() {
            return Err(adapter_error(self.format, ColumnarReadFailure::Empty));
        }
        Ok(DecodedColumnarTable {
            metadata,
            columns: self.columns,
        })
    }
}

fn numeric_values(
    format: &str,
    name: &str,
    array: &dyn arrow_array::Array,
) -> Result<Vec<f64>, ColumnarReadError> {
    use arrow_array::{
        BooleanArray, Float32Array, Float64Array, Int8Array, Int16Array, Int32Array, Int64Array,
        UInt8Array, UInt16Array, UInt32Array, UInt64Array,
    };
    if array.null_count() != 0 {
        return Err(adapter_error(
            format,
            ColumnarReadFailure::NullValues {
                column: name.to_owned(),
                count: array.null_count(),
            },
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
                        crate::numeric::exact_signed_integer(name, *value as i64).map_err(
                            |detail| {
                                adapter_error(format, ColumnarReadFailure::InexactInteger(detail))
                            },
                        )
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
                        crate::numeric::exact_unsigned_integer(name, *value as u64).map_err(
                            |detail| {
                                adapter_error(format, ColumnarReadFailure::InexactInteger(detail))
                            },
                        )
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
            ColumnarReadFailure::UnsupportedType {
                column: name.to_owned(),
                data_type: Box::new(array.data_type().clone()),
            },
        )
    })?
}

#[cfg(test)]
mod tests {
    use super::{
        ColumnarLimits, ColumnarReadFailure, DecodedColumnarTable, decode_arrow_batches,
        encode_parquet_table, finish_columnar_table,
    };
    use crate::table::EngineeringTableSource;
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
        let error = decode_arrow_batches(
            "arrow_ipc",
            [Ok::<_, arrow_schema::ArrowError>(batch.clone())],
            HashMap::new(),
            limits,
            "invalid Arrow record batch",
        )
        .err()
        .expect("row limit");
        assert!(matches!(
            error.reason,
            ColumnarReadFailure::RowLimit { rows: 2, limit: 1 }
        ));
        assert_eq!(
            error.to_string(),
            "arrow_ipc import: the table has 2 rows; the limit is 1"
        );

        let limits = ColumnarLimits {
            max_rows: 2,
            ..limits
        };
        let error = decode_arrow_batches(
            "arrow_ipc",
            [Ok::<_, arrow_schema::ArrowError>(batch)],
            HashMap::new(),
            limits,
            "invalid Arrow record batch",
        )
        .err()
        .expect("precision loss");
        assert!(
            matches!(&error.reason, ColumnarReadFailure::InexactInteger(crate::numeric::ExactIntegerError::Signed { identity, value }) if identity == "count" && *value == (1_i64 << 53) + 1)
        );
        assert_eq!(
            error.to_string(),
            "arrow_ipc import: 'count' integer 9007199254740993 cannot be represented exactly as f64"
        );
        assert!(
            std::error::Error::source(&error.reason)
                .unwrap()
                .is::<crate::numeric::ExactIntegerError>()
        );
    }

    #[test]
    fn cumulative_row_limit_stops_before_reading_another_batch() {
        let schema = Arc::new(Schema::new(vec![
            Field::new("time", DataType::Float64, false),
            Field::new("voltage", DataType::Float64, false),
        ]));
        let batch = RecordBatch::try_new(
            schema,
            vec![
                Arc::new(Float64Array::from(vec![0.0, 1.0])) as ArrayRef,
                Arc::new(Float64Array::from(vec![2.0, 3.0])) as ArrayRef,
            ],
        )
        .expect("record batch");
        let batches = [Ok::<_, arrow_schema::ArrowError>(batch.clone()), Ok(batch)]
            .into_iter()
            .chain(std::iter::once_with(|| {
                panic!("must not read a third batch")
            }));
        let error = decode_arrow_batches(
            "arrow_ipc",
            batches,
            HashMap::new(),
            ColumnarLimits {
                max_columns: 2,
                max_rows: 3,
                max_values: 8,
            },
            "invalid Arrow record batch",
        )
        .err()
        .expect("cumulative row limit");
        assert!(matches!(
            error.reason,
            ColumnarReadFailure::RowLimit { rows: 4, limit: 3 }
        ));
        assert_eq!(
            error.to_string(),
            "arrow_ipc import: the table has 4 rows; the limit is 3"
        );
    }

    struct SelectedTable;

    impl EngineeringTableSource for SelectedTable {
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

    #[test]
    fn malformed_containers_preserve_parser_causes() {
        use std::error::Error as _;
        let limits = ColumnarLimits {
            max_columns: 2,
            max_rows: 2,
            max_values: 4,
        };
        let error =
            super::decode_arrow_ipc(b"invalid", limits, "arrow_ipc").expect_err("invalid Arrow");
        let ColumnarReadFailure::ArrowFraming { file, stream } = &error.reason else {
            panic!("expected both framing errors: {error}");
        };
        assert_eq!(
            error.to_string(),
            format!(
                "arrow_ipc import: neither Arrow file nor stream framing is valid (file: {file}; stream: {stream})"
            )
        );
        assert!(
            error
                .reason
                .source()
                .unwrap()
                .is::<arrow_schema::ArrowError>()
        );
        let error =
            super::decode_parquet(b"invalid", limits, "parquet").expect_err("invalid Parquet");
        let ColumnarReadFailure::ParquetMetadata(source) = &error.reason else {
            panic!("expected a metadata error: {error}");
        };
        assert_eq!(
            error.to_string(),
            format!("parquet import: invalid Parquet metadata: {source}")
        );
        assert!(
            error
                .reason
                .source()
                .unwrap()
                .is::<parquet::errors::ParquetError>()
        );
    }

    #[test]
    fn metadata_selects_coordinate_and_domain_before_pairing_signals() {
        let table = || DecodedColumnarTable {
            metadata: HashMap::from([
                ("rspice.coordinate".into(), "x".into()),
                ("rspice.analysis".into(), " AC ".into()),
            ]),
            columns: vec![
                ("gain_IM".into(), vec![-0.0]),
                ("x".into(), vec![3.0]),
                ("gain_RE".into(), vec![1.0]),
                ("plain".into(), vec![4.0]),
            ],
        };
        let decoded = finish_columnar_table("arrow_ipc", table()).unwrap();
        assert_eq!(decoded.domain, crate::WaveformDomain::Ac);
        assert_eq!(decoded.coordinate_name, "x");
        assert_eq!(decoded.coordinate, [3.0]);
        assert_eq!(
            decoded
                .signals
                .iter()
                .map(|s| s.name.as_str())
                .collect::<Vec<_>>(),
            ["plain", "gain"]
        );
        assert_eq!(decoded.signals[0].real, [4.0]);
        assert_eq!(decoded.signals[1].real, [1.0]);
        assert_eq!(
            decoded.signals[1].imag.as_ref().unwrap()[0].to_bits(),
            (-0.0_f64).to_bits()
        );

        let mut invalid = table();
        invalid
            .metadata
            .insert("rspice.coordinate".into(), "missing".into());
        invalid
            .metadata
            .insert("rspice.analysis".into(), " UNKNOWN ".into());
        let error = finish_columnar_table("arrow_ipc", invalid).unwrap_err();
        assert!(
            matches!(&error.reason, ColumnarReadFailure::MissingCoordinate(name) if name == "missing")
        );
        assert_eq!(
            error.to_string(),
            "arrow_ipc import: schema metadata names missing coordinate 'missing'"
        );

        let mut invalid = table();
        invalid.columns.remove(0);
        invalid
            .metadata
            .insert("rspice.analysis".into(), " UNKNOWN ".into());
        let error = finish_columnar_table("parquet", invalid).unwrap_err();
        assert!(
            matches!(&error.reason, ColumnarReadFailure::AnalysisDomain(source) if source.value == "unknown")
        );
        assert_eq!(
            error.to_string(),
            "parquet import: unsupported analysis domain 'unknown'"
        );

        let mut invalid = table();
        invalid.columns.remove(0);
        let error = finish_columnar_table("parquet", invalid).unwrap_err();
        assert!(
            matches!(&error.reason, ColumnarReadFailure::ComplexColumns(crate::numeric::ComplexColumnError::MissingImaginary(name)) if name == "gain")
        );
    }
}

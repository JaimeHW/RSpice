//! Official XSPICE `filesource` analog file-source code model.

use crate::Value;
use crate::xspice::{
    AnalysisType, CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec,
    PortDirection, PortSpec, PortType, data_file,
};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

#[derive(Debug, Default)]
pub struct FileSource;

#[derive(Debug, Default)]
pub struct FileSourceAlias;

#[derive(Debug, Clone, Copy)]
struct RawFileSourceField {
    line: usize,
    value: Value,
}

#[derive(Debug, Clone)]
struct FileSourceRow {
    time: Value,
}

#[derive(Debug, Clone)]
struct FileSourceRowsData {
    rows: Vec<FileSourceRow>,
    values: Vec<Value>,
    width: usize,
    strictly_increasing_time: bool,
    transient_breakpoints: Vec<Value>,
}

impl FileSourceRowsData {
    fn new(rows: Vec<FileSourceRow>, values: Vec<Value>, width: usize) -> Self {
        let strictly_increasing_time = rows
            .windows(2)
            .all(|window| window[0].time < window[1].time);
        let mut transient_breakpoints: Vec<Value> = rows.iter().map(|row| row.time).collect();
        if !strictly_increasing_time {
            transient_breakpoints.sort_by(|left, right| {
                left.partial_cmp(right).unwrap_or(std::cmp::Ordering::Equal)
            });
            transient_breakpoints.dedup_by(|left, right| *left == *right);
        }
        Self {
            rows,
            values,
            width,
            strictly_increasing_time,
            transient_breakpoints,
        }
    }

    fn row_values(&self, row: usize) -> &[Value] {
        let Some(start) = row.checked_mul(self.width) else {
            return &[];
        };
        let Some(end) = start.checked_add(self.width) else {
            return &[];
        };
        self.values.get(start..end).unwrap_or(&[])
    }

    fn next_nonmonotonic_breakpoint(&self, time: Value) -> Option<Value> {
        if self.strictly_increasing_time {
            return None;
        }
        let breakpoints = self.transient_breakpoints.as_slice();
        let index = breakpoints.partition_point(|breakpoint| *breakpoint <= time);
        breakpoints.get(index).copied()
    }
}

impl std::ops::Deref for FileSourceRowsData {
    type Target = [FileSourceRow];

    fn deref(&self) -> &Self::Target {
        self.rows.as_slice()
    }
}

const FILESOURCE_ROW_CURSOR_STATE: usize = 0;
const FILESOURCE_ROWS_RESOURCE: &str = "xspice.filesource.rows";
const FILESOURCE_TRANSFORMED_ROWS_RESOURCE: &str = "xspice.filesource.transformed_rows";

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct FileSourceCacheKey {
    file: String,
    width: usize,
    len: u64,
    modified_nanos: u128,
    content_hash: u64,
    virtual_file: bool,
}

#[derive(Debug, Clone)]
struct FileSourceRowsResource {
    file: String,
    width: usize,
    virtual_stamp: Option<data_file::DataFileStamp>,
    fields: Arc<Vec<RawFileSourceField>>,
}

#[derive(Debug, Clone, PartialEq)]
struct FileSourceTransformedRowsSignature {
    file: String,
    width: usize,
    virtual_stamp: Option<data_file::DataFileStamp>,
    timeoffset: Value,
    timescale: Value,
    timerelative: bool,
    amploffset_revision: Option<u64>,
    amplscale_revision: Option<u64>,
}

#[derive(Debug, Clone)]
struct FileSourceTransformedRowsResource {
    signature: FileSourceTransformedRowsSignature,
    rows: CmResult<Arc<FileSourceRowsData>>,
}

#[derive(Debug, Default)]
struct FileSourceCache {
    virtual_epoch: u64,
    entries: HashMap<FileSourceCacheKey, Arc<Vec<RawFileSourceField>>>,
}

impl FileSourceCache {
    fn sync_virtual_epoch(&mut self) {
        if data_file::sync_virtual_data_file_epoch(&mut self.virtual_epoch) {
            self.entries.retain(|key, _| !key.virtual_file);
        }
    }

    #[cfg(test)]
    fn clear(&mut self) {
        self.entries.clear();
        self.virtual_epoch = data_file::virtual_data_file_epoch();
    }
}

fn filesource_cache() -> &'static Mutex<FileSourceCache> {
    static CACHE: OnceLock<Mutex<FileSourceCache>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(FileSourceCache::default()))
}

fn lock_filesource_cache() -> MutexGuard<'static, FileSourceCache> {
    filesource_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn filesource_error(file: &str, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("filesource file '{}': {}", file, message.into()))
}

fn filesource_line_error(file: &str, line: usize, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!(
        "filesource file '{}' line {}: {}",
        file,
        line,
        message.into()
    ))
}

fn invalid_filesource_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn filesource_width_error(file: &str, width: usize, message: impl Into<String>) -> CmError {
    filesource_error(
        file,
        format!(
            "output vector width {width} is too large: {}",
            message.into()
        ),
    )
}

fn filesource_expected_columns(file: &str, width: usize) -> CmResult<usize> {
    width
        .checked_add(1)
        .ok_or_else(|| filesource_width_error(file, width, "column count overflows usize"))
}

fn filesource_value_capacity(file: &str, row_count: usize, width: usize) -> CmResult<usize> {
    row_count.checked_mul(width).ok_or_else(|| {
        filesource_width_error(
            file,
            width,
            format!("{row_count} row(s) exceed addressable value storage"),
        )
    })
}

fn reserve_filesource_rows(
    file: &str,
    row_capacity: usize,
    rows: &mut Vec<FileSourceRow>,
) -> CmResult<()> {
    rows.try_reserve_exact(row_capacity).map_err(|err| {
        filesource_error(
            file,
            format!("unable to reserve {row_capacity} rows: {err}"),
        )
    })
}

fn reserve_filesource_values(
    file: &str,
    value_capacity: usize,
    values: &mut Vec<Value>,
) -> CmResult<()> {
    values.try_reserve_exact(value_capacity).map_err(|err| {
        filesource_error(
            file,
            format!("unable to reserve {value_capacity} values: {err}"),
        )
    })
}

fn cache_key(file: &str, width: usize, stamp: data_file::DataFileStamp) -> FileSourceCacheKey {
    FileSourceCacheKey {
        file: file.to_string(),
        width,
        len: stamp.len,
        modified_nanos: stamp.modified_nanos,
        content_hash: stamp.content_hash,
        virtual_file: stamp.virtual_file,
    }
}

fn parse_filesource_value_token(
    file: &str,
    line: usize,
    input: &str,
    value_index: usize,
) -> CmResult<Option<(Value, usize)>> {
    let Some((value, len)) = data_file::parse_numeric_prefix_len(input) else {
        return Ok(None);
    };
    if !value.is_finite() {
        return Err(filesource_line_error(
            file,
            line,
            format!("value[{value_index}] must be finite, got {input}"),
        ));
    }
    Ok(Some((value, len)))
}

fn parse_filesource_time_token(
    file: &str,
    line: usize,
    input: &str,
) -> CmResult<Option<(Value, usize)>> {
    let Some((value, len)) = data_file::parse_numeric_prefix_len(input) else {
        return Ok(None);
    };
    if !value.is_finite() {
        return Err(filesource_line_error(
            file,
            line,
            format!("time must be finite, got {input}"),
        ));
    }
    Ok(Some((value, len)))
}

fn skip_filesource_data_delimiters(input: &str, mut offset: usize) -> usize {
    let bytes = input.as_bytes();
    while offset < bytes.len() && (bytes[offset].is_ascii_whitespace() || bytes[offset] == b',') {
        offset += 1;
    }
    offset
}

fn parse_filesource_contents(
    file: &str,
    width: usize,
    contents: &str,
) -> CmResult<Vec<RawFileSourceField>> {
    parse_filesource_contents_limited(
        file,
        width,
        contents,
        crate::resource::ResourceLimits::default().max_external_data_values,
    )
}

fn push_filesource_field(
    file: &str,
    fields: &mut Vec<RawFileSourceField>,
    field: RawFileSourceField,
    limit: usize,
) -> CmResult<()> {
    crate::resource::ResourceLimitError::ensure(
        crate::resource::ResourceKind::ExternalDataValues,
        fields.len().saturating_add(1),
        limit,
    )
    .map_err(|error| filesource_error(file, error.to_string()))?;
    fields.try_reserve(1).map_err(|error| {
        filesource_error(
            file,
            format!("unable to reserve parsed data value: {error}"),
        )
    })?;
    fields.push(field);
    Ok(())
}

fn parse_filesource_contents_limited(
    file: &str,
    width: usize,
    contents: &str,
    value_limit: usize,
) -> CmResult<Vec<RawFileSourceField>> {
    if width == 0 {
        return Err(filesource_error(
            file,
            "output vector width must be greater than zero",
        ));
    }

    let expected_columns = filesource_expected_columns(file, width)?;
    let mut fields = Vec::new();

    for (line_index, line) in contents.lines().enumerate() {
        let line_number = line_index + 1;
        let trimmed_start = line.trim_start();
        if trimmed_start.is_empty()
            || trimmed_start.starts_with('*')
            || trimmed_start.starts_with('#')
            || trimmed_start.starts_with(';')
        {
            continue;
        }

        let Some((time, time_len)) = parse_filesource_time_token(file, line_number, trimmed_start)?
        else {
            continue;
        };

        push_filesource_field(
            file,
            &mut fields,
            RawFileSourceField {
                line: line_number,
                value: time,
            },
            value_limit,
        )?;

        let mut offset = time_len;
        for value_index in 0..width {
            offset = skip_filesource_data_delimiters(trimmed_start, offset);
            if offset >= trimmed_start.len() {
                break;
            }
            let Some((value, len)) = parse_filesource_value_token(
                file,
                line_number,
                &trimmed_start[offset..],
                value_index,
            )?
            else {
                break;
            };
            push_filesource_field(
                file,
                &mut fields,
                RawFileSourceField {
                    line: line_number,
                    value,
                },
                value_limit,
            )?;
            offset += len;
        }
    }

    if fields.len() < expected_columns {
        return Err(filesource_error(file, "file contains no data rows"));
    }

    Ok(fields)
}

fn parse_filesource_file(file: &str, width: usize) -> CmResult<Vec<RawFileSourceField>> {
    let contents = data_file::read_to_string(file).map_err(|err| filesource_error(file, err))?;
    parse_filesource_contents(file, width, &contents)
}

fn load_filesource(
    file: &str,
    width: usize,
) -> CmResult<(
    Arc<Vec<RawFileSourceField>>,
    Option<data_file::DataFileStamp>,
)> {
    load_filesource_limited(file, width, crate::resource::ResourceLimits::default())
}

fn load_filesource_limited(
    file: &str,
    width: usize,
    resource_limits: crate::resource::ResourceLimits,
) -> CmResult<(
    Arc<Vec<RawFileSourceField>>,
    Option<data_file::DataFileStamp>,
)> {
    let (contents, stamp) =
        data_file::read_to_string_with_stamp_limited(file, resource_limits.max_external_data_bytes)
            .map_err(|err| filesource_error(file, err))?;
    let virtual_stamp = data_file::loaded_virtual_data_file_stamp(stamp);
    let key = cache_key(file, width, stamp);
    {
        let mut guard = lock_filesource_cache();
        guard.sync_virtual_epoch();
        if let Some(fields) = guard.entries.get(&key) {
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::ExternalDataValues,
                fields.len(),
                resource_limits.max_external_data_values,
            )
            .map_err(|error| filesource_error(file, error.to_string()))?;
            return Ok((fields.clone(), virtual_stamp));
        }
    }

    let fields = Arc::new(parse_filesource_contents_limited(
        file,
        width,
        &contents,
        resource_limits.max_external_data_values,
    )?);
    let mut guard = lock_filesource_cache();
    guard.sync_virtual_epoch();
    guard.entries.insert(key, fields.clone());
    Ok((fields, virtual_stamp))
}

fn filesource_resource_fields(
    ctx: &CmContext,
    file: &str,
    width: usize,
) -> Option<(
    Arc<Vec<RawFileSourceField>>,
    Option<data_file::DataFileStamp>,
)> {
    let resource = ctx.resource::<FileSourceRowsResource>(FILESOURCE_ROWS_RESOURCE)?;
    let virtual_stamp = data_file::virtual_data_file_stamp(file);
    (resource.file == file && resource.width == width && resource.virtual_stamp == virtual_stamp)
        .then(|| (Arc::clone(&resource.fields), resource.virtual_stamp))
}

fn load_filesource_for_context(
    ctx: &mut CmContext,
    file: &str,
    width: usize,
) -> CmResult<(
    Arc<Vec<RawFileSourceField>>,
    Option<data_file::DataFileStamp>,
)> {
    if let Some(fields) = filesource_resource_fields(ctx, file, width) {
        return Ok(fields);
    }

    let (fields, virtual_stamp) = load_filesource_limited(file, width, ctx.resource_limits())?;
    ctx.set_resource(
        FILESOURCE_ROWS_RESOURCE,
        Arc::new(FileSourceRowsResource {
            file: file.to_string(),
            width,
            virtual_stamp,
            fields: Arc::clone(&fields),
        }),
    );
    Ok((fields, virtual_stamp))
}

fn load_filesource_from_context(
    ctx: &CmContext,
    file: &str,
    width: usize,
) -> CmResult<(
    Arc<Vec<RawFileSourceField>>,
    Option<data_file::DataFileStamp>,
)> {
    filesource_resource_fields(ctx, file, width).map_or_else(
        || load_filesource_limited(file, width, ctx.resource_limits()),
        Ok,
    )
}

fn transformed_rows_signature(
    ctx: &CmContext,
    file: &str,
    width: usize,
    virtual_stamp: Option<data_file::DataFileStamp>,
) -> CmResult<FileSourceTransformedRowsSignature> {
    Ok(FileSourceTransformedRowsSignature {
        file: file.to_string(),
        width,
        virtual_stamp,
        timeoffset: ctx.param("timeoffset"),
        timescale: ctx.param("timescale"),
        timerelative: bool_param(ctx, "timerelative")?,
        amploffset_revision: ctx.real_vector_param_revision("amploffset"),
        amplscale_revision: ctx.real_vector_param_revision("amplscale"),
    })
}

fn transformed_rows_signature_matches(
    ctx: &CmContext,
    signature: &FileSourceTransformedRowsSignature,
    file: &str,
    width: usize,
    virtual_stamp: Option<data_file::DataFileStamp>,
) -> CmResult<bool> {
    Ok(signature.file == file
        && signature.width == width
        && signature.virtual_stamp == virtual_stamp
        && signature.timeoffset == ctx.param("timeoffset")
        && signature.timescale == ctx.param("timescale")
        && signature.timerelative == bool_param(ctx, "timerelative")?
        && ctx.real_vector_param_revision("amploffset") == signature.amploffset_revision
        && ctx.real_vector_param_revision("amplscale") == signature.amplscale_revision)
}

fn transformed_rows_for_context(
    ctx: &mut CmContext,
    file: &str,
    width: usize,
) -> CmResult<Arc<FileSourceRowsData>> {
    let (raw_fields, virtual_stamp) = load_filesource_for_context(ctx, file, width)?;
    if let Some(resource) =
        ctx.resource::<FileSourceTransformedRowsResource>(FILESOURCE_TRANSFORMED_ROWS_RESOURCE)
        && transformed_rows_signature_matches(ctx, &resource.signature, file, width, virtual_stamp)?
    {
        return resource.rows.clone();
    }

    let signature = transformed_rows_signature(ctx, file, width, virtual_stamp)?;
    let rows = transform_rows(ctx, &raw_fields, width).map(Arc::new);
    ctx.set_resource(
        FILESOURCE_TRANSFORMED_ROWS_RESOURCE,
        Arc::new(FileSourceTransformedRowsResource {
            signature,
            rows: rows.clone(),
        }),
    );
    rows
}

fn transformed_rows_from_context(
    ctx: &CmContext,
    file: &str,
    width: usize,
) -> CmResult<Arc<FileSourceRowsData>> {
    let (raw_fields, virtual_stamp) = load_filesource_from_context(ctx, file, width)?;
    if let Some(resource) =
        ctx.resource::<FileSourceTransformedRowsResource>(FILESOURCE_TRANSFORMED_ROWS_RESOURCE)
        && transformed_rows_signature_matches(ctx, &resource.signature, file, width, virtual_stamp)?
    {
        return resource.rows.clone();
    }

    transform_rows(ctx, &raw_fields, width).map(Arc::new)
}

fn finite_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(invalid_filesource_param(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    Ok(value)
}

fn bool_param(ctx: &CmContext, name: &str) -> CmResult<bool> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(invalid_filesource_param(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    Ok(value != 0.0)
}

fn vector_transform_param<'a>(ctx: &'a CmContext, name: &str) -> CmResult<&'a [Value]> {
    let values = ctx.real_vector_param(name).unwrap_or(&[]);
    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(invalid_filesource_param(
                name,
                format!("element {index} must be finite, got {value}"),
            ));
        }
    }
    Ok(values)
}

fn transform_rows(
    ctx: &CmContext,
    raw_fields: &[RawFileSourceField],
    width: usize,
) -> CmResult<FileSourceRowsData> {
    let timeoffset = finite_param(ctx, "timeoffset")?;
    let timescale = finite_param(ctx, "timescale")?;
    let timerelative = bool_param(ctx, "timerelative")?;
    let amplscale = vector_transform_param(ctx, "amplscale")?;
    let amploffset = vector_transform_param(ctx, "amploffset")?;

    let file = ctx.string_param("file").unwrap_or("filesource.txt");
    let expected_columns = filesource_expected_columns(file, width)?;
    let row_capacity = raw_fields.len() / expected_columns;
    let value_capacity = filesource_value_capacity(file, row_capacity, width)?;
    let mut rows = Vec::new();
    let mut values = Vec::new();
    reserve_filesource_rows(file, row_capacity, &mut rows)?;
    reserve_filesource_values(file, value_capacity, &mut values)?;
    let mut row_time = 0.0;
    let mut row_column = 0;
    let mut previous_time = timeoffset;
    for raw in raw_fields {
        let value = if row_column == 0 {
            let time_delta = raw.value * timescale;
            let time = if timerelative {
                previous_time += time_delta;
                previous_time
            } else {
                timeoffset + time_delta
            };
            if !time.is_finite() {
                return Err(filesource_line_error(
                    file,
                    raw.line,
                    format!("transformed time must be finite, got {time}"),
                ));
            }
            time
        } else {
            let index = row_column - 1;
            let scale = amplscale.get(index).copied().unwrap_or(1.0);
            let offset = amploffset.get(index).copied().unwrap_or(0.0);
            raw.value * scale + offset
        };

        if row_column == 0 {
            row_time = value;
        } else {
            values.push(value);
        }
        row_column += 1;
        if row_column == expected_columns {
            rows.push(FileSourceRow { time: row_time });
            row_column = 0;
        }
    }
    let retained_values = filesource_value_capacity(file, rows.len(), width)?;
    values.truncate(retained_values);

    if rows.is_empty() {
        return Err(filesource_error(
            ctx.string_param("file").unwrap_or("filesource.txt"),
            "file contains no data rows",
        ));
    }

    Ok(FileSourceRowsData::new(rows, values, width))
}

fn schedule_next_breakpoint_with_hint(
    ctx: &mut CmContext,
    data: &FileSourceRowsData,
    monotonic_next_index: Option<usize>,
) {
    if ctx.analysis != AnalysisType::Transient {
        return;
    }
    if ctx.evaluation_phase() == EvaluationPhase::RollbackableProbe {
        return;
    }

    if data.strictly_increasing_time {
        let index = monotonic_next_index
            .unwrap_or_else(|| data.rows.partition_point(|row| row.time <= ctx.time));
        if let Some(row) = data.rows.get(index) {
            ctx.request_breakpoint(row.time);
        }
        return;
    }

    if let Some(time) = data.next_nonmonotonic_breakpoint(ctx.time) {
        ctx.request_breakpoint(time);
    }
}

fn schedule_next_breakpoint(ctx: &mut CmContext, data: &FileSourceRowsData) {
    schedule_next_breakpoint_with_hint(ctx, data, None);
}

fn filesource_cursor(ctx: &CmContext, rows: &[FileSourceRow]) -> usize {
    let max = rows.len().saturating_sub(1);
    let cursor = ctx.int_state(FILESOURCE_ROW_CURSOR_STATE);
    if cursor < 0 {
        0
    } else {
        (cursor as usize).min(max)
    }
}

fn commit_filesource_cursor(ctx: &mut CmContext, cursor: usize) {
    if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
        ctx.set_int_state(FILESOURCE_ROW_CURSOR_STATE, cursor as i64);
    }
}

fn locate_strictly_increasing_filesource_interval(
    ctx: &mut CmContext,
    rows: &[FileSourceRow],
) -> usize {
    let cursor = filesource_cursor(ctx, rows);
    let lower = if rows[cursor].time < ctx.time {
        if cursor + 1 >= rows.len() || ctx.time <= rows[cursor + 1].time {
            cursor
        } else {
            let advanced = rows[cursor + 1..].partition_point(|row| row.time < ctx.time);
            cursor + advanced
        }
    } else {
        let upper = rows[..=cursor].partition_point(|row| row.time < ctx.time);
        upper.saturating_sub(1).min(rows.len() - 1)
    };
    commit_filesource_cursor(ctx, lower);
    lower
}

fn locate_filesource_interval(ctx: &mut CmContext, data: &FileSourceRowsData) -> usize {
    let rows = data.rows.as_slice();
    if data.strictly_increasing_time {
        return locate_strictly_increasing_filesource_interval(ctx, rows);
    }

    let mut lower = filesource_cursor(ctx, rows);

    while lower > 0 && ctx.time < rows[lower].time {
        lower -= 1;
    }
    while lower + 1 < rows.len() && ctx.time > rows[lower + 1].time {
        lower += 1;
    }

    commit_filesource_cursor(ctx, lower);
    lower
}

fn set_filesource_output_from_row(
    ctx: &mut CmContext,
    data: &FileSourceRowsData,
    row: usize,
) -> CmResult<()> {
    let values = data.row_values(row);
    ctx.set_output_vector_from_slice("out", values)?;
    Ok(())
}

fn evaluate_rows_into_output(
    ctx: &mut CmContext,
    data: &FileSourceRowsData,
    step_mode: bool,
) -> CmResult<()> {
    let rows = data.rows.as_slice();

    let first = &rows[0];
    if ctx.time < first.time {
        schedule_next_breakpoint_with_hint(ctx, data, Some(0));
        commit_filesource_cursor(ctx, 0);
        let row = if rows.len() > 1 { 1 } else { 0 };
        set_filesource_output_from_row(ctx, data, row)?;
        return Ok(());
    }
    if ctx.time == first.time {
        schedule_next_breakpoint_with_hint(ctx, data, Some(1));
        commit_filesource_cursor(ctx, 0);
        set_filesource_output_from_row(ctx, data, 0)?;
        return Ok(());
    }

    let lower = locate_filesource_interval(ctx, data);
    if data.strictly_increasing_time {
        let next = lower + 1;
        let next = if rows.get(next).is_some_and(|row| row.time <= ctx.time) {
            next + 1
        } else {
            next
        };
        schedule_next_breakpoint_with_hint(ctx, data, Some(next));
    } else {
        schedule_next_breakpoint(ctx, data);
    }
    if lower + 1 >= rows.len() {
        set_filesource_output_from_row(ctx, data, lower)?;
        return Ok(());
    }

    let lower_row = &rows[lower];
    let upper_row = &rows[lower + 1];

    if step_mode {
        set_filesource_output_from_row(ctx, data, lower)?;
        return Ok(());
    }

    let span = upper_row.time - lower_row.time;
    let alpha = if span > 0.0 {
        (ctx.time - lower_row.time) / span
    } else {
        1.0
    };
    let lower_values = data.row_values(lower);
    let upper_values = data.row_values(lower + 1);
    debug_assert_eq!(lower_values.len(), data.width);
    debug_assert_eq!(upper_values.len(), data.width);
    ctx.set_output_vector_from_fn("out", data.width, |index| {
        let lower = lower_values[index];
        let upper = upper_values[index];
        lower + alpha * (upper - lower)
    })?;
    Ok(())
}

#[cfg(test)]
fn evaluate_rows(ctx: &mut CmContext, data: &FileSourceRowsData) -> CmResult<Vec<Value>> {
    let step_mode = bool_param(ctx, "amplstep")?;
    evaluate_rows_into_output(ctx, data, step_mode)?;
    Ok(ctx.output_vector("out"))
}

fn filesource_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![PortSpec {
            name: "out".to_string(),
            direction: PortDirection::Out,
            default_type: PortType::Voltage,
            allowed_types: vec![
                PortType::Voltage,
                PortType::DifferentialVoltage,
                PortType::Current,
                PortType::DifferentialCurrent,
            ],
            is_vector: true,
            null_allowed: false,
            vector_min_len: Some(1),
            vector_max_len: None,
            description: "Analog output vector".to_string(),
        }]
    })
}

fn filesource_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::string("file", "filesource.txt")
                .with_description("Stimulus file containing time followed by output values"),
            ParamSpec::real("timeoffset", 0.0)
                .with_description("Absolute time offset applied after scaling"),
            ParamSpec::real("timescale", 1.0).with_description("Scale factor for file time values"),
            ParamSpec::boolean("timerelative", false)
                .with_description("Treat file time values as deltas from the previous row"),
            ParamSpec::boolean("amplstep", false)
                .with_description("Hold the lower-row value instead of interpolating"),
            ParamSpec::real_vector("amploffset", Vec::new())
                .with_vector_min_len(1)
                .with_description("Per-output amplitude offsets"),
            ParamSpec::real_vector("amplscale", Vec::new())
                .with_vector_min_len(1)
                .with_description("Per-output amplitude scales"),
        ]
    })
}

impl CodeModel for FileSource {
    fn name(&self) -> &str {
        "filesource"
    }

    fn description(&self) -> &str {
        "File-backed analog vector source"
    }

    fn ports(&self) -> &[PortSpec] {
        filesource_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        filesource_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let width = ctx.port_width("out");
        let file = ctx
            .string_param("file")
            .unwrap_or("filesource.txt")
            .to_string();
        let rows = transformed_rows_for_context(ctx, &file, width)?;
        bool_param(ctx, "amplstep")?;
        ctx.allocate_int_states(1);
        ctx.set_int_state(FILESOURCE_ROW_CURSOR_STATE, 0);
        set_filesource_output_from_row(ctx, &rows, 0)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.analysis == AnalysisType::Ac {
            return Ok(());
        }

        let width = ctx.port_width("out");
        let file = ctx
            .string_param("file")
            .unwrap_or("filesource.txt")
            .to_string();
        let rows = transformed_rows_for_context(ctx, &file, width)?;
        let step_mode = bool_param(ctx, "amplstep")?;
        evaluate_rows_into_output(ctx, &rows, step_mode)
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        Vec::new()
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port == "out"
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        let width = ctx.port_width("out");
        let file = ctx.string_param("file").unwrap_or("filesource.txt");
        let rows = transformed_rows_from_context(ctx, file, width)?;
        Ok(rows.transient_breakpoints.clone())
    }
}

impl CodeModel for FileSourceAlias {
    fn name(&self) -> &str {
        "file_source"
    }

    fn description(&self) -> &str {
        FileSource.description()
    }

    fn ports(&self) -> &[PortSpec] {
        FileSource.ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        FileSource.parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        FileSource.init(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        FileSource.evaluate(ctx)
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        FileSource.ac_gain(ctx)
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        FileSource.excludes_output_from_transient_voltage_lte(output_port)
    }

    fn transient_breakpoints(&self, ctx: &CmContext) -> CmResult<Vec<Value>> {
        FileSource.transient_breakpoints(ctx)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data_file_test_guard() -> std::sync::MutexGuard<'static, ()> {
        data_file::test_registry_guard()
    }

    fn single_value_rows(rows: &[(Value, Value)]) -> FileSourceRowsData {
        FileSourceRowsData::new(
            rows.iter()
                .map(|(time, _)| FileSourceRow { time: *time })
                .collect(),
            rows.iter().map(|(_, value)| *value).collect(),
            1,
        )
    }

    fn filesource_context(file: &str) -> CmContext {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_port_width("out", 1);
        ctx.set_string_param("file", file);
        ctx.set_param("timeoffset", 0.0);
        ctx.set_param("timescale", 1.0);
        ctx.set_param("timerelative", 0.0);
        ctx.set_param("amplstep", 0.0);
        ctx
    }

    fn poison_filesource_cache_lock() {
        let result = std::panic::catch_unwind(|| {
            let _guard = lock_filesource_cache();
            panic!("poison filesource cache lock for recovery test");
        });
        assert!(result.is_err(), "recovery test must poison the mutex");
    }

    #[test]
    fn filesource_cache_lock_recovers_after_poison() {
        poison_filesource_cache_lock();
        lock_filesource_cache().clear();
    }

    #[test]
    fn filesource_context_enforces_its_engine_resource_policy() {
        let _guard = data_file_test_guard();
        let file = "virtual://filesource/resource-policy";
        let contents = "0 1\n1 2\n";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(file, contents).expect("register filesource data");
        let mut ctx = filesource_context(file);
        let mut limits = crate::resource::ResourceLimits::default();
        limits.max_external_data_bytes = contents.len() - 1;
        ctx.set_resource_limits(limits);

        let error = FileSource
            .init(&mut ctx)
            .expect_err("filesource must inherit its context's byte limit");
        assert!(
            error
                .to_string()
                .contains("external_data_bytes limit exceeded")
        );
        data_file::unregister_data_file(file).expect("unregister filesource data");
    }

    #[test]
    fn filesource_delimiter_skip_uses_ascii_byte_delimiters() {
        let line = "0,\t \r\n1";

        assert_eq!(skip_filesource_data_delimiters(line, 1), 6);
        assert_eq!(&line[6..], "1");
    }

    #[test]
    fn filesource_value_parse_reports_indexed_nonfinite_value() {
        let err = parse_filesource_contents("stim.txt", 2, "0 1 Inf\n")
            .expect_err("nonfinite filesource value should fail");
        let message = err.to_string();

        assert!(
            message.contains("line 1: value[1] must be finite, got Inf"),
            "filesource value error should identify the failing value index, got {message}"
        );
    }

    #[test]
    fn filesource_rejects_nonfinite_boolean_params() {
        let _guard = data_file_test_guard();
        let file = "virtual://filesource/nonfinite-bool-params";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(file, "0 1\n1e-9 2\n").expect("register filesource data");

        for (param, value) in [("timerelative", f64::NAN), ("amplstep", f64::INFINITY)] {
            let mut ctx = filesource_context(file);
            ctx.set_param(param, value);

            let err = FileSource
                .init(&mut ctx)
                .expect_err("filesource must reject nonfinite boolean params during init");
            assert!(
                matches!(&err, CmError::InvalidParameter { .. }),
                "filesource {param} produced {err:?}"
            );
            assert!(
                err.to_string().contains(param),
                "filesource error should name rejected parameter {param}: {err}"
            );
        }

        let mut ctx = filesource_context(file);
        FileSource.init(&mut ctx).expect("filesource initializes");
        ctx.set_param("amplstep", f64::NAN);
        FileSource
            .evaluate(&mut ctx)
            .expect_err("mutated nonfinite amplstep must fail during evaluation");

        let mut ctx = filesource_context(file);
        ctx.set_param("timerelative", f64::INFINITY);
        FileSource
            .transient_breakpoints(&ctx)
            .expect_err("nonfinite timerelative must fail during breakpoint generation");

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn filesource_interval_cursor_advances_and_rolls_back_like_ngspice_pointer() {
        let rows = single_value_rows(&[(0.0, 0.0), (1.0e-9, 1.0), (2.0e-9, 2.0)]);
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.allocate_int_states(1);

        ctx.time = 1.5e-9;
        let mid = evaluate_rows(&mut ctx, &rows).expect("filesource rows evaluate");
        assert!((mid[0] - 1.5).abs() < 1.0e-12);
        assert_eq!(ctx.int_state(FILESOURCE_ROW_CURSOR_STATE), 1);

        ctx.time = 0.5e-9;
        let back = evaluate_rows(&mut ctx, &rows).expect("filesource rows evaluate");
        assert!((back[0] - 0.5).abs() < 1.0e-12);
        assert_eq!(ctx.int_state(FILESOURCE_ROW_CURSOR_STATE), 0);
    }

    #[test]
    fn filesource_streams_output_and_preserves_previous_vector_value() {
        let rows = single_value_rows(&[(0.0, 0.0), (1.0e-9, 2.0), (2.0e-9, 4.0)]);
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_port_width("out", 1);
        ctx.allocate_int_states(1);
        ctx.set_output_vector("out", vec![10.0])
            .expect("seed filesource output");

        ctx.time = 0.5e-9;
        evaluate_rows_into_output(&mut ctx, &rows, false).expect("filesource rows evaluate");
        assert_eq!(ctx.output_vector("out"), vec![1.0]);
        assert_eq!(ctx.output_vector_prev("out"), vec![10.0]);

        ctx.time = 1.5e-9;
        evaluate_rows_into_output(&mut ctx, &rows, false).expect("filesource rows evaluate");
        assert_eq!(ctx.output_vector("out"), vec![3.0]);
        assert_eq!(ctx.output_vector_prev("out"), vec![1.0]);
    }

    #[test]
    fn filesource_interval_cursor_does_not_commit_rollbackable_probe() {
        let rows = single_value_rows(&[(0.0, 0.0), (1.0e-9, 1.0), (2.0e-9, 2.0)]);
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.allocate_int_states(1);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);

        ctx.time = 0.5e-9;
        let out = evaluate_rows(&mut ctx, &rows).expect("filesource rows evaluate");
        assert_eq!(out, vec![0.5]);
        assert_eq!(ctx.int_state(FILESOURCE_ROW_CURSOR_STATE), 0);
        assert!(
            ctx.take_requested_breakpoints().is_empty(),
            "rollbackable filesource probes must not leave transient breakpoints behind"
        );
    }

    #[test]
    fn filesource_breakpoint_lookup_uses_monotonic_row_index() {
        let rows = single_value_rows(&[(0.0, 0.0), (1.0e-9, 1.0), (2.0e-9, 2.0)]);
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.5e-9;

        assert!(rows.strictly_increasing_time);
        assert_eq!(
            rows.transient_breakpoints.as_slice(),
            &[0.0, 1.0e-9, 2.0e-9],
            "monotonic filesource data should precompute row transient breakpoint times"
        );
        schedule_next_breakpoint(&mut ctx, &rows);
        assert_eq!(ctx.take_requested_breakpoints(), vec![1.0e-9]);
    }

    #[test]
    fn filesource_breakpoint_lookup_preserves_nonmonotonic_min_future_time() {
        let rows = single_value_rows(&[
            (0.0, 0.0),
            (2.0e-9, 2.0),
            (1.0e-9, 1.0),
            (2.0e-9, 20.0),
            (3.0e-9, 3.0),
        ]);
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.time = 0.5e-9;

        assert!(!rows.strictly_increasing_time);
        assert_eq!(
            rows.transient_breakpoints.as_slice(),
            &[0.0, 1.0e-9, 2.0e-9, 3.0e-9],
            "nonmonotonic filesource data should precompute sorted unique transient breakpoint times"
        );
        schedule_next_breakpoint(&mut ctx, &rows);
        assert_eq!(ctx.take_requested_breakpoints(), vec![1.0e-9]);

        ctx.time = 1.0e-9;
        schedule_next_breakpoint(&mut ctx, &rows);
        assert_eq!(ctx.take_requested_breakpoints(), vec![2.0e-9]);
    }

    #[test]
    fn filesource_monotonic_interval_lookup_keeps_previous_row_at_exact_step_time() {
        let rows = single_value_rows(&[(0.0, 0.0), (1.0e-9, 1.0), (2.0e-9, 2.0)]);
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.allocate_int_states(1);
        ctx.set_param("amplstep", 1.0);
        ctx.time = 1.0e-9;

        let out = evaluate_rows(&mut ctx, &rows).expect("filesource rows evaluate");
        assert_eq!(out, vec![0.0]);
        assert_eq!(ctx.int_state(FILESOURCE_ROW_CURSOR_STATE), 0);
        assert_eq!(
            ctx.take_requested_breakpoints(),
            vec![2.0e-9],
            "exact row times should hold the previous row while scheduling the next future row"
        );
    }

    #[test]
    fn filesource_preserves_ngspice_flat_stream_alignment_for_short_rows() {
        let fields = parse_filesource_contents(
            "stim.txt",
            2,
            "\
0 10
1 20 30
",
        )
        .expect("ngspice retains fields parsed before a short row");
        assert_eq!(
            fields.iter().map(|field| field.value).collect::<Vec<_>>(),
            vec![0.0, 10.0, 1.0, 20.0, 30.0]
        );
    }

    #[test]
    fn filesource_context_resource_reloads_when_virtual_file_changes() {
        let _guard = data_file_test_guard();
        let file = "virtual://filesource/context-resource-reload";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(file, "0 1\n1e-9 3\n")
            .expect("register first virtual filesource data");

        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_port_width("out", 1);
        ctx.set_string_param("file", file);
        ctx.set_param("timeoffset", 0.0);
        ctx.set_param("timescale", 1.0);
        ctx.set_param("timerelative", 0.0);
        ctx.set_param("amplstep", 0.0);
        ctx.set_real_vector_param("amploffset", vec![0.0]);
        ctx.set_real_vector_param("amplscale", vec![1.0]);

        FileSource.init(&mut ctx).expect("filesource init");
        ctx.time = 0.5e-9;
        FileSource.evaluate(&mut ctx).expect("filesource evaluate");
        assert!((ctx.output_vector("out")[0] - 2.0).abs() < 1.0e-12);

        data_file::register_data_file(file, "0 10\n1e-9 12\n")
            .expect("replace virtual filesource data");
        FileSource
            .evaluate(&mut ctx)
            .expect("filesource reevaluates after virtual replace");
        assert!((ctx.output_vector("out")[0] - 11.0).abs() < 1.0e-12);

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn filesource_transformed_rows_cache_reloads_when_transform_params_change() {
        let _guard = data_file_test_guard();
        let file = "virtual://filesource/transformed-row-cache";
        let _ = data_file::unregister_data_file(file);
        lock_filesource_cache().clear();
        data_file::register_data_file(file, "0 1\n1e-9 3\n").expect("register filesource data");

        let mut ctx = CmContext::new();
        ctx.set_port_width("out", 1);
        ctx.set_string_param("file", file);
        ctx.set_param("timeoffset", 0.0);
        ctx.set_param("timescale", 1.0);
        ctx.set_param("timerelative", 0.0);
        ctx.set_real_vector_param("amploffset", vec![0.0]);
        ctx.set_real_vector_param("amplscale", vec![1.0]);

        let first = transformed_rows_for_context(&mut ctx, file, 1).expect("transform rows");
        let second =
            transformed_rows_for_context(&mut ctx, file, 1).expect("reuse transformed rows");
        assert!(Arc::ptr_eq(&first, &second));

        ctx.set_real_vector_param("unrelated", vec![4.0, 5.0]);
        let after_unrelated =
            transformed_rows_for_context(&mut ctx, file, 1).expect("reuse after unrelated vector");
        assert!(Arc::ptr_eq(&first, &after_unrelated));

        ctx.set_param("timeoffset", 5.0);
        let shifted = transformed_rows_for_context(&mut ctx, file, 1).expect("reload shifted rows");
        assert!(!Arc::ptr_eq(&first, &shifted));
        assert_eq!(shifted[0].time, 5.0);

        ctx.set_real_vector_param("amplscale", vec![10.0]);
        let scaled = transformed_rows_for_context(&mut ctx, file, 1).expect("reload scaled rows");
        assert!(!Arc::ptr_eq(&shifted, &scaled));
        assert_eq!(scaled.row_values(0), &[10.0]);

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn filesource_cache_retires_replaced_virtual_file_entries() {
        let _guard = data_file_test_guard();
        let file = "virtual://filesource/cache-retention";
        let _ = data_file::unregister_data_file(file);
        lock_filesource_cache().clear();

        data_file::register_data_file(file, "0 1\n1e-9 2\n")
            .expect("register first virtual filesource data");
        let (first, _) = load_filesource(file, 1).expect("load first virtual filesource data");
        let mut ctx = CmContext::new();
        ctx.set_string_param("file", file);
        ctx.set_param("timeoffset", 0.0);
        ctx.set_param("timescale", 1.0);
        ctx.set_param("timerelative", 0.0);
        ctx.set_real_vector_param("amploffset", vec![0.0]);
        ctx.set_real_vector_param("amplscale", vec![1.0]);
        let first_rows = transform_rows(&ctx, &first, 1).expect("transform first rows");
        assert_eq!(first_rows.row_values(1), &[2.0]);

        data_file::register_data_file(file, "0 10\n1e-9 20\n")
            .expect("replace virtual filesource data");
        let (second, _) = load_filesource(file, 1).expect("load replaced virtual filesource data");
        let second_rows = transform_rows(&ctx, &second, 1).expect("transform replaced rows");
        assert_eq!(second_rows.row_values(1), &[20.0]);

        let cached_for_file = {
            let guard = lock_filesource_cache();
            guard.entries.keys().filter(|key| key.file == file).count()
        };
        assert_eq!(cached_for_file, 1);

        let _ = data_file::unregister_data_file(file);
    }
}

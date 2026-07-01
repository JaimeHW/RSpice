//! Official file-backed XSPICE `table2d` and `table3d` code models.

use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
    data_file,
};
use crate::{Complex64, Value};
use std::collections::HashMap;
use std::sync::{
    Arc, Mutex, MutexGuard, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

const BOUNDARY_DERIVATIVE_RAMP_FRACTION: Value = 0.125;
const TABLE2D_RESOURCE: &str = "xspice.table2d.data";
const TABLE3D_RESOURCE: &str = "xspice.table3d.data";
const TABLE2D_EVAL_RESOURCE: &str = "xspice.table2d.eval";
const TABLE3D_EVAL_RESOURCE: &str = "xspice.table3d.eval";
const TABLE_EVAL_SCRATCH_RESOURCE: &str = "xspice.table.eval_scratch";
const AXIS_UNSET_ENO_INDEX: usize = usize::MAX;
const AXIS_CURSOR_LINEAR_STEPS: usize = 8;

#[derive(Debug, Default)]
pub struct Table2D;

#[derive(Debug, Default)]
pub struct Table3D;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum TableKind {
    Table2D,
    Table3D,
}

impl TableKind {
    fn model_name(self) -> &'static str {
        match self {
            TableKind::Table2D => "table2d",
            TableKind::Table3D => "table3d",
        }
    }

    fn default_file(self) -> &'static str {
        match self {
            TableKind::Table2D => "2D-table-model.txt",
            TableKind::Table3D => "3D-table-model.txt",
        }
    }
}

#[derive(Debug)]
struct TableAxis {
    values: Vec<Value>,
    local_diffs: Vec<Value>,
    inverse_spans: Vec<Value>,
    lower_ramp: Value,
    upper_ramp: Value,
    last_eno_index: AtomicUsize,
}

impl Clone for TableAxis {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            local_diffs: self.local_diffs.clone(),
            inverse_spans: self.inverse_spans.clone(),
            lower_ramp: self.lower_ramp,
            upper_ramp: self.upper_ramp,
            last_eno_index: AtomicUsize::new(AXIS_UNSET_ENO_INDEX),
        }
    }
}

impl TableAxis {
    fn new(values: Vec<Value>) -> Self {
        debug_assert!(values.len() >= 2);

        let inverse_spans = values
            .windows(2)
            .map(|window| 1.0 / (window[1] - window[0]))
            .collect();

        let last_index = values.len() - 1;
        let local_diffs = (0..values.len())
            .map(|index| {
                if index == 0 {
                    values[1] - values[0]
                } else if index >= last_index {
                    values[last_index] - values[last_index - 1]
                } else {
                    0.5 * (values[index + 1] - values[index - 1])
                }
            })
            .collect();

        let lower_ramp = BOUNDARY_DERIVATIVE_RAMP_FRACTION * (values[1] - values[0]);
        let upper_ramp =
            BOUNDARY_DERIVATIVE_RAMP_FRACTION * (values[last_index] - values[last_index - 1]);

        Self {
            values,
            local_diffs,
            inverse_spans,
            lower_ramp,
            upper_ramp,
            last_eno_index: AtomicUsize::new(AXIS_UNSET_ENO_INDEX),
        }
    }

    fn len(&self) -> usize {
        self.values.len()
    }

    fn values(&self) -> &[Value] {
        &self.values
    }
}

#[derive(Debug, Clone)]
struct TableLine<'a> {
    line: usize,
    header_ignored: bool,
    data_comment: bool,
    tokens: Vec<&'a str>,
}

#[derive(Debug)]
struct TokenCursor<'a> {
    lines: Vec<TableLine<'a>>,
    next: usize,
}

impl<'a> TokenCursor<'a> {
    fn new(lines: Vec<TableLine<'a>>) -> Self {
        Self { lines, next: 0 }
    }

    fn next_line(&mut self, file: &str, model: &str, role: &str) -> CmResult<&TableLine<'a>> {
        self.skip_header_ignored();
        let index = self.next;
        if self.lines.get(index).is_none() {
            return Err(table_file_error(
                model,
                file,
                format!("missing {role}; table file ended early"),
            ));
        }
        self.next += 1;
        Ok(&self.lines[index])
    }

    fn next_data_line_optional(&mut self) -> Option<&TableLine<'a>> {
        loop {
            let index = self.next;
            let data_comment = self.lines.get(index)?.data_comment;
            self.next += 1;
            if data_comment {
                continue;
            }
            return Some(&self.lines[index]);
        }
    }

    fn skip_header_ignored(&mut self) {
        while self
            .lines
            .get(self.next)
            .is_some_and(|line| line.header_ignored)
        {
            self.next += 1;
        }
    }

    fn next_dimension(&mut self, file: &str, model: &str, role: &str) -> CmResult<usize> {
        let line = self.next_line(file, model, role)?;
        let token = line.tokens.first().copied().ok_or_else(|| {
            table_line_error(model, file, line.line, format!("missing {role} token"))
        })?;
        let value = parse_table_spice_value(token);
        let truncated = value.trunc();
        if !value.is_finite() || truncated < 2.0 || truncated > usize::MAX as Value {
            return Err(table_line_error(
                model,
                file,
                line.line,
                format!("{role} must truncate to an integer >= 2, got {}", token),
            ));
        }
        Ok(truncated as usize)
    }

    fn reject_extra_tokens(&self, file: &str, model: &str) -> CmResult<()> {
        if let Some(line) = self
            .lines
            .iter()
            .skip(self.next)
            .find(|line| !line.header_ignored)
        {
            let token = line.tokens.first().copied().unwrap_or("");
            return Err(table_line_error(
                model,
                file,
                line.line,
                format!("unexpected extra token '{token}'"),
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
struct Table2DData {
    x: TableAxis,
    y: TableAxis,
    values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct Table3DData {
    x: TableAxis,
    y: TableAxis,
    z: TableAxis,
    values: Vec<Value>,
}

#[derive(Debug, Clone, Copy)]
struct AxisEval {
    lower: usize,
    upper: usize,
    t: Value,
    derivative_scale: Value,
    eno_index: usize,
    eno_offset: Value,
    local_diff: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TableEval2D {
    value: Value,
    dx: Value,
    dy: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TableEval3D {
    value: Value,
    dx: Value,
    dy: Value,
    dz: Value,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct TableCacheKey {
    file: String,
    len: u64,
    modified_nanos: u128,
    content_hash: u64,
    virtual_file: bool,
}

#[derive(Debug, Clone)]
struct Table2DResource {
    file: String,
    virtual_stamp: Option<data_file::DataFileStamp>,
    table: Arc<Table2DData>,
}

#[derive(Debug, Clone)]
struct Table3DResource {
    file: String,
    virtual_stamp: Option<data_file::DataFileStamp>,
    table: Arc<Table3DData>,
}

#[derive(Debug, Clone, PartialEq)]
struct Table2DEvalSignature {
    table_id: usize,
    order: usize,
    offset: Value,
    gain: Value,
    inx: Value,
    iny: Value,
}

#[derive(Debug, Clone)]
struct Table2DEvalResource {
    signature: Table2DEvalSignature,
    result: TableEval2D,
}

#[derive(Debug, Clone, PartialEq)]
struct Table3DEvalSignature {
    table_id: usize,
    order: usize,
    offset: Value,
    gain: Value,
    inx: Value,
    iny: Value,
    inz: Value,
}

#[derive(Debug, Clone)]
struct Table3DEvalResource {
    signature: Table3DEvalSignature,
    result: TableEval3D,
}

#[derive(Debug, Default)]
struct TableEvalScratch {
    values_along_y: Vec<Value>,
    dx_along_y: Vec<Value>,
    x_scratch: Vec<Value>,
    y_scratch: Vec<Value>,
    values_over_zy: Vec<Value>,
    dx_over_zy: Vec<Value>,
    yz_x_scratch: Vec<Value>,
    yz_y_scratch: Vec<Value>,
}

#[derive(Debug)]
struct TableDataCache<T> {
    virtual_epoch: u64,
    entries: HashMap<TableCacheKey, Arc<T>>,
}

impl<T> Default for TableDataCache<T> {
    fn default() -> Self {
        Self {
            virtual_epoch: 0,
            entries: HashMap::new(),
        }
    }
}

impl<T> TableDataCache<T> {
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

fn table2d_cache() -> &'static Mutex<TableDataCache<Table2DData>> {
    static CACHE: OnceLock<Mutex<TableDataCache<Table2DData>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(TableDataCache::default()))
}

fn table3d_cache() -> &'static Mutex<TableDataCache<Table3DData>> {
    static CACHE: OnceLock<Mutex<TableDataCache<Table3DData>>> = OnceLock::new();
    CACHE.get_or_init(|| Mutex::new(TableDataCache::default()))
}

fn lock_table2d_cache() -> MutexGuard<'static, TableDataCache<Table2DData>> {
    table2d_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn lock_table3d_cache() -> MutexGuard<'static, TableDataCache<Table3DData>> {
    table3d_cache()
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

fn table_file_error(model: &str, file: &str, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("{model} file '{}': {}", file, message.into()))
}

fn table_line_error(model: &str, file: &str, line: usize, message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!(
        "{model} file '{}' line {}: {}",
        file,
        line,
        message.into()
    ))
}

fn invalid_table_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn parse_table_spice_value(token: &str) -> Value {
    data_file::parse_ngspice_spice_value(token)
}

fn table_cache_key(file: &str, stamp: data_file::DataFileStamp) -> TableCacheKey {
    TableCacheKey {
        file: file.to_string(),
        len: stamp.len,
        modified_nanos: stamp.modified_nanos,
        content_hash: stamp.content_hash,
        virtual_file: stamp.virtual_file,
    }
}

fn tokenize_table_contents<'a>(
    model: &str,
    file: &str,
    contents: &'a str,
) -> CmResult<Vec<TableLine<'a>>> {
    let mut lines = Vec::new();

    for (line_idx, line) in contents.lines().enumerate() {
        let line_no = line_idx + 1;
        let header_ignored = line.is_empty() || line.starts_with('*');
        let data_comment = line.starts_with('*');
        let mut tokens = Vec::new();
        if !header_ignored {
            for token in
                line.split(|ch: char| ch.is_whitespace() || matches!(ch, '=' | '(' | ')' | ','))
            {
                if token.is_empty() {
                    continue;
                }
                tokens.push(token);
            }
        }
        lines.push(TableLine {
            line: line_no,
            header_ignored,
            data_comment,
            tokens,
        });
    }

    if !lines.iter().any(|line| !line.header_ignored) {
        return Err(table_file_error(model, file, "contains no table data"));
    }

    Ok(lines)
}

fn parse_axis(
    cursor: &mut TokenCursor<'_>,
    file: &str,
    model: &str,
    role: &str,
    len: usize,
) -> CmResult<TableAxis> {
    let row = cursor.next_line(file, model, role)?;
    if row.tokens.len() > len {
        return Err(table_line_error(
            model,
            file,
            row.line,
            format!("too many numbers in {role} row"),
        ));
    }
    if row.tokens.len() < len {
        return Err(table_line_error(
            model,
            file,
            row.line,
            format!("not enough numbers in {role} row"),
        ));
    }

    let mut axis = Vec::with_capacity(len);
    for (index, token) in row.tokens.iter().copied().enumerate() {
        let value = parse_table_spice_value(token);
        if !value.is_finite() {
            return Err(table_line_error(
                model,
                file,
                row.line,
                format!("{role}[{index}] must be finite, got {token}"),
            ));
        }
        if let Some(previous) = axis.last()
            && value <= *previous
        {
            return Err(table_file_error(
                model,
                file,
                format!("{role} axis values must be strictly increasing"),
            ));
        }
        axis.push(value);
    }
    Ok(TableAxis::new(axis))
}

fn parse_table2d_values(
    cursor: &mut TokenCursor<'_>,
    file: &str,
    model: &str,
    x_len: usize,
    y_len: usize,
) -> CmResult<Vec<Value>> {
    let mut values = Vec::with_capacity(x_len * y_len);
    for row in 0..y_len {
        let Some(table_row) = cursor.next_data_line_optional() else {
            values.resize(values.len() + x_len * (y_len - row), 0.0);
            break;
        };
        if table_row.tokens.len() > x_len {
            return Err(table_line_error(
                model,
                file,
                table_row.line,
                format!("too many numbers in y row no. {}", row + 1),
            ));
        }
        if table_row.tokens.len() < x_len {
            return Err(table_line_error(
                model,
                file,
                table_row.line,
                format!("not enough numbers in y row no. {}", row + 1),
            ));
        }
        for (column, token) in table_row.tokens.iter().copied().enumerate() {
            let value = parse_table_spice_value(token);
            if !value.is_finite() {
                return Err(table_line_error(
                    model,
                    file,
                    table_row.line,
                    format!(
                        "table value row {row} column {column} must be finite, got {}",
                        token
                    ),
                ));
            }
            values.push(value);
        }
    }
    Ok(values)
}

fn parse_table3d_values(
    cursor: &mut TokenCursor<'_>,
    file: &str,
    model: &str,
    x_len: usize,
    y_len: usize,
    z_len: usize,
) -> CmResult<Vec<Value>> {
    let mut values = Vec::with_capacity(x_len * y_len * z_len);
    for z in 0..z_len {
        for y in 0..y_len {
            let table_row = cursor.next_data_line_optional().ok_or_else(|| {
                table_file_error(
                    model,
                    file,
                    format!("missing table {z} row {y}; table file ended early"),
                )
            })?;
            if table_row.tokens.len() > x_len {
                return Err(table_line_error(
                    model,
                    file,
                    table_row.line,
                    format!("too many numbers in y row no. {y} of table {z}"),
                ));
            }
            if table_row.tokens.len() < x_len {
                return Err(table_line_error(
                    model,
                    file,
                    table_row.line,
                    format!("not enough numbers in y row no. {y} of table {z}"),
                ));
            }
            for (x, token) in table_row.tokens.iter().copied().enumerate() {
                let value = parse_table_spice_value(token);
                if !value.is_finite() {
                    return Err(table_line_error(
                        model,
                        file,
                        table_row.line,
                        format!(
                            "table value z {z} row {y} column {x} must be finite, got {}",
                            token
                        ),
                    ));
                }
                values.push(value);
            }
        }
    }
    Ok(values)
}

fn parse_table2d_file(file: &str) -> CmResult<Table2DData> {
    let model = TableKind::Table2D.model_name();
    let contents =
        data_file::read_to_string(file).map_err(|err| table_file_error(model, file, err))?;
    parse_table2d_contents(file, &contents)
}

fn parse_table2d_contents(file: &str, contents: &str) -> CmResult<Table2DData> {
    let model = TableKind::Table2D.model_name();
    let mut cursor = TokenCursor::new(tokenize_table_contents(model, file, contents)?);
    parse_table2d_cursor(file, model, &mut cursor)
}

fn parse_table2d_cursor(
    file: &str,
    model: &str,
    cursor: &mut TokenCursor<'_>,
) -> CmResult<Table2DData> {
    let x_len = cursor.next_dimension(file, model, "x dimension")?;
    let y_len = cursor.next_dimension(file, model, "y dimension")?;
    let x = parse_axis(cursor, file, model, "x", x_len)?;
    let y = parse_axis(cursor, file, model, "y", y_len)?;
    let values = parse_table2d_values(cursor, file, model, x_len, y_len)?;
    cursor.reject_extra_tokens(file, model)?;
    Ok(Table2DData { x, y, values })
}

fn parse_table3d_file(file: &str) -> CmResult<Table3DData> {
    let model = TableKind::Table3D.model_name();
    let contents =
        data_file::read_to_string(file).map_err(|err| table_file_error(model, file, err))?;
    parse_table3d_contents(file, &contents)
}

fn parse_table3d_contents(file: &str, contents: &str) -> CmResult<Table3DData> {
    let model = TableKind::Table3D.model_name();
    let mut cursor = TokenCursor::new(tokenize_table_contents(model, file, contents)?);
    parse_table3d_cursor(file, model, &mut cursor)
}

fn parse_table3d_cursor(
    file: &str,
    model: &str,
    cursor: &mut TokenCursor<'_>,
) -> CmResult<Table3DData> {
    let x_len = cursor.next_dimension(file, model, "x dimension")?;
    let y_len = cursor.next_dimension(file, model, "y dimension")?;
    let z_len = cursor.next_dimension(file, model, "z dimension")?;
    let x = parse_axis(cursor, file, model, "x", x_len)?;
    let y = parse_axis(cursor, file, model, "y", y_len)?;
    let z = parse_axis(cursor, file, model, "z", z_len)?;
    let values = parse_table3d_values(cursor, file, model, x_len, y_len, z_len)?;
    Ok(Table3DData { x, y, z, values })
}

fn table_order(ctx: &CmContext) -> CmResult<usize> {
    let order = ctx.param("order");
    if !order.is_finite() || (order.round() - order).abs() > 1.0e-12 {
        return Err(invalid_table_param("order", "must be a finite integer"));
    }
    Ok(order.round().max(2.0) as usize)
}

fn validate_table2d_order(table: &Table2DData, order: usize) -> CmResult<()> {
    let y_window = 2 * order - 2;
    if order > table.x.len() || y_window > table.y.len() {
        return Err(invalid_table_param(
            "order",
            format!(
                "order {order} exceeds table2d grid: requires at least {order} x-points and {y_window} y-points, got {} x {}",
                table.x.len(),
                table.y.len()
            ),
        ));
    }
    Ok(())
}

fn validate_table3d_order(table: &Table3DData, order: usize) -> CmResult<()> {
    let yz_window = 2 * order - 2;
    if order > table.x.len() || yz_window > table.y.len() || yz_window > table.z.len() {
        return Err(invalid_table_param(
            "order",
            format!(
                "order {order} exceeds table3d grid: requires at least {order} x-points and {yz_window} y/z-points, got {} x {} x {}",
                table.x.len(),
                table.y.len(),
                table.z.len()
            ),
        ));
    }
    Ok(())
}

fn load_table2d(file: &str) -> CmResult<(Arc<Table2DData>, Option<data_file::DataFileStamp>)> {
    let model = TableKind::Table2D.model_name();
    let (contents, stamp) = data_file::read_to_string_with_stamp(file)
        .map_err(|err| table_file_error(model, file, err))?;
    let virtual_stamp = data_file::loaded_virtual_data_file_stamp(stamp);
    let key = table_cache_key(file, stamp);
    {
        let mut guard = lock_table2d_cache();
        guard.sync_virtual_epoch();
        if let Some(table) = guard.entries.get(&key) {
            return Ok((Arc::clone(table), virtual_stamp));
        }
    }

    let table = Arc::new(parse_table2d_contents(file, &contents)?);
    let mut guard = lock_table2d_cache();
    guard.sync_virtual_epoch();
    guard.entries.insert(key, Arc::clone(&table));
    Ok((table, virtual_stamp))
}

fn load_table3d(file: &str) -> CmResult<(Arc<Table3DData>, Option<data_file::DataFileStamp>)> {
    let model = TableKind::Table3D.model_name();
    let (contents, stamp) = data_file::read_to_string_with_stamp(file)
        .map_err(|err| table_file_error(model, file, err))?;
    let virtual_stamp = data_file::loaded_virtual_data_file_stamp(stamp);
    let key = table_cache_key(file, stamp);
    {
        let mut guard = lock_table3d_cache();
        guard.sync_virtual_epoch();
        if let Some(table) = guard.entries.get(&key) {
            return Ok((Arc::clone(table), virtual_stamp));
        }
    }

    let table = Arc::new(parse_table3d_contents(file, &contents)?);
    let mut guard = lock_table3d_cache();
    guard.sync_virtual_epoch();
    guard.entries.insert(key, Arc::clone(&table));
    Ok((table, virtual_stamp))
}

fn table2d_resource(ctx: &CmContext, file: &str) -> Option<Arc<Table2DData>> {
    let resource = ctx.resource::<Table2DResource>(TABLE2D_RESOURCE)?;
    let virtual_stamp = data_file::virtual_data_file_stamp(file);
    (resource.file == file && resource.virtual_stamp == virtual_stamp)
        .then(|| Arc::clone(&resource.table))
}

fn table3d_resource(ctx: &CmContext, file: &str) -> Option<Arc<Table3DData>> {
    let resource = ctx.resource::<Table3DResource>(TABLE3D_RESOURCE)?;
    let virtual_stamp = data_file::virtual_data_file_stamp(file);
    (resource.file == file && resource.virtual_stamp == virtual_stamp)
        .then(|| Arc::clone(&resource.table))
}

fn ensure_table_eval_scratch(ctx: &mut CmContext) {
    if ctx
        .resource::<Mutex<TableEvalScratch>>(TABLE_EVAL_SCRATCH_RESOURCE)
        .is_none()
    {
        ctx.set_resource(
            TABLE_EVAL_SCRATCH_RESOURCE,
            Arc::new(Mutex::new(TableEvalScratch::default())),
        );
    }
}

fn with_table_eval_scratch<R>(
    ctx: &CmContext,
    evaluate: impl FnOnce(&mut TableEvalScratch) -> R,
) -> Option<R> {
    let scratch = ctx.resource::<Mutex<TableEvalScratch>>(TABLE_EVAL_SCRATCH_RESOURCE)?;
    let mut scratch = scratch
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    Some(evaluate(&mut scratch))
}

fn load_table2d_for_context(ctx: &mut CmContext, file: &str) -> CmResult<Arc<Table2DData>> {
    if let Some(table) = table2d_resource(ctx, file) {
        ensure_table_eval_scratch(ctx);
        return Ok(table);
    }

    let (table, virtual_stamp) = load_table2d(file)?;
    ctx.set_resource(
        TABLE2D_RESOURCE,
        Arc::new(Table2DResource {
            file: file.to_string(),
            virtual_stamp,
            table: Arc::clone(&table),
        }),
    );
    ensure_table_eval_scratch(ctx);
    Ok(table)
}

fn load_table3d_for_context(ctx: &mut CmContext, file: &str) -> CmResult<Arc<Table3DData>> {
    if let Some(table) = table3d_resource(ctx, file) {
        ensure_table_eval_scratch(ctx);
        return Ok(table);
    }

    let (table, virtual_stamp) = load_table3d(file)?;
    ctx.set_resource(
        TABLE3D_RESOURCE,
        Arc::new(Table3DResource {
            file: file.to_string(),
            virtual_stamp,
            table: Arc::clone(&table),
        }),
    );
    ensure_table_eval_scratch(ctx);
    Ok(table)
}

fn load_table2d_from_context(ctx: &CmContext, file: &str) -> CmResult<Arc<Table2DData>> {
    table2d_resource(ctx, file).map_or_else(|| load_table2d(file).map(|(table, _)| table), Ok)
}

fn load_table3d_from_context(ctx: &CmContext, file: &str) -> CmResult<Arc<Table3DData>> {
    table3d_resource(ctx, file).map_or_else(|| load_table3d(file).map(|(table, _)| table), Ok)
}

fn axis_eno_contains(values: &[Value], index: usize, clamped: Value) -> bool {
    debug_assert!(index < values.len());
    let last_index = values.len() - 1;
    if index >= last_index {
        clamped >= values[last_index]
    } else {
        values[index] <= clamped && clamped < values[index + 1]
    }
}

fn axis_eno_index_binary(values: &[Value], clamped: Value) -> usize {
    values
        .partition_point(|value| *value <= clamped)
        .saturating_sub(1)
        .min(values.len() - 1)
}

fn axis_eno_index_with_cursor(axis: &TableAxis, clamped: Value) -> usize {
    let values = axis.values();
    let last_index = values.len() - 1;
    let mut index = axis.last_eno_index.load(Ordering::Relaxed);

    if index == AXIS_UNSET_ENO_INDEX || index > last_index {
        index = axis_eno_index_binary(values, clamped);
        axis.last_eno_index.store(index, Ordering::Relaxed);
        return index;
    }

    if axis_eno_contains(values, index, clamped) {
        return index;
    }

    let mut steps = 0;
    if index < last_index && clamped >= values[index + 1] {
        while index < last_index && clamped >= values[index + 1] && steps < AXIS_CURSOR_LINEAR_STEPS
        {
            index += 1;
            steps += 1;
        }
    } else {
        while index > 0 && clamped < values[index] && steps < AXIS_CURSOR_LINEAR_STEPS {
            index -= 1;
            steps += 1;
        }
    }

    if !axis_eno_contains(values, index, clamped) {
        index = axis_eno_index_binary(values, clamped);
    }
    axis.last_eno_index.store(index, Ordering::Relaxed);
    index
}

fn axis_eval(axis: &TableAxis, input: Value) -> AxisEval {
    let values = axis.values();
    let last_index = values.len() - 1;
    let first = values[0];
    let last = values[last_index];

    let derivative_scale = if input < first {
        if axis.lower_ramp > 0.0 && input >= first - axis.lower_ramp {
            ((input - (first - axis.lower_ramp)) / axis.lower_ramp).clamp(0.0, 1.0)
        } else {
            0.0
        }
    } else if input > last {
        if axis.upper_ramp > 0.0 && input <= last + axis.upper_ramp {
            ((last + axis.upper_ramp - input) / axis.upper_ramp).clamp(0.0, 1.0)
        } else {
            0.0
        }
    } else {
        1.0
    };

    let clamped = input.clamp(first, last);
    let eno_index = axis_eno_index_with_cursor(axis, clamped);
    let eno_offset = clamped - values[eno_index];
    let local_diff = axis.local_diffs[eno_index];

    if clamped <= first {
        return AxisEval {
            lower: 0,
            upper: 1,
            t: 0.0,
            derivative_scale,
            eno_index,
            eno_offset,
            local_diff,
        };
    }
    if clamped >= last {
        return AxisEval {
            lower: last_index - 1,
            upper: last_index,
            t: 1.0,
            derivative_scale,
            eno_index,
            eno_offset,
            local_diff,
        };
    }

    let lower = eno_index.min(last_index - 1);
    AxisEval {
        lower,
        upper: lower + 1,
        t: (clamped - values[lower]) * axis.inverse_spans[lower],
        derivative_scale,
        eno_index,
        eno_offset,
        local_diff,
    }
}

fn table2d_value(table: &Table2DData, x_index: usize, y_index: usize) -> Value {
    table.values[y_index * table.x.len() + x_index]
}

fn table2d_row(table: &Table2DData, y_index: usize) -> &[Value] {
    let x_len = table.x.len();
    let start = y_index * x_len;
    &table.values[start..start + x_len]
}

fn table3d_value(table: &Table3DData, x_index: usize, y_index: usize, z_index: usize) -> Value {
    table.values[(z_index * table.y.len() + y_index) * table.x.len() + x_index]
}

fn table3d_x_row(table: &Table3DData, y_index: usize, z_index: usize) -> &[Value] {
    let x_len = table.x.len();
    let start = (z_index * table.y.len() + y_index) * x_len;
    &table.values[start..start + x_len]
}

#[derive(Debug, Clone, Copy)]
struct Eno1dEval {
    value: Value,
    derivative: Value,
}

#[derive(Debug, Clone, Copy)]
struct Eno2dEval {
    value: Value,
    dx: Value,
    dy: Value,
}

#[derive(Debug, Clone, Copy)]
struct Eno3dEval {
    dx: Value,
    dy: Value,
    dz: Value,
}

fn eno_window_start(index: usize, order: usize, len: usize) -> usize {
    let window = 2 * order - 2;
    debug_assert!(window <= len);
    if index + 2 < order {
        0
    } else if index + order > len {
        len - window
    } else {
        index + 2 - order
    }
}

fn eno1d_eval(samples: &[Value], order: usize, index: usize, offset: Value) -> Eno1dEval {
    let mut scratch = Vec::new();
    eno1d_eval_with_scratch(samples, order, index, offset, &mut scratch)
}

fn eno1d_eval_with_scratch(
    samples: &[Value],
    order: usize,
    index: usize,
    offset: Value,
    scratch: &mut Vec<Value>,
) -> Eno1dEval {
    debug_assert!(order >= 2);
    debug_assert!(order <= samples.len());
    debug_assert!(index < samples.len());

    let max_start = samples.len() - order;
    let start_hi = index.min(max_start);
    let start_lo = start_hi.min(index.saturating_sub(order - 2));
    let window_end = start_hi + order;
    let window_len = window_end - start_lo;
    let row_offset =
        |degree: usize| -> usize { degree * window_len - degree.saturating_sub(1) * degree / 2 };

    let total_len = order * window_len - order.saturating_sub(1) * order / 2;
    scratch.resize(total_len, 0.0);
    scratch[..window_len].copy_from_slice(&samples[start_lo..window_end]);
    for degree in 1..order {
        let previous = row_offset(degree - 1);
        let current = row_offset(degree);
        let row_len = window_len - degree;
        for i in 0..row_len {
            scratch[current + i] = scratch[previous + i + 1] - scratch[previous + i];
        }
    }

    let highest_order = row_offset(order - 1);
    let smoothness = (start_lo..=start_hi)
        .map(|start| scratch[highest_order + start - start_lo].abs())
        .fold(Value::INFINITY, Value::min);

    let mut value = 0.0;
    let mut derivative = 0.0;
    let mut stencil_count = 0_usize;

    for start in start_lo..=start_hi {
        let local_start = start - start_lo;
        if scratch[highest_order + local_start].abs() > smoothness {
            continue;
        }
        stencil_count += 1;

        let relative = offset + index as Value - start as Value;
        let mut basis = 1.0;
        let mut basis_derivative = 0.0;

        for degree in 0..order {
            let divided_difference = scratch[row_offset(degree) + local_start];
            value += basis * divided_difference;
            derivative += basis_derivative * divided_difference;

            let degree = degree as Value;
            let denominator = degree + 1.0;
            basis_derivative = (basis + basis_derivative * (relative - degree)) / denominator;
            basis *= (relative - degree) / denominator;
        }
    }

    let scale = 1.0 / stencil_count.max(1) as Value;
    Eno1dEval {
        value: value * scale,
        derivative: derivative * scale,
    }
}

fn eno2d_eval_grid<'a, F>(
    row_count: usize,
    mut row_at: F,
    order: usize,
    x_index: usize,
    y_index: usize,
    x_offset: Value,
    y_offset: Value,
) -> Eno2dEval
where
    F: FnMut(usize) -> &'a [Value],
{
    let mut values_along_y = Vec::new();
    let mut dx_along_y = Vec::new();
    let mut x_scratch = Vec::new();
    let mut y_scratch = Vec::new();
    eno2d_eval_grid_with_scratch(
        row_count,
        &mut row_at,
        order,
        x_index,
        y_index,
        x_offset,
        y_offset,
        &mut values_along_y,
        &mut dx_along_y,
        &mut x_scratch,
        &mut y_scratch,
    )
}

#[allow(clippy::too_many_arguments)]
fn eno2d_eval_grid_with_scratch<'a, F>(
    row_count: usize,
    row_at: &mut F,
    order: usize,
    x_index: usize,
    y_index: usize,
    x_offset: Value,
    y_offset: Value,
    values_along_y: &mut Vec<Value>,
    dx_along_y: &mut Vec<Value>,
    x_scratch: &mut Vec<Value>,
    y_scratch: &mut Vec<Value>,
) -> Eno2dEval
where
    F: FnMut(usize) -> &'a [Value],
{
    let y_window_start = eno_window_start(y_index, order, row_count);
    let y_window = 2 * order - 2;
    let y_local_index = y_index - y_window_start;

    values_along_y.resize(y_window, 0.0);
    dx_along_y.resize(y_window, 0.0);
    for (slot, row_index) in (y_window_start..y_window_start + y_window).enumerate() {
        let row = row_at(row_index);
        let eval = eno1d_eval_with_scratch(row, order, x_index, x_offset, x_scratch);
        values_along_y[slot] = eval.value;
        dx_along_y[slot] = eval.derivative;
    }

    let y_eval = eno1d_eval_with_scratch(values_along_y, order, y_local_index, y_offset, y_scratch);
    let dx_eval = eno1d_eval_with_scratch(dx_along_y, order, y_local_index, y_offset, y_scratch);
    Eno2dEval {
        value: y_eval.value,
        dx: dx_eval.value,
        dy: y_eval.derivative,
    }
}

fn table2d_eno_derivatives(
    table: &Table2DData,
    order: usize,
    x_axis: AxisEval,
    y_axis: AxisEval,
) -> Eno2dEval {
    let mut scratch = TableEvalScratch::default();
    table2d_eno_derivatives_with_scratch(table, order, x_axis, y_axis, &mut scratch)
}

fn table2d_eno_derivatives_with_scratch(
    table: &Table2DData,
    order: usize,
    x_axis: AxisEval,
    y_axis: AxisEval,
    scratch: &mut TableEvalScratch,
) -> Eno2dEval {
    eno2d_eval_grid_with_scratch(
        table.y.len(),
        &mut |y| table2d_row(table, y),
        order,
        x_axis.eno_index,
        y_axis.eno_index,
        x_axis.eno_offset,
        y_axis.eno_offset,
        &mut scratch.values_along_y,
        &mut scratch.dx_along_y,
        &mut scratch.x_scratch,
        &mut scratch.y_scratch,
    )
}

fn table3d_eno_derivatives(
    table: &Table3DData,
    order: usize,
    x_axis: AxisEval,
    y_axis: AxisEval,
    z_axis: AxisEval,
) -> Eno3dEval {
    let mut scratch = TableEvalScratch::default();
    table3d_eno_derivatives_with_scratch(table, order, x_axis, y_axis, z_axis, &mut scratch)
}

fn table3d_eno_derivatives_with_scratch(
    table: &Table3DData,
    order: usize,
    x_axis: AxisEval,
    y_axis: AxisEval,
    z_axis: AxisEval,
    scratch: &mut TableEvalScratch,
) -> Eno3dEval {
    let yz_window = 2 * order - 2;
    let y_window_start = eno_window_start(y_axis.eno_index, order, table.y.len());
    let z_window_start = eno_window_start(z_axis.eno_index, order, table.z.len());
    let y_local_index = y_axis.eno_index - y_window_start;
    let z_local_index = z_axis.eno_index - z_window_start;

    let yz_len = yz_window * yz_window;
    scratch.values_over_zy.resize(yz_len, 0.0);
    scratch.dx_over_zy.resize(yz_len, 0.0);
    for (z_slot, z) in (z_window_start..z_window_start + yz_window).enumerate() {
        for (y_slot, y) in (y_window_start..y_window_start + yz_window).enumerate() {
            let eval = eno1d_eval_with_scratch(
                table3d_x_row(table, y, z),
                order,
                x_axis.eno_index,
                x_axis.eno_offset,
                &mut scratch.x_scratch,
            );
            let slot = z_slot * yz_window + y_slot;
            scratch.values_over_zy[slot] = eval.value;
            scratch.dx_over_zy[slot] = eval.derivative;
        }
    }

    let yz_eval = eno2d_eval_grid_with_scratch(
        yz_window,
        &mut |row| {
            let start = row * yz_window;
            &scratch.values_over_zy[start..start + yz_window]
        },
        order,
        y_local_index,
        z_local_index,
        y_axis.eno_offset,
        z_axis.eno_offset,
        &mut scratch.values_along_y,
        &mut scratch.dx_along_y,
        &mut scratch.yz_x_scratch,
        &mut scratch.yz_y_scratch,
    );
    let dx_eval = eno2d_eval_grid_with_scratch(
        yz_window,
        &mut |row| {
            let start = row * yz_window;
            &scratch.dx_over_zy[start..start + yz_window]
        },
        order,
        y_local_index,
        z_local_index,
        y_axis.eno_offset,
        z_axis.eno_offset,
        &mut scratch.values_along_y,
        &mut scratch.dx_along_y,
        &mut scratch.yz_x_scratch,
        &mut scratch.yz_y_scratch,
    );

    Eno3dEval {
        dx: dx_eval.value,
        dy: yz_eval.dx,
        dz: yz_eval.dy,
    }
}

fn evaluate_table2d_data(table: &Table2DData, order: usize, x: Value, y: Value) -> TableEval2D {
    let mut scratch = TableEvalScratch::default();
    evaluate_table2d_data_with_scratch(table, order, x, y, &mut scratch)
}

fn evaluate_table2d_data_with_scratch(
    table: &Table2DData,
    order: usize,
    x: Value,
    y: Value,
    scratch: &mut TableEvalScratch,
) -> TableEval2D {
    let x_axis = axis_eval(&table.x, x);
    let y_axis = axis_eval(&table.y, y);
    let x0 = x_axis.lower;
    let x1 = x_axis.upper;
    let y0 = y_axis.lower;
    let y1 = y_axis.upper;
    let tx = x_axis.t;
    let ty = y_axis.t;

    let v00 = table2d_value(table, x0, y0);
    let v10 = table2d_value(table, x1, y0);
    let v01 = table2d_value(table, x0, y1);
    let v11 = table2d_value(table, x1, y1);

    let one_minus_tx = 1.0 - tx;
    let one_minus_ty = 1.0 - ty;
    let value = one_minus_tx * one_minus_ty * v00
        + tx * one_minus_ty * v10
        + one_minus_tx * ty * v01
        + tx * ty * v11;

    let derivatives = table2d_eno_derivatives_with_scratch(table, order, x_axis, y_axis, scratch);
    let dx = derivatives.dx / x_axis.local_diff * x_axis.derivative_scale;
    let dy = derivatives.dy / y_axis.local_diff * y_axis.derivative_scale;

    TableEval2D { value, dx, dy }
}

fn evaluate_table3d_data(
    table: &Table3DData,
    order: usize,
    x: Value,
    y: Value,
    z: Value,
) -> TableEval3D {
    let mut scratch = TableEvalScratch::default();
    evaluate_table3d_data_with_scratch(table, order, x, y, z, &mut scratch)
}

fn evaluate_table3d_data_with_scratch(
    table: &Table3DData,
    order: usize,
    x: Value,
    y: Value,
    z: Value,
    scratch: &mut TableEvalScratch,
) -> TableEval3D {
    let x_axis = axis_eval(&table.x, x);
    let y_axis = axis_eval(&table.y, y);
    let z_axis = axis_eval(&table.z, z);
    let x_indices = [x_axis.lower, x_axis.upper];
    let y_indices = [y_axis.lower, y_axis.upper];
    let z_indices = [z_axis.lower, z_axis.upper];
    let wx = [1.0 - x_axis.t, x_axis.t];
    let wy = [1.0 - y_axis.t, y_axis.t];
    let wz = [1.0 - z_axis.t, z_axis.t];

    let mut corners = [[[0.0; 2]; 2]; 2];
    for zi in 0..2 {
        for yi in 0..2 {
            for xi in 0..2 {
                corners[zi][yi][xi] =
                    table3d_value(table, x_indices[xi], y_indices[yi], z_indices[zi]);
            }
        }
    }

    let mut value = 0.0;
    for zi in 0..2 {
        for yi in 0..2 {
            for xi in 0..2 {
                value += wx[xi] * wy[yi] * wz[zi] * corners[zi][yi][xi];
            }
        }
    }

    let derivatives =
        table3d_eno_derivatives_with_scratch(table, order, x_axis, y_axis, z_axis, scratch);

    TableEval3D {
        value,
        dx: derivatives.dx / x_axis.local_diff * x_axis.derivative_scale,
        dy: derivatives.dy / y_axis.local_diff * y_axis.derivative_scale,
        dz: derivatives.dz / z_axis.local_diff * z_axis.derivative_scale,
    }
}

fn table_file_param<'a>(ctx: &'a CmContext, kind: TableKind) -> &'a str {
    ctx.string_param("file")
        .filter(|path| !path.trim().is_empty())
        .unwrap_or_else(|| kind.default_file())
}

fn table_arc_id<T>(table: &Arc<T>) -> usize {
    Arc::as_ptr(table) as usize
}

fn table2d_eval_signature(
    ctx: &CmContext,
    table: &Arc<Table2DData>,
    order: usize,
    inx: Value,
    iny: Value,
) -> Table2DEvalSignature {
    Table2DEvalSignature {
        table_id: table_arc_id(table),
        order,
        offset: ctx.param("offset"),
        gain: ctx.param("gain"),
        inx,
        iny,
    }
}

fn table3d_eval_signature(
    ctx: &CmContext,
    table: &Arc<Table3DData>,
    order: usize,
    inx: Value,
    iny: Value,
    inz: Value,
) -> Table3DEvalSignature {
    Table3DEvalSignature {
        table_id: table_arc_id(table),
        order,
        offset: ctx.param("offset"),
        gain: ctx.param("gain"),
        inx,
        iny,
        inz,
    }
}

fn scaled_table2d_eval_from_parts(
    ctx: &CmContext,
    table: &Table2DData,
    order: usize,
    inx: Value,
    iny: Value,
) -> TableEval2D {
    let raw = with_table_eval_scratch(ctx, |scratch| {
        evaluate_table2d_data_with_scratch(table, order, inx, iny, scratch)
    })
    .unwrap_or_else(|| evaluate_table2d_data(table, order, inx, iny));
    let gain = ctx.param("gain");
    TableEval2D {
        value: ctx.param("offset") + gain * raw.value,
        dx: gain * raw.dx,
        dy: gain * raw.dy,
    }
}

fn scaled_table2d_eval(ctx: &CmContext) -> CmResult<TableEval2D> {
    let file = table_file_param(ctx, TableKind::Table2D);
    let table = load_table2d_from_context(ctx, file)?;
    let order = table_order(ctx)?;
    validate_table2d_order(&table, order)?;
    let inx = ctx.input("inx");
    let iny = ctx.input("iny");
    let signature = table2d_eval_signature(ctx, &table, order, inx, iny);
    if let Some(resource) = ctx.resource::<Table2DEvalResource>(TABLE2D_EVAL_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.result);
    }
    Ok(scaled_table2d_eval_from_parts(
        ctx,
        table.as_ref(),
        order,
        inx,
        iny,
    ))
}

fn scaled_table2d_eval_cached(ctx: &mut CmContext) -> CmResult<TableEval2D> {
    let file = table_file_param(ctx, TableKind::Table2D).to_string();
    let table = load_table2d_for_context(ctx, &file)?;
    let order = table_order(ctx)?;
    validate_table2d_order(&table, order)?;
    let inx = ctx.input("inx");
    let iny = ctx.input("iny");
    let signature = table2d_eval_signature(ctx, &table, order, inx, iny);
    if let Some(resource) = ctx.resource::<Table2DEvalResource>(TABLE2D_EVAL_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.result);
    }
    let result = scaled_table2d_eval_from_parts(ctx, table.as_ref(), order, inx, iny);
    ctx.set_resource(
        TABLE2D_EVAL_RESOURCE,
        Arc::new(Table2DEvalResource { signature, result }),
    );
    Ok(result)
}

fn scaled_table3d_eval_from_parts(
    ctx: &CmContext,
    table: &Table3DData,
    order: usize,
    inx: Value,
    iny: Value,
    inz: Value,
) -> TableEval3D {
    let raw = with_table_eval_scratch(ctx, |scratch| {
        evaluate_table3d_data_with_scratch(table, order, inx, iny, inz, scratch)
    })
    .unwrap_or_else(|| evaluate_table3d_data(table, order, inx, iny, inz));
    let gain = ctx.param("gain");
    TableEval3D {
        value: ctx.param("offset") + gain * raw.value,
        dx: gain * raw.dx,
        dy: gain * raw.dy,
        dz: gain * raw.dz,
    }
}

fn scaled_table3d_eval(ctx: &CmContext) -> CmResult<TableEval3D> {
    let file = table_file_param(ctx, TableKind::Table3D);
    let table = load_table3d_from_context(ctx, file)?;
    let order = table_order(ctx)?;
    validate_table3d_order(&table, order)?;
    let inx = ctx.input("inx");
    let iny = ctx.input("iny");
    let inz = ctx.input("inz");
    let signature = table3d_eval_signature(ctx, &table, order, inx, iny, inz);
    if let Some(resource) = ctx.resource::<Table3DEvalResource>(TABLE3D_EVAL_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.result);
    }
    Ok(scaled_table3d_eval_from_parts(
        ctx,
        table.as_ref(),
        order,
        inx,
        iny,
        inz,
    ))
}

fn scaled_table3d_eval_cached(ctx: &mut CmContext) -> CmResult<TableEval3D> {
    let file = table_file_param(ctx, TableKind::Table3D).to_string();
    let table = load_table3d_for_context(ctx, &file)?;
    let order = table_order(ctx)?;
    validate_table3d_order(&table, order)?;
    let inx = ctx.input("inx");
    let iny = ctx.input("iny");
    let inz = ctx.input("inz");
    let signature = table3d_eval_signature(ctx, &table, order, inx, iny, inz);
    if let Some(resource) = ctx.resource::<Table3DEvalResource>(TABLE3D_EVAL_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.result);
    }
    let result = scaled_table3d_eval_from_parts(ctx, table.as_ref(), order, inx, iny, inz);
    ctx.set_resource(
        TABLE3D_EVAL_RESOURCE,
        Arc::new(Table3DEvalResource { signature, result }),
    );
    Ok(result)
}

fn table_input_port(name: &str) -> PortSpec {
    PortSpec {
        name: name.to_string(),
        direction: PortDirection::In,
        default_type: PortType::Voltage,
        allowed_types: vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
            PortType::VoltageName,
        ],
        is_vector: false,
        null_allowed: false,
        vector_min_len: None,
        vector_max_len: None,
        description: "Table input".to_string(),
    }
}

fn table_output_port() -> PortSpec {
    PortSpec {
        name: "out".to_string(),
        direction: PortDirection::Out,
        default_type: PortType::Current,
        allowed_types: vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
        ],
        is_vector: false,
        null_allowed: false,
        vector_min_len: None,
        vector_max_len: None,
        description: "Table output".to_string(),
    }
}

fn table_params(kind: TableKind) -> &'static [ParamSpec] {
    static TABLE2D_PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    static TABLE3D_PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    let params = || {
        vec![
            ParamSpec::integer("order", 3)
                .with_description("Interpolation order, clamped to the official minimum"),
            ParamSpec::integer("verbose", 0).with_description(
                "Verbose level, accepted outside the official range like ngspice",
            ),
            ParamSpec::real("offset", 0.0),
            ParamSpec::real("gain", 1.0),
            ParamSpec::string("file", kind.default_file()),
        ]
    };
    match kind {
        TableKind::Table2D => TABLE2D_PARAMS.get_or_init(params),
        TableKind::Table3D => TABLE3D_PARAMS.get_or_init(params),
    }
}

impl CodeModel for Table2D {
    fn name(&self) -> &str {
        "table2d"
    }

    fn description(&self) -> &str {
        "File-backed two-dimensional XSPICE lookup table"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                table_input_port("inx"),
                table_input_port("iny"),
                table_output_port(),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        table_params(TableKind::Table2D)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let order = table_order(ctx)?;
        let file = table_file_param(ctx, TableKind::Table2D).to_string();
        let table = load_table2d_for_context(ctx, &file)?;
        validate_table2d_order(&table, order)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = scaled_table2d_eval_cached(ctx)?;
        ctx.set_output_with_partial("out", eval.value, 0.0);
        Ok(())
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        match scaled_table2d_eval(ctx) {
            Ok(eval) => vec![eval.dx, eval.dy],
            Err(_) => vec![0.0, 0.0],
        }
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        match scaled_table2d_eval(ctx) {
            Ok(eval) => vec![("inx".to_string(), eval.dx), ("iny".to_string(), eval.dy)],
            Err(_) => Vec::new(),
        }
    }
}

impl CodeModel for Table3D {
    fn name(&self) -> &str {
        "table3d"
    }

    fn description(&self) -> &str {
        "File-backed three-dimensional XSPICE lookup table"
    }

    fn ports(&self) -> &[PortSpec] {
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                table_input_port("inx"),
                table_input_port("iny"),
                table_input_port("inz"),
                table_output_port(),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        table_params(TableKind::Table3D)
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let order = table_order(ctx)?;
        let file = table_file_param(ctx, TableKind::Table3D).to_string();
        let table = load_table3d_for_context(ctx, &file)?;
        validate_table3d_order(&table, order)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let eval = scaled_table3d_eval_cached(ctx)?;
        ctx.set_output_with_partial("out", eval.value, 0.0);
        Ok(())
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        match scaled_table3d_eval(ctx) {
            Ok(eval) => vec![eval.dx, eval.dy, eval.dz],
            Err(_) => vec![0.0, 0.0, 0.0],
        }
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        match scaled_table3d_eval(ctx) {
            Ok(eval) => vec![
                ("inx".to_string(), eval.dx),
                ("iny".to_string(), eval.dy),
                ("inz".to_string(), eval.dz),
            ],
            Err(_) => Vec::new(),
        }
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        _frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        match scaled_table3d_eval(ctx) {
            Ok(eval) => vec![
                ("inx".to_string(), Complex64::new(eval.dx, 0.0)),
                ("iny".to_string(), Complex64::new(eval.dy, 0.0)),
                ("inz".to_string(), Complex64::new(eval.dz, 0.0)),
            ],
            Err(_) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn data_file_test_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: std::sync::OnceLock<std::sync::Mutex<()>> = std::sync::OnceLock::new();
        GUARD
            .get_or_init(|| std::sync::Mutex::new(()))
            .lock()
            .expect("data-file test guard")
    }

    fn poison_table2d_cache_lock() {
        let result = std::panic::catch_unwind(|| {
            let _guard = lock_table2d_cache();
            panic!("poison table2d cache lock for recovery test");
        });
        assert!(result.is_err(), "recovery test must poison the mutex");
    }

    fn poison_table3d_cache_lock() {
        let result = std::panic::catch_unwind(|| {
            let _guard = lock_table3d_cache();
            panic!("poison table3d cache lock for recovery test");
        });
        assert!(result.is_err(), "recovery test must poison the mutex");
    }

    #[test]
    fn table_cache_locks_recover_after_poison() {
        poison_table2d_cache_lock();
        lock_table2d_cache().clear();

        poison_table3d_cache_lock();
        lock_table3d_cache().clear();
    }

    #[test]
    fn tokenize_table_contents_borrows_input_tokens() {
        let file = "inline-table";
        let model = TableKind::Table2D.model_name();
        let contents = "* header\n2\n2\n0 1\n0 1\n3 4\n";
        let lines = tokenize_table_contents(model, file, contents).expect("tokenized table");
        let token = lines[3].tokens[1];
        let contents_start = contents.as_ptr() as usize;
        let contents_end = contents_start + contents.len();
        let token_start = token.as_ptr() as usize;
        let token_end = token_start + token.len();

        assert_eq!(token, "1");
        assert!(token_start >= contents_start);
        assert!(token_end <= contents_end);
    }

    #[test]
    fn token_cursor_returns_borrowed_table_lines() {
        let file = "inline-table";
        let model = TableKind::Table2D.model_name();
        let lines = tokenize_table_contents(model, file, "* header\n2\n2\n0 1\n0 1\n3 4\n")
            .expect("tokenized table");
        let x_tokens = lines[3].tokens.as_ptr();
        let data_tokens = lines[5].tokens.as_ptr();
        let mut cursor = TokenCursor::new(lines);

        assert_eq!(
            cursor.next_dimension(file, model, "x dimension").unwrap(),
            2
        );
        assert_eq!(
            cursor.next_dimension(file, model, "y dimension").unwrap(),
            2
        );
        {
            let x = cursor.next_line(file, model, "x").unwrap();
            assert_eq!(x.tokens.as_ptr(), x_tokens);
        }
        {
            let _y = cursor.next_line(file, model, "y").unwrap();
        }
        let row = cursor.next_data_line_optional().unwrap();

        assert_eq!(row.tokens.as_ptr(), data_tokens);
    }

    #[test]
    fn table3d_missing_value_row_reports_coordinates() {
        let err = parse_table3d_contents(
            "inline-table3d",
            "\
2
2
2
0 1
0 1
0 1
10 11
12 13
20 21
",
        )
        .expect_err("short table3d data must report the missing row");
        let message = err.to_string();

        assert!(
            message.contains("missing table 1 row 1; table file ended early"),
            "missing table3d row error should include z/y coordinates, got {message}"
        );
    }

    #[test]
    fn eno1d_eval_reconstructs_quadratic_from_local_window() {
        let samples: Vec<Value> = (0..64)
            .map(|index| {
                let x = index as Value;
                0.5 * x * x + 2.0 * x + 1.0
            })
            .collect();

        let eval = eno1d_eval(&samples, 3, 31, 0.25);
        let x = 31.25;
        let expected_value = 0.5 * x * x + 2.0 * x + 1.0;
        let expected_derivative = x + 2.0;

        assert!((eval.value - expected_value).abs() < 1.0e-12);
        assert!((eval.derivative - expected_derivative).abs() < 1.0e-12);
    }

    #[test]
    fn eno1d_reused_scratch_matches_allocating_wrapper() {
        let samples: Vec<Value> = (0..64)
            .map(|index| {
                let x = index as Value;
                x.sin() + 0.125 * x * x
            })
            .collect();
        let mut scratch = Vec::new();

        let fast = eno1d_eval_with_scratch(&samples, 4, 31, 0.4, &mut scratch);
        let expected = eno1d_eval(&samples, 4, 31, 0.4);
        assert_eq!(fast.value, expected.value);
        assert_eq!(fast.derivative, expected.derivative);

        let capacity = scratch.capacity();
        let again = eno1d_eval_with_scratch(&samples, 4, 32, 0.1, &mut scratch);
        let expected_again = eno1d_eval(&samples, 4, 32, 0.1);
        assert_eq!(scratch.capacity(), capacity);
        assert_eq!(again.value, expected_again.value);
        assert_eq!(again.derivative, expected_again.derivative);
    }

    #[test]
    fn eno1d_eval_overwrites_dirty_reused_scratch() {
        let samples: Vec<Value> = (0..16)
            .map(|index| {
                let x = index as Value;
                x * x + 0.5 * x
            })
            .collect();
        let mut scratch = vec![Value::NAN; 128];

        let fast = eno1d_eval_with_scratch(&samples, 4, 7, 0.25, &mut scratch);
        let expected = eno1d_eval(&samples, 4, 7, 0.25);

        assert!(fast.value.is_finite());
        assert!(fast.derivative.is_finite());
        assert_eq!(fast.value, expected.value);
        assert_eq!(fast.derivative, expected.derivative);
    }

    #[test]
    fn table_axis_precomputes_evaluation_metadata() {
        let axis = TableAxis::new(vec![0.0, 1.0, 3.0, 6.0]);

        assert_eq!(axis.local_diffs, vec![1.0, 1.5, 2.5, 3.0]);
        assert_eq!(axis.inverse_spans, vec![1.0, 0.5, 1.0 / 3.0]);
        assert_eq!(
            axis.last_eno_index.load(Ordering::Relaxed),
            AXIS_UNSET_ENO_INDEX
        );

        let middle = axis_eval(&axis, 2.0);
        assert_eq!(middle.lower, 1);
        assert_eq!(middle.upper, 2);
        assert_eq!(middle.t, 0.5);
        assert_eq!(middle.local_diff, 1.5);
        assert_eq!(middle.derivative_scale, 1.0);
        assert_eq!(axis.last_eno_index.load(Ordering::Relaxed), 1);

        let exact = axis_eval(&axis, 3.0);
        assert_eq!(exact.lower, 2);
        assert_eq!(exact.upper, 3);
        assert_eq!(exact.t, 0.0);
        assert_eq!(
            exact.eno_index, 2,
            "exact interior knots must keep the original partition-point semantics"
        );
        assert_eq!(axis.last_eno_index.load(Ordering::Relaxed), 2);

        let near_upper = axis_eval(&axis, 6.25);
        assert_eq!(near_upper.lower, 2);
        assert_eq!(near_upper.upper, 3);
        assert_eq!(near_upper.t, 1.0);
        assert!((near_upper.derivative_scale - (1.0 / 3.0)).abs() < 1.0e-12);
        assert_eq!(axis.last_eno_index.load(Ordering::Relaxed), 3);
    }

    #[test]
    fn table_axis_cursor_falls_back_for_large_input_jumps() {
        let axis = TableAxis::new((1..=24).map(|value| value as Value).collect());

        let low = axis_eval(&axis, 2.5);
        assert_eq!(low.eno_index, 1);
        assert_eq!(axis.last_eno_index.load(Ordering::Relaxed), 1);

        let high = axis_eval(&axis, 22.5);
        assert_eq!(high.eno_index, 21);
        assert_eq!(
            axis.last_eno_index.load(Ordering::Relaxed),
            21,
            "large non-local axis jumps should land on the binary-search bracket"
        );
    }

    #[test]
    fn table2d_eval_reused_scratch_matches_allocating_wrapper() {
        let axis = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let mut values = Vec::new();
        for &y in &axis {
            for &x in &axis {
                values.push(x * x + 0.5 * y * y + x * y);
            }
        }
        let table = Table2DData {
            x: TableAxis::new(axis.clone()),
            y: TableAxis::new(axis),
            values,
        };
        let mut scratch = TableEvalScratch::default();

        let fast = evaluate_table2d_data_with_scratch(&table, 3, 1.4, 2.2, &mut scratch);
        let expected = evaluate_table2d_data(&table, 3, 1.4, 2.2);
        assert_eq!(fast.value, expected.value);
        assert_eq!(fast.dx, expected.dx);
        assert_eq!(fast.dy, expected.dy);

        let capacities = (
            scratch.values_along_y.capacity(),
            scratch.dx_along_y.capacity(),
            scratch.x_scratch.capacity(),
            scratch.y_scratch.capacity(),
        );
        let again = evaluate_table2d_data_with_scratch(&table, 3, 2.1, 1.6, &mut scratch);
        let expected_again = evaluate_table2d_data(&table, 3, 2.1, 1.6);
        assert_eq!(
            (
                scratch.values_along_y.capacity(),
                scratch.dx_along_y.capacity(),
                scratch.x_scratch.capacity(),
                scratch.y_scratch.capacity(),
            ),
            capacities
        );
        assert_eq!(again.value, expected_again.value);
        assert_eq!(again.dx, expected_again.dx);
        assert_eq!(again.dy, expected_again.dy);
    }

    #[test]
    fn table2d_eval_overwrites_dirty_reused_scratch() {
        let axis = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let mut values = Vec::new();
        for &y in &axis {
            for &x in &axis {
                values.push(x * x + y * y + x * y);
            }
        }
        let table = Table2DData {
            x: TableAxis::new(axis.clone()),
            y: TableAxis::new(axis),
            values,
        };
        let mut scratch = TableEvalScratch {
            values_along_y: vec![Value::NAN; 32],
            dx_along_y: vec![Value::NAN; 32],
            x_scratch: vec![Value::NAN; 64],
            y_scratch: vec![Value::NAN; 64],
            ..TableEvalScratch::default()
        };

        let fast = evaluate_table2d_data_with_scratch(&table, 3, 1.4, 2.2, &mut scratch);
        let expected = evaluate_table2d_data(&table, 3, 1.4, 2.2);

        assert!(fast.value.is_finite());
        assert!(fast.dx.is_finite());
        assert!(fast.dy.is_finite());
        assert_eq!(fast.value, expected.value);
        assert_eq!(fast.dx, expected.dx);
        assert_eq!(fast.dy, expected.dy);
    }

    #[test]
    fn table3d_eval_reused_scratch_matches_allocating_wrapper() {
        let axis = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let mut values = Vec::new();
        for &z in &axis {
            for &y in &axis {
                for &x in &axis {
                    values.push(x * x + 0.5 * y * y + 0.25 * z * z + x * y + y * z);
                }
            }
        }
        let table = Table3DData {
            x: TableAxis::new(axis.clone()),
            y: TableAxis::new(axis.clone()),
            z: TableAxis::new(axis),
            values,
        };
        let mut scratch = TableEvalScratch::default();

        let fast = evaluate_table3d_data_with_scratch(&table, 3, 1.4, 2.2, 0.7, &mut scratch);
        let expected = evaluate_table3d_data(&table, 3, 1.4, 2.2, 0.7);
        assert_eq!(fast.value, expected.value);
        assert_eq!(fast.dx, expected.dx);
        assert_eq!(fast.dy, expected.dy);
        assert_eq!(fast.dz, expected.dz);

        let capacities = (
            scratch.values_over_zy.capacity(),
            scratch.dx_over_zy.capacity(),
            scratch.values_along_y.capacity(),
            scratch.dx_along_y.capacity(),
            scratch.x_scratch.capacity(),
            scratch.yz_x_scratch.capacity(),
            scratch.yz_y_scratch.capacity(),
        );
        let again = evaluate_table3d_data_with_scratch(&table, 3, 2.1, 1.6, 3.2, &mut scratch);
        let expected_again = evaluate_table3d_data(&table, 3, 2.1, 1.6, 3.2);
        assert_eq!(
            (
                scratch.values_over_zy.capacity(),
                scratch.dx_over_zy.capacity(),
                scratch.values_along_y.capacity(),
                scratch.dx_along_y.capacity(),
                scratch.x_scratch.capacity(),
                scratch.yz_x_scratch.capacity(),
                scratch.yz_y_scratch.capacity(),
            ),
            capacities
        );
        assert_eq!(again.value, expected_again.value);
        assert_eq!(again.dx, expected_again.dx);
        assert_eq!(again.dy, expected_again.dy);
        assert_eq!(again.dz, expected_again.dz);
    }

    #[test]
    fn table3d_eval_overwrites_dirty_reused_scratch() {
        let axis = vec![0.0, 1.0, 2.0, 3.0, 4.0];
        let mut values = Vec::new();
        for &z in &axis {
            for &y in &axis {
                for &x in &axis {
                    values.push(x * x + y * y + z * z + x * y + y * z);
                }
            }
        }
        let table = Table3DData {
            x: TableAxis::new(axis.clone()),
            y: TableAxis::new(axis.clone()),
            z: TableAxis::new(axis),
            values,
        };
        let mut scratch = TableEvalScratch {
            values_along_y: vec![Value::NAN; 32],
            dx_along_y: vec![Value::NAN; 32],
            x_scratch: vec![Value::NAN; 64],
            y_scratch: vec![Value::NAN; 64],
            values_over_zy: vec![Value::NAN; 64],
            dx_over_zy: vec![Value::NAN; 64],
            yz_x_scratch: vec![Value::NAN; 64],
            yz_y_scratch: vec![Value::NAN; 64],
        };

        let fast = evaluate_table3d_data_with_scratch(&table, 3, 1.4, 2.2, 0.7, &mut scratch);
        let expected = evaluate_table3d_data(&table, 3, 1.4, 2.2, 0.7);

        assert!(fast.value.is_finite());
        assert!(fast.dx.is_finite());
        assert!(fast.dy.is_finite());
        assert!(fast.dz.is_finite());
        assert_eq!(fast.value, expected.value);
        assert_eq!(fast.dx, expected.dx);
        assert_eq!(fast.dy, expected.dy);
        assert_eq!(fast.dz, expected.dz);
    }

    #[test]
    fn table2d_eval_cache_reuses_current_result_until_signature_changes() {
        let _guard = data_file_test_guard();
        let file = "virtual://table2d/eval-cache";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(
            file,
            "\
2
2
0 1
0 1
1 2
3 4
",
        )
        .expect("register virtual table2d data");

        let mut ctx = CmContext::new();
        ctx.set_string_param("file", file);
        ctx.set_param("order", 2.0);
        ctx.set_param("offset", 0.0);
        ctx.set_param("gain", 1.0);
        ctx.set_input_analog("inx", 0.25);
        ctx.set_input_analog("iny", 0.5);
        ctx.init_output("out", PortType::Current);

        Table2D.init(&mut ctx).expect("table2d init");
        let initial = scaled_table2d_eval_cached(&mut ctx).expect("table2d eval caches");
        assert!(initial.value.is_finite());

        let cached = ctx
            .resource::<Table2DEvalResource>(TABLE2D_EVAL_RESOURCE)
            .expect("table2d cached eval")
            .clone();
        let sentinel = TableEval2D {
            value: 123.0,
            dx: 7.0,
            dy: 11.0,
        };
        ctx.set_resource(
            TABLE2D_EVAL_RESOURCE,
            Arc::new(Table2DEvalResource {
                signature: cached.signature.clone(),
                result: sentinel,
            }),
        );

        assert_eq!(
            scaled_table2d_eval(&ctx).expect("read-only table2d eval reuses cache"),
            sentinel
        );
        assert_eq!(Table2D.ac_gain(&ctx), vec![7.0, 11.0]);
        assert_eq!(
            Table2D.output_input_partials(&ctx, "out"),
            vec![("inx".to_string(), 7.0), ("iny".to_string(), 11.0)]
        );

        ctx.set_input_analog("inx", 0.75);
        let changed_input = scaled_table2d_eval(&ctx).expect("changed input invalidates cache");
        assert_ne!(changed_input, sentinel);

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn table3d_eval_cache_reuses_current_result_until_signature_changes() {
        let _guard = data_file_test_guard();
        let file = "virtual://table3d/eval-cache";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(
            file,
            "\
2
2
2
0 1
0 1
0 1
1 2
3 4
5 6
7 8
",
        )
        .expect("register virtual table3d data");

        let mut ctx = CmContext::new();
        ctx.set_string_param("file", file);
        ctx.set_param("order", 2.0);
        ctx.set_param("offset", 0.0);
        ctx.set_param("gain", 1.0);
        ctx.set_input_analog("inx", 0.25);
        ctx.set_input_analog("iny", 0.5);
        ctx.set_input_analog("inz", 0.75);
        ctx.init_output("out", PortType::Current);

        Table3D.init(&mut ctx).expect("table3d init");
        let initial = scaled_table3d_eval_cached(&mut ctx).expect("table3d eval caches");
        assert!(initial.value.is_finite());

        let cached = ctx
            .resource::<Table3DEvalResource>(TABLE3D_EVAL_RESOURCE)
            .expect("table3d cached eval")
            .clone();
        let sentinel = TableEval3D {
            value: 456.0,
            dx: 13.0,
            dy: 17.0,
            dz: 19.0,
        };
        ctx.set_resource(
            TABLE3D_EVAL_RESOURCE,
            Arc::new(Table3DEvalResource {
                signature: cached.signature.clone(),
                result: sentinel,
            }),
        );

        assert_eq!(
            scaled_table3d_eval(&ctx).expect("read-only table3d eval reuses cache"),
            sentinel
        );
        assert_eq!(Table3D.ac_gain(&ctx), vec![13.0, 17.0, 19.0]);
        assert_eq!(
            Table3D.output_input_partials(&ctx, "out"),
            vec![
                ("inx".to_string(), 13.0),
                ("iny".to_string(), 17.0),
                ("inz".to_string(), 19.0)
            ]
        );
        assert_eq!(
            Table3D.output_input_ac_partials(&ctx, "out", 1.0e3),
            vec![
                ("inx".to_string(), Complex64::new(13.0, 0.0)),
                ("iny".to_string(), Complex64::new(17.0, 0.0)),
                ("inz".to_string(), Complex64::new(19.0, 0.0))
            ]
        );

        ctx.set_param("gain", 2.0);
        let changed_gain = scaled_table3d_eval(&ctx).expect("changed gain invalidates cache");
        assert_ne!(changed_gain, sentinel);

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn table2d_context_resource_reloads_when_virtual_file_changes() {
        let _guard = data_file_test_guard();
        let file = "virtual://table2d/context-resource-reload";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(
            file,
            "\
2
2
0 1
0 1
1 1
1 1
",
        )
        .expect("register first virtual table2d data");

        let mut ctx = CmContext::new();
        ctx.set_string_param("file", file);
        ctx.set_param("order", 2.0);
        ctx.set_param("offset", 0.0);
        ctx.set_param("gain", 1.0);
        ctx.set_input_analog("inx", 0.5);
        ctx.set_input_analog("iny", 0.5);

        Table2D.init(&mut ctx).expect("table2d init");
        Table2D.evaluate(&mut ctx).expect("table2d evaluate");
        assert!((ctx.output("out") - 1.0).abs() < 1.0e-12);

        data_file::register_data_file(
            file,
            "\
2
2
0 1
0 1
5 5
5 5
",
        )
        .expect("replace virtual table2d data");
        Table2D
            .evaluate(&mut ctx)
            .expect("table2d reevaluates after virtual replace");
        assert!((ctx.output("out") - 5.0).abs() < 1.0e-12);

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn table_caches_retire_replaced_virtual_file_entries() {
        let _guard = data_file_test_guard();
        let table2d_file = "virtual://table2d/cache-retention";
        let table3d_file = "virtual://table3d/cache-retention";
        let _ = data_file::unregister_data_file(table2d_file);
        let _ = data_file::unregister_data_file(table3d_file);
        lock_table2d_cache().clear();
        lock_table3d_cache().clear();

        data_file::register_data_file(
            table2d_file,
            "\
2
2
0 1
0 1
1 1
1 1
",
        )
        .expect("register first virtual table2d data");
        let (first2d, _) = load_table2d(table2d_file).expect("load first virtual table2d data");
        assert_eq!(first2d.values, vec![1.0, 1.0, 1.0, 1.0]);

        data_file::register_data_file(
            table2d_file,
            "\
2
2
0 1
0 1
5 5
5 5
",
        )
        .expect("replace virtual table2d data");
        let (second2d, _) = load_table2d(table2d_file).expect("load replaced virtual table2d data");
        assert_eq!(second2d.values, vec![5.0, 5.0, 5.0, 5.0]);

        data_file::register_data_file(
            table3d_file,
            "\
2
2
2
0 1
0 1
0 1
1 1
1 1
1 1
1 1
",
        )
        .expect("register first virtual table3d data");
        let (first3d, _) = load_table3d(table3d_file).expect("load first virtual table3d data");
        assert_eq!(first3d.values, vec![1.0; 8]);

        data_file::register_data_file(
            table3d_file,
            "\
2
2
2
0 1
0 1
0 1
3 3
3 3
3 3
3 3
",
        )
        .expect("replace virtual table3d data");
        let (second3d, _) = load_table3d(table3d_file).expect("load replaced virtual table3d data");
        assert_eq!(second3d.values, vec![3.0; 8]);

        let cached_table2d_entries = {
            let guard = lock_table2d_cache();
            guard
                .entries
                .keys()
                .filter(|key| key.file == table2d_file)
                .count()
        };
        assert_eq!(cached_table2d_entries, 1);

        let cached_table3d_entries = {
            let guard = lock_table3d_cache();
            guard
                .entries
                .keys()
                .filter(|key| key.file == table3d_file)
                .count()
        };
        assert_eq!(cached_table3d_entries, 1);

        let _ = data_file::unregister_data_file(table2d_file);
        let _ = data_file::unregister_data_file(table3d_file);
    }
}

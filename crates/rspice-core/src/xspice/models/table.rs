//! Official file-backed XSPICE `table2d` and `table3d` code models.

use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
    data_file,
};
use crate::{Complex64, Value};
use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, OnceLock};

const BOUNDARY_DERIVATIVE_RAMP_FRACTION: Value = 0.125;
const TABLE2D_RESOURCE: &str = "xspice.table2d.data";
const TABLE3D_RESOURCE: &str = "xspice.table3d.data";

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

#[derive(Debug, Clone)]
struct TableLine {
    line: usize,
    header_ignored: bool,
    data_comment: bool,
    tokens: Vec<String>,
}

#[derive(Debug)]
struct TokenCursor {
    lines: Vec<TableLine>,
    next: usize,
}

impl TokenCursor {
    fn new(lines: Vec<TableLine>) -> Self {
        Self { lines, next: 0 }
    }

    fn next_line(&mut self, file: &str, model: &str, role: &str) -> CmResult<TableLine> {
        self.skip_header_ignored();
        let line = self.lines.get(self.next).ok_or_else(|| {
            table_file_error(
                model,
                file,
                format!("missing {role}; table file ended early"),
            )
        })?;
        self.next += 1;
        Ok(TableLine {
            line: line.line,
            header_ignored: line.header_ignored,
            data_comment: line.data_comment,
            tokens: line.tokens.clone(),
        })
    }

    fn next_data_line_optional(&mut self) -> Option<TableLine> {
        loop {
            let line = self.lines.get(self.next)?;
            self.next += 1;
            if line.data_comment {
                continue;
            }
            return Some(TableLine {
                line: line.line,
                header_ignored: line.header_ignored,
                data_comment: line.data_comment,
                tokens: line.tokens.clone(),
            });
        }
    }

    fn next_data_line(&mut self, file: &str, model: &str, role: &str) -> CmResult<TableLine> {
        self.next_data_line_optional().ok_or_else(|| {
            table_file_error(
                model,
                file,
                format!("missing {role}; table file ended early"),
            )
        })
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
        let token = line.tokens.first().ok_or_else(|| {
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
            let token = line.tokens.first().map(String::as_str).unwrap_or("");
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
    x: Vec<Value>,
    y: Vec<Value>,
    values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct Table3DData {
    x: Vec<Value>,
    y: Vec<Value>,
    z: Vec<Value>,
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

#[derive(Debug, Clone, Copy)]
struct TableEval2D {
    value: Value,
    dx: Value,
    dy: Value,
}

#[derive(Debug, Clone, Copy)]
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

fn tokenize_table_contents(model: &str, file: &str, contents: &str) -> CmResult<Vec<TableLine>> {
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
                tokens.push(token.to_string());
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

fn tokenize_table_file(model: &str, file: &str) -> CmResult<Vec<TableLine>> {
    let contents =
        data_file::read_to_string(file).map_err(|err| table_file_error(model, file, err))?;
    tokenize_table_contents(model, file, &contents)
}

fn parse_axis(
    cursor: &mut TokenCursor,
    file: &str,
    model: &str,
    role: &str,
    len: usize,
) -> CmResult<Vec<Value>> {
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
    for (index, token) in row.tokens.into_iter().enumerate() {
        let value = parse_table_spice_value(&token);
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
    Ok(axis)
}

fn parse_table2d_values(
    cursor: &mut TokenCursor,
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
        for (column, token) in table_row.tokens.into_iter().enumerate() {
            let value = parse_table_spice_value(&token);
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
    cursor: &mut TokenCursor,
    file: &str,
    model: &str,
    x_len: usize,
    y_len: usize,
    z_len: usize,
) -> CmResult<Vec<Value>> {
    let mut values = Vec::with_capacity(x_len * y_len * z_len);
    for z in 0..z_len {
        for y in 0..y_len {
            let table_row = cursor.next_data_line(file, model, &format!("table {z} row {y}"))?;
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
            for (x, token) in table_row.tokens.into_iter().enumerate() {
                let value = parse_table_spice_value(&token);
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
    let mut cursor = TokenCursor::new(tokenize_table_file(model, file)?);
    parse_table2d_cursor(file, model, &mut cursor)
}

fn parse_table2d_contents(file: &str, contents: &str) -> CmResult<Table2DData> {
    let model = TableKind::Table2D.model_name();
    let mut cursor = TokenCursor::new(tokenize_table_contents(model, file, contents)?);
    parse_table2d_cursor(file, model, &mut cursor)
}

fn parse_table2d_cursor(
    file: &str,
    model: &str,
    cursor: &mut TokenCursor,
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
    let mut cursor = TokenCursor::new(tokenize_table_file(model, file)?);
    parse_table3d_cursor(file, model, &mut cursor)
}

fn parse_table3d_contents(file: &str, contents: &str) -> CmResult<Table3DData> {
    let model = TableKind::Table3D.model_name();
    let mut cursor = TokenCursor::new(tokenize_table_contents(model, file, contents)?);
    parse_table3d_cursor(file, model, &mut cursor)
}

fn parse_table3d_cursor(
    file: &str,
    model: &str,
    cursor: &mut TokenCursor,
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

fn load_table2d_for_context(ctx: &mut CmContext, file: &str) -> CmResult<Arc<Table2DData>> {
    if let Some(table) = table2d_resource(ctx, file) {
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
    Ok(table)
}

fn load_table3d_for_context(ctx: &mut CmContext, file: &str) -> CmResult<Arc<Table3DData>> {
    if let Some(table) = table3d_resource(ctx, file) {
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
    Ok(table)
}

fn load_table2d_from_context(ctx: &CmContext, file: &str) -> CmResult<Arc<Table2DData>> {
    table2d_resource(ctx, file).map_or_else(|| load_table2d(file).map(|(table, _)| table), Ok)
}

fn load_table3d_from_context(ctx: &CmContext, file: &str) -> CmResult<Arc<Table3DData>> {
    table3d_resource(ctx, file).map_or_else(|| load_table3d(file).map(|(table, _)| table), Ok)
}

fn axis_eval(axis: &[Value], input: Value) -> AxisEval {
    let last_index = axis.len() - 1;
    let first = axis[0];
    let last = axis[last_index];

    let derivative_scale = if input < first {
        let ramp = BOUNDARY_DERIVATIVE_RAMP_FRACTION * (axis[1] - first);
        if ramp > 0.0 && input >= first - ramp {
            ((input - (first - ramp)) / ramp).clamp(0.0, 1.0)
        } else {
            0.0
        }
    } else if input > last {
        let ramp = BOUNDARY_DERIVATIVE_RAMP_FRACTION * (last - axis[last_index - 1]);
        if ramp > 0.0 && input <= last + ramp {
            ((last + ramp - input) / ramp).clamp(0.0, 1.0)
        } else {
            0.0
        }
    } else {
        1.0
    };

    let clamped = input.clamp(first, last);
    let eno_index = axis
        .partition_point(|value| *value <= clamped)
        .saturating_sub(1)
        .min(last_index);
    let eno_offset = clamped - axis[eno_index];
    let local_diff = if eno_index >= last_index {
        axis[last_index] - axis[last_index - 1]
    } else if eno_index == 0 {
        axis[1] - axis[0]
    } else {
        0.5 * (axis[eno_index + 1] - axis[eno_index - 1])
    };

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
        t: (clamped - axis[lower]) / (axis[lower + 1] - axis[lower]),
        derivative_scale,
        eno_index,
        eno_offset,
        local_diff,
    }
}

fn table2d_value(table: &Table2DData, x_index: usize, y_index: usize) -> Value {
    table.values[y_index * table.x.len() + x_index]
}

fn table3d_value(table: &Table3DData, x_index: usize, y_index: usize, z_index: usize) -> Value {
    table.values[(z_index * table.y.len() + y_index) * table.x.len() + x_index]
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
    debug_assert!(order >= 2);
    debug_assert!(order <= samples.len());
    debug_assert!(index < samples.len());

    let mut differences = Vec::with_capacity(order);
    differences.push(samples.to_vec());
    for degree in 1..order {
        let previous = &differences[degree - 1];
        let mut row = Vec::with_capacity(previous.len().saturating_sub(1));
        for i in 0..previous.len() - 1 {
            row.push(previous[i + 1] - previous[i]);
        }
        differences.push(row);
    }

    let max_start = samples.len() - order;
    let start_hi = index.min(max_start);
    let start_lo = start_hi.min(index.saturating_sub(order - 2));
    let smoothness = (start_lo..=start_hi)
        .map(|start| differences[order - 1][start].abs())
        .fold(Value::INFINITY, Value::min);

    let mut value = 0.0;
    let mut derivative = 0.0;
    let mut stencil_count = 0_usize;

    for start in start_lo..=start_hi {
        if differences[order - 1][start].abs() > smoothness {
            continue;
        }
        stencil_count += 1;

        let relative = offset + index as Value - start as Value;
        let mut basis = 1.0;
        let mut basis_derivative = 0.0;

        for (degree, row) in differences.iter().enumerate().take(order) {
            let divided_difference = row[start];
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

fn eno2d_eval_grid(
    rows: &[Vec<Value>],
    order: usize,
    x_index: usize,
    y_index: usize,
    x_offset: Value,
    y_offset: Value,
) -> Eno2dEval {
    let y_window_start = eno_window_start(y_index, order, rows.len());
    let y_window = 2 * order - 2;
    let y_local_index = y_index - y_window_start;

    let mut values_along_y = Vec::with_capacity(y_window);
    let mut dx_along_y = Vec::with_capacity(y_window);
    for row in rows.iter().skip(y_window_start).take(y_window) {
        let eval = eno1d_eval(row, order, x_index, x_offset);
        values_along_y.push(eval.value);
        dx_along_y.push(eval.derivative);
    }

    let y_eval = eno1d_eval(&values_along_y, order, y_local_index, y_offset);
    let dx_eval = eno1d_eval(&dx_along_y, order, y_local_index, y_offset);
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
    let mut rows = Vec::with_capacity(table.y.len());
    for y in 0..table.y.len() {
        let mut row = Vec::with_capacity(table.x.len());
        for x in 0..table.x.len() {
            row.push(table2d_value(table, x, y));
        }
        rows.push(row);
    }

    eno2d_eval_grid(
        &rows,
        order,
        x_axis.eno_index,
        y_axis.eno_index,
        x_axis.eno_offset,
        y_axis.eno_offset,
    )
}

fn table3d_eno_derivatives(
    table: &Table3DData,
    order: usize,
    x_axis: AxisEval,
    y_axis: AxisEval,
    z_axis: AxisEval,
) -> Eno3dEval {
    let yz_window = 2 * order - 2;
    let y_window_start = eno_window_start(y_axis.eno_index, order, table.y.len());
    let z_window_start = eno_window_start(z_axis.eno_index, order, table.z.len());
    let y_local_index = y_axis.eno_index - y_window_start;
    let z_local_index = z_axis.eno_index - z_window_start;

    let mut values_over_zy = Vec::with_capacity(yz_window);
    let mut dx_over_zy = Vec::with_capacity(yz_window);
    for z in z_window_start..z_window_start + yz_window {
        let mut value_row = Vec::with_capacity(yz_window);
        let mut dx_row = Vec::with_capacity(yz_window);
        for y in y_window_start..y_window_start + yz_window {
            let mut x_row = Vec::with_capacity(table.x.len());
            for x in 0..table.x.len() {
                x_row.push(table3d_value(table, x, y, z));
            }
            let eval = eno1d_eval(&x_row, order, x_axis.eno_index, x_axis.eno_offset);
            value_row.push(eval.value);
            dx_row.push(eval.derivative);
        }
        values_over_zy.push(value_row);
        dx_over_zy.push(dx_row);
    }

    let yz_eval = eno2d_eval_grid(
        &values_over_zy,
        order,
        y_local_index,
        z_local_index,
        y_axis.eno_offset,
        z_axis.eno_offset,
    );
    let dx_eval = eno2d_eval_grid(
        &dx_over_zy,
        order,
        y_local_index,
        z_local_index,
        y_axis.eno_offset,
        z_axis.eno_offset,
    );

    Eno3dEval {
        dx: dx_eval.value,
        dy: yz_eval.dx,
        dz: yz_eval.dy,
    }
}

fn evaluate_table2d_data(table: &Table2DData, order: usize, x: Value, y: Value) -> TableEval2D {
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

    let derivatives = table2d_eno_derivatives(table, order, x_axis, y_axis);
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

    let derivatives = table3d_eno_derivatives(table, order, x_axis, y_axis, z_axis);

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

fn scaled_table2d_eval(ctx: &CmContext) -> CmResult<TableEval2D> {
    let file = table_file_param(ctx, TableKind::Table2D);
    let table = load_table2d_from_context(ctx, file)?;
    let order = table_order(ctx)?;
    validate_table2d_order(&table, order)?;
    let raw = evaluate_table2d_data(&table, order, ctx.input("inx"), ctx.input("iny"));
    let gain = ctx.param("gain");
    Ok(TableEval2D {
        value: ctx.param("offset") + gain * raw.value,
        dx: gain * raw.dx,
        dy: gain * raw.dy,
    })
}

fn scaled_table3d_eval(ctx: &CmContext) -> CmResult<TableEval3D> {
    let file = table_file_param(ctx, TableKind::Table3D);
    let table = load_table3d_from_context(ctx, file)?;
    let order = table_order(ctx)?;
    validate_table3d_order(&table, order)?;
    let raw = evaluate_table3d_data(
        &table,
        order,
        ctx.input("inx"),
        ctx.input("iny"),
        ctx.input("inz"),
    );
    let gain = ctx.param("gain");
    Ok(TableEval3D {
        value: ctx.param("offset") + gain * raw.value,
        dx: gain * raw.dx,
        dy: gain * raw.dy,
        dz: gain * raw.dz,
    })
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
        let eval = scaled_table2d_eval(ctx)?;
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
        let eval = scaled_table3d_eval(ctx)?;
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

//! Official XSPICE transfer-function code models.

use crate::{
    Complex64, Value,
    xspice::{
        AnalysisType, CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec,
        PortDirection, PortSpec, PortType, data_file,
    },
};
use std::f64::consts::PI;
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

const XFER_TABLE_RESOURCE: &str = "xspice.xfer.table";
const SXFER_COEFFICIENT_RESOURCE: &str = "xspice.s_xfer.coefficients";
const SXFER_TRANSIENT_SCRATCH_RESOURCE: &str = "xspice.s_xfer.transient_scratch";
const XFER_UNSET_UPPER_INDEX: usize = usize::MAX;
const XFER_CURSOR_LINEAR_STEPS: usize = 8;

#[derive(Debug, Clone)]
struct XferPoint {
    frequency: Value,
    gain: Complex64,
}

#[derive(Debug)]
struct XferTableData {
    points: Vec<XferPoint>,
    strictly_increasing_frequency: bool,
    last_upper_index: AtomicUsize,
}

#[derive(Debug, Clone)]
struct SXferCoefficients {
    numerator: Vec<Value>,
    denominator: Vec<Value>,
    gain: Value,
}

#[derive(Debug, Clone, PartialEq)]
struct XferTableSignature {
    file: String,
    file_virtual_stamp: Option<data_file::DataFileStamp>,
    table_revision: Option<u64>,
    table_provided: bool,
    r_i: Value,
    r_i_provided: bool,
    db: Value,
    db_provided: bool,
    rad: Value,
    rad_provided: bool,
    span: Value,
    offset: Value,
}

#[derive(Debug, Clone)]
struct XferTableResource {
    signature: XferTableSignature,
    result: CmResult<Arc<XferTableData>>,
}

#[derive(Debug, Clone, PartialEq)]
struct SXferCoefficientSignature {
    gain: Value,
    denormalized_freq: Value,
    num_coeff_revision: Option<u64>,
    den_coeff_revision: Option<u64>,
}

#[derive(Debug, Clone)]
struct SXferCoefficientResource {
    signature: SXferCoefficientSignature,
    coefficients: Option<Arc<SXferCoefficients>>,
}

type SXferTransientScratchResource = Vec<Value>;

fn transfer_ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::Voltage,
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
                description: "Small-signal input".to_string(),
            },
            PortSpec {
                name: "out".to_string(),
                direction: PortDirection::Out,
                default_type: PortType::Voltage,
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
                description: "Small-signal output".to_string(),
            },
        ]
    })
}

fn xfer_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("table", vec![0.0, 0.0, 0.0])
                .with_vector_min_len(3)
                .with_description("Frequency/value/value table, selected by span and offset"),
            ParamSpec::string("file", "").with_description("Touchstone-format data file"),
            ParamSpec::boolean("r_i", false)
                .with_description("Interpret table values as real/imaginary pairs"),
            ParamSpec::boolean("db", true)
                .with_description("Interpret table magnitude values as dB"),
            ParamSpec::boolean("rad", false).with_description("Interpret phase values as radians"),
            ParamSpec::integer("span", 3)
                .with_description("Number of values per source row, clamped to at least 3"),
            ParamSpec::integer("offset", 1).with_description(
                "One-based offset of the selected value pair, clamped to at least 1",
            ),
        ]
    })
}

fn s_xfer_params() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real("in_offset", 0.0).with_description("Input offset"),
            ParamSpec::real("gain", 1.0).with_description("Transfer-function gain"),
            ParamSpec::real_vector("num_coeff", Vec::new())
                .required()
                .with_vector_min_len(1)
                .with_description("Numerator coefficients in descending powers of s"),
            ParamSpec::real_vector("den_coeff", Vec::new())
                .required()
                .with_vector_min_len(1)
                .with_description("Denominator coefficients in descending powers of s"),
            ParamSpec::real_vector("int_ic", Vec::new()).with_description(
                "Integrator initial conditions, defaulting missing states to official 0.0",
            ),
            ParamSpec::real("denormalized_freq", 1.0)
                .with_description("Coefficient denormalization frequency in radians/second"),
        ]
    })
}

fn xfer_error(message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: "xfer".to_string(),
        message: message.into(),
    }
}

fn reserve_xfer_points(source: &str, row_count: usize) -> CmResult<Vec<XferPoint>> {
    let mut points = Vec::new();
    points.try_reserve_exact(row_count).map_err(|err| {
        xfer_error(format!(
            "{source} has {row_count} row(s), unable to reserve transfer points: {err}"
        ))
    })?;
    Ok(points)
}

fn xfer_param_error(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn s_xfer_error(message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: "s_xfer".to_string(),
        message: message.into(),
    }
}

fn resize_s_xfer_values(context: &str, values: &mut Vec<Value>, len: usize) -> CmResult<()> {
    if values.capacity() < len {
        let additional = len - values.capacity();
        values.try_reserve_exact(additional).map_err(|err| {
            s_xfer_error(format!(
                "{context}: unable to reserve {len} value(s): {err}"
            ))
        })?;
    }
    values.resize(len, 0.0);
    Ok(())
}

fn s_xfer_param_error(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn xfer_complex_value(a: Value, b: Value, ri: bool, db: bool, rad: bool) -> CmResult<Complex64> {
    if !a.is_finite() || !b.is_finite() {
        return Err(xfer_error("table values must be finite"));
    }

    if ri {
        return Ok(Complex64::new(a, b));
    }

    let magnitude = if db { 10.0_f64.powf(a / 20.0) } else { a };
    let phase = if rad { b } else { b * PI / 180.0 };
    if !magnitude.is_finite() || !phase.is_finite() {
        return Err(xfer_error(
            "table values must produce finite magnitude and phase",
        ));
    }

    let gain = Complex64::from_polar(magnitude, phase);
    if !gain.re.is_finite() || !gain.im.is_finite() {
        return Err(xfer_error("table values must produce finite gain"));
    }
    Ok(gain)
}

fn xfer_bool_param(ctx: &CmContext, name: &str) -> CmResult<bool> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(xfer_param_error(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    Ok(value > 0.5)
}

fn xfer_int_param(ctx: &CmContext, name: &str, min: Value) -> CmResult<usize> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(xfer_param_error(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    let rounded = value.round().max(min);
    if rounded > usize::MAX as Value {
        return Err(xfer_param_error(
            name,
            format!("value is too large to index a table row, got {value}"),
        ));
    }
    Ok(rounded as usize)
}

fn xfer_effective_span_offset(ctx: &CmContext) -> CmResult<(usize, usize)> {
    let span = xfer_int_param(ctx, "span", 3.0)?;
    let offset = xfer_int_param(ctx, "offset", 1.0)?;
    Ok((span, offset))
}

fn xfer_table_from_model(ctx: &CmContext) -> CmResult<Vec<XferPoint>> {
    let table = ctx.real_vector_param("table").unwrap_or(&[]);
    if table.is_empty() {
        return Err(xfer_error(
            "table parameter is required when file is not provided",
        ));
    }

    let (span, offset) = xfer_effective_span_offset(ctx)?;
    if offset < 1 || span < offset + 2 {
        return Ok(Vec::new());
    }
    if table.len() % span != 0 {
        return Ok(Vec::new());
    }

    let ri = xfer_bool_param(ctx, "r_i")?;
    let db = xfer_bool_param(ctx, "db")?;
    let rad = xfer_bool_param(ctx, "rad")?;
    let row_count = table.len() / span;
    let mut points = reserve_xfer_points("table parameter", row_count)?;
    let mut last_frequency = None;
    for row in table.chunks_exact(span) {
        let frequency = row[0];
        if !frequency.is_finite() {
            return Err(xfer_error("table frequencies must be finite"));
        }
        if row_count > 1 && frequency < 0.0 {
            return Ok(Vec::new());
        }
        if let Some(previous) = last_frequency
            && frequency < previous
        {
            return Ok(Vec::new());
        }
        let gain = xfer_complex_value(row[offset], row[offset + 1], ri, db, rad)?;
        points.push(XferPoint { frequency, gain });
        last_frequency = Some(frequency);
    }

    if points.is_empty() {
        return Err(xfer_error("table must contain at least one row"));
    }
    Ok(points)
}

fn parse_touchstone_options(line: &str) -> (bool, bool) {
    let mut ri = false;
    let mut db = false;

    for token in line.trim_start_matches('#').split_whitespace() {
        match token {
            "RI" => ri = true,
            "DB" => db = true,
            _ => {}
        }
    }

    (ri, db)
}

fn parse_touchstone_numbers(line: &str) -> Vec<Value> {
    let mut numbers = Vec::new();
    let mut input = line;

    while numbers.len() < 9 {
        input = input.trim_start();
        let Some((value, consumed)) = data_file::parse_numeric_prefix_len(input) else {
            break;
        };
        numbers.push(value);
        input = &input[consumed..];
    }

    numbers
}

fn parse_touchstone_rows(contents: &str, span: usize, offset: usize) -> Vec<[Value; 3]> {
    let mut selected = Vec::new();
    let mut skip = 0usize;
    let mut want = 0u8;

    for line in contents.lines() {
        let data = line.split('!').next().unwrap_or("").trim();
        if data.is_empty() || data.starts_with('#') {
            continue;
        }
        let numbers = parse_touchstone_numbers(data);
        if span == 9 && numbers.len() == 5 {
            break;
        }
        let mut index = if skip > 0 {
            if numbers.len() > skip {
                let index = skip;
                skip = 0;
                index
            } else {
                skip -= numbers.len();
                continue;
            }
        } else {
            0
        };

        while index < numbers.len() {
            selected.push(numbers[index]);
            match want {
                0 => {
                    want = 2;
                    index += offset;
                    if index >= numbers.len() {
                        skip = index - numbers.len();
                    }
                }
                1 => {
                    want = 0;
                    index += span - offset - 1;
                    if index >= numbers.len() {
                        skip = index - numbers.len();
                    }
                }
                2 => {
                    index += 1;
                    want = 1;
                    skip = 0;
                }
                _ => unreachable!("xfer Touchstone scanner has only three states"),
            }
        }
    }

    selected
        .chunks_exact(3)
        .map(|chunk| [chunk[0], chunk[1], chunk[2]])
        .collect()
}

fn xfer_table_from_touchstone(
    ctx: &CmContext,
    file: &str,
) -> (Option<data_file::DataFileStamp>, CmResult<Vec<XferPoint>>) {
    let (span, offset) = match xfer_effective_span_offset(ctx) {
        Ok(values) => values,
        Err(err) => return (None, Err(err)),
    };
    if offset < 1 || span < offset + 2 {
        return (None, Ok(Vec::new()));
    }

    let (contents, stamp) = match data_file::read_to_string_with_stamp(file) {
        Ok(file) => file,
        Err(err) => {
            return (
                None,
                Err(xfer_error(format!(
                    "failed to read Touchstone file '{file}': {err}"
                ))),
            );
        }
    };
    let file_virtual_stamp = data_file::loaded_virtual_data_file_stamp(stamp);
    let option_line = match contents.lines().find(|line| line.starts_with('#')) {
        Some(line) => line,
        None => return (file_virtual_stamp, Ok(Vec::new())),
    };
    let (mut ri, mut db) = parse_touchstone_options(option_line.trim_start());
    let mut rad = false;
    if ctx.param_was_provided("r_i") {
        ri = match xfer_bool_param(ctx, "r_i") {
            Ok(value) => value,
            Err(err) => return (file_virtual_stamp, Err(err)),
        };
    }
    if ctx.param_was_provided("db") {
        db = match xfer_bool_param(ctx, "db") {
            Ok(value) => value,
            Err(err) => return (file_virtual_stamp, Err(err)),
        };
    }
    if ctx.param_was_provided("rad") {
        rad = match xfer_bool_param(ctx, "rad") {
            Ok(value) => value,
            Err(err) => return (file_virtual_stamp, Err(err)),
        };
    }
    let rows = parse_touchstone_rows(&contents, span, offset);
    let mut points = match reserve_xfer_points(file, rows.len()) {
        Ok(points) => points,
        Err(err) => return (file_virtual_stamp, Err(err)),
    };
    for [frequency, a, b] in rows {
        // ngspice parses the Touchstone frequency unit token for xfer files
        // but stores the raw frequency value without applying the multiplier.
        if !frequency.is_finite() {
            return (
                file_virtual_stamp,
                Err(xfer_error(format!(
                    "Touchstone file '{file}' contains an invalid frequency"
                ))),
            );
        }
        let gain = match xfer_complex_value(a, b, ri, db, rad) {
            Ok(gain) => gain,
            Err(err) => return (file_virtual_stamp, Err(err)),
        };
        points.push(XferPoint { frequency, gain });
    }

    if points.is_empty() {
        return (
            file_virtual_stamp,
            Err(xfer_error(format!(
                "Touchstone file '{file}' contains no transfer rows"
            ))),
        );
    }
    (file_virtual_stamp, Ok(points))
}

fn xfer_table_uncached(
    ctx: &CmContext,
) -> (Option<data_file::DataFileStamp>, CmResult<Vec<XferPoint>>) {
    let file = ctx.string_param("file").unwrap_or("").trim();
    if !file.is_empty() {
        if ctx.param_was_provided("table")
            && ctx
                .real_vector_param("table")
                .is_some_and(|table| !table.is_empty())
        {
            return (
                data_file::virtual_data_file_stamp(file),
                Err(xfer_error(
                    "file and table parameters are mutually exclusive",
                )),
            );
        }
        return xfer_table_from_touchstone(ctx, file);
    }

    (None, xfer_table_from_model(ctx))
}

fn xfer_table_data(points: Vec<XferPoint>) -> XferTableData {
    let strictly_increasing_frequency = points
        .windows(2)
        .all(|pair| pair[0].frequency < pair[1].frequency);
    XferTableData {
        points,
        strictly_increasing_frequency,
        last_upper_index: AtomicUsize::new(XFER_UNSET_UPPER_INDEX),
    }
}

fn xfer_table_signature_with_virtual_stamp(
    ctx: &CmContext,
    file_virtual_stamp: Option<data_file::DataFileStamp>,
) -> XferTableSignature {
    let file = ctx.string_param("file").unwrap_or("").trim().to_string();
    XferTableSignature {
        file_virtual_stamp,
        file,
        table_revision: ctx.real_vector_param_revision("table"),
        table_provided: ctx.param_was_provided("table"),
        r_i: ctx.param("r_i"),
        r_i_provided: ctx.param_was_provided("r_i"),
        db: ctx.param("db"),
        db_provided: ctx.param_was_provided("db"),
        rad: ctx.param("rad"),
        rad_provided: ctx.param_was_provided("rad"),
        span: ctx.param("span"),
        offset: ctx.param("offset"),
    }
}

fn xfer_table_signature_matches(ctx: &CmContext, signature: &XferTableSignature) -> bool {
    let file = ctx.string_param("file").unwrap_or("").trim();
    signature.file == file
        && signature.file_virtual_stamp == data_file::virtual_data_file_stamp(file)
        && ctx.real_vector_param_revision("table") == signature.table_revision
        && signature.table_provided == ctx.param_was_provided("table")
        && signature.r_i == ctx.param("r_i")
        && signature.r_i_provided == ctx.param_was_provided("r_i")
        && signature.db == ctx.param("db")
        && signature.db_provided == ctx.param_was_provided("db")
        && signature.rad == ctx.param("rad")
        && signature.rad_provided == ctx.param_was_provided("rad")
        && signature.span == ctx.param("span")
        && signature.offset == ctx.param("offset")
}

fn cache_xfer_table(ctx: &mut CmContext) -> CmResult<Arc<XferTableData>> {
    if let Some(resource) = ctx.resource::<XferTableResource>(XFER_TABLE_RESOURCE)
        && xfer_table_signature_matches(ctx, &resource.signature)
    {
        return resource.result.clone();
    }

    let (file_virtual_stamp, result) = xfer_table_uncached(ctx);
    let signature = xfer_table_signature_with_virtual_stamp(ctx, file_virtual_stamp);
    let result = result.map(xfer_table_data).map(Arc::new);
    let cached = result.clone();
    ctx.set_resource(
        XFER_TABLE_RESOURCE,
        Arc::new(XferTableResource { signature, result }),
    );
    cached
}

fn xfer_table(ctx: &CmContext) -> CmResult<Arc<XferTableData>> {
    if let Some(resource) = ctx.resource::<XferTableResource>(XFER_TABLE_RESOURCE) {
        if xfer_table_signature_matches(ctx, &resource.signature) {
            return resource.result.clone();
        }
    }
    let (_, result) = xfer_table_uncached(ctx);
    result.map(xfer_table_data).map(Arc::new)
}

fn xfer_gain_linear_scan(points: &[XferPoint], frequency: Value) -> Complex64 {
    let Some(first) = points.first() else {
        return Complex64::new(0.0, 0.0);
    };
    let Some(last) = points.last() else {
        return first.gain;
    };

    if frequency <= first.frequency {
        return first.gain;
    }
    if frequency >= last.frequency {
        return last.gain;
    }

    for pair in points.windows(2) {
        let lower = &pair[0];
        let upper = &pair[1];
        if frequency <= upper.frequency {
            let span = upper.frequency - lower.frequency;
            if span.abs() <= Value::EPSILON {
                return upper.gain;
            }
            let factor = (frequency - lower.frequency) / span;
            return lower.gain + (upper.gain - lower.gain) * factor;
        }
    }
    last.gain
}

fn xfer_interpolated_gain(lower: &XferPoint, upper: &XferPoint, frequency: Value) -> Complex64 {
    let span = upper.frequency - lower.frequency;
    if span.abs() <= Value::EPSILON {
        return upper.gain;
    }
    let factor = (frequency - lower.frequency) / span;
    lower.gain + (upper.gain - lower.gain) * factor
}

fn xfer_interval_contains(points: &[XferPoint], upper_index: usize, frequency: Value) -> bool {
    debug_assert!(upper_index > 0);
    debug_assert!(upper_index < points.len());
    points[upper_index - 1].frequency <= frequency && frequency <= points[upper_index].frequency
}

fn xfer_upper_index_binary(points: &[XferPoint], frequency: Value) -> usize {
    points.partition_point(|point| point.frequency < frequency)
}

fn xfer_upper_index_with_cursor(table: &XferTableData, frequency: Value) -> usize {
    let points = table.points.as_slice();
    let point_count = points.len();
    let mut upper_index = table.last_upper_index.load(Ordering::Relaxed);

    if upper_index == XFER_UNSET_UPPER_INDEX || upper_index == 0 || upper_index >= point_count {
        upper_index = xfer_upper_index_binary(points, frequency);
        table.last_upper_index.store(upper_index, Ordering::Relaxed);
        return upper_index;
    }

    if xfer_interval_contains(points, upper_index, frequency) {
        return upper_index;
    }

    let mut steps = 0;
    if frequency > points[upper_index].frequency {
        while upper_index + 1 < point_count
            && frequency > points[upper_index].frequency
            && steps < XFER_CURSOR_LINEAR_STEPS
        {
            upper_index += 1;
            steps += 1;
        }
    } else {
        while upper_index > 1
            && frequency < points[upper_index - 1].frequency
            && steps < XFER_CURSOR_LINEAR_STEPS
        {
            upper_index -= 1;
            steps += 1;
        }
    }

    if !xfer_interval_contains(points, upper_index, frequency) {
        upper_index = xfer_upper_index_binary(points, frequency);
    }
    table.last_upper_index.store(upper_index, Ordering::Relaxed);
    upper_index
}

fn xfer_gain_at(table: &XferTableData, frequency: Value) -> Complex64 {
    let points = table.points.as_slice();
    if !table.strictly_increasing_frequency {
        return xfer_gain_linear_scan(points, frequency);
    }

    let Some(first) = points.first() else {
        return Complex64::new(0.0, 0.0);
    };
    let Some(last) = points.last() else {
        return first.gain;
    };

    if frequency <= first.frequency {
        return first.gain;
    }
    if frequency >= last.frequency {
        return last.gain;
    }

    let upper_index = xfer_upper_index_with_cursor(table, frequency);
    xfer_interpolated_gain(&points[upper_index - 1], &points[upper_index], frequency)
}

fn xfer_first_real_gain(ctx: &CmContext) -> Value {
    xfer_table(ctx)
        .ok()
        .and_then(|table| table.points.first().map(|point| point.gain.re))
        .unwrap_or(0.0)
}

fn xfer_first_real_gain_cached(ctx: &mut CmContext) -> CmResult<Value> {
    Ok(cache_xfer_table(ctx)?
        .points
        .first()
        .map(|point| point.gain.re)
        .unwrap_or(0.0))
}

fn finite_coefficients(name: &str, values: &[Value]) -> CmResult<()> {
    if values.is_empty() {
        return Err(s_xfer_error(format!(
            "{name} coefficient array is required"
        )));
    }
    if values.iter().any(|value| !value.is_finite()) {
        return Err(s_xfer_error(format!(
            "{name} coefficient array contains a non-finite value"
        )));
    }
    Ok(())
}

fn finite_s_xfer_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(s_xfer_param_error(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    Ok(value)
}

fn finite_s_xfer_vector_param(ctx: &CmContext, name: &str) -> CmResult<Vec<Value>> {
    let values = ctx.real_vector_param(name).unwrap_or(&[]);
    for (index, value) in values.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(s_xfer_param_error(
                name,
                format!("element {index} must be finite, got {value}"),
            ));
        }
    }
    Ok(values.to_vec())
}

fn descending_to_denormalized_ascending(
    coefficients: &[Value],
    denormalized_freq: Value,
) -> Vec<Value> {
    coefficients
        .iter()
        .rev()
        .enumerate()
        .map(|(power, coefficient)| coefficient / denormalized_freq.powi(power as i32))
        .collect()
}

fn s_xfer_coefficients(ctx: &CmContext) -> CmResult<SXferCoefficients> {
    let numerator_desc = ctx.real_vector_param("num_coeff").unwrap_or(&[]);
    let denominator_desc = ctx.real_vector_param("den_coeff").unwrap_or(&[]);
    finite_coefficients("Numerator", numerator_desc)?;
    finite_coefficients("Denominator", denominator_desc)?;

    if numerator_desc.len() > denominator_desc.len() {
        return Err(s_xfer_error(
            "Numerator coefficient array size greater than denominator coefficient array size",
        ));
    }

    let denormalized_freq = ctx.param_or("denormalized_freq", 1.0);
    if !denormalized_freq.is_finite() || denormalized_freq == 0.0 {
        return Err(s_xfer_error(
            "denormalized_freq must be finite and non-zero",
        ));
    }

    let mut numerator = descending_to_denormalized_ascending(numerator_desc, denormalized_freq);
    let mut denominator = descending_to_denormalized_ascending(denominator_desc, denormalized_freq);
    let leading = denominator.last().copied().unwrap_or(0.0);
    if leading.abs() <= 1.0e-30 {
        return Err(s_xfer_error(
            "highest-order denominator coefficient must be non-zero",
        ));
    }

    let mut gain = ctx.param("gain");
    if leading != 1.0 {
        for coefficient in &mut denominator {
            *coefficient /= leading;
        }
        gain /= leading;
    }
    if !gain.is_finite() {
        return Err(s_xfer_error("gain must be finite"));
    }
    for coefficient in &mut numerator {
        if !coefficient.is_finite() {
            return Err(s_xfer_error(
                "denormalized numerator coefficient is non-finite",
            ));
        }
    }
    for coefficient in &denominator {
        if !coefficient.is_finite() {
            return Err(s_xfer_error(
                "denormalized denominator coefficient is non-finite",
            ));
        }
    }

    Ok(SXferCoefficients {
        numerator,
        denominator,
        gain,
    })
}

fn s_xfer_coefficient_signature(ctx: &CmContext) -> SXferCoefficientSignature {
    SXferCoefficientSignature {
        gain: ctx.param("gain"),
        denormalized_freq: ctx.param_or("denormalized_freq", 1.0),
        num_coeff_revision: ctx.real_vector_param_revision("num_coeff"),
        den_coeff_revision: ctx.real_vector_param_revision("den_coeff"),
    }
}

fn s_xfer_coefficient_signature_matches(
    ctx: &CmContext,
    signature: &SXferCoefficientSignature,
) -> bool {
    signature.gain == ctx.param("gain")
        && signature.denormalized_freq == ctx.param_or("denormalized_freq", 1.0)
        && ctx.real_vector_param_revision("num_coeff") == signature.num_coeff_revision
        && ctx.real_vector_param_revision("den_coeff") == signature.den_coeff_revision
}

fn s_xfer_has_improper_order(ctx: &CmContext) -> bool {
    let numerator_len = ctx.real_vector_param("num_coeff").map_or(0, <[Value]>::len);
    let denominator_len = ctx.real_vector_param("den_coeff").map_or(0, <[Value]>::len);
    numerator_len > denominator_len
}

fn cache_s_xfer_coefficients(ctx: &mut CmContext) -> CmResult<Option<Arc<SXferCoefficients>>> {
    let signature = s_xfer_coefficient_signature(ctx);
    if let Some(resource) = ctx.resource::<SXferCoefficientResource>(SXFER_COEFFICIENT_RESOURCE)
        && resource.signature == signature
    {
        return Ok(resource.coefficients.clone());
    }

    let coefficients = if s_xfer_has_improper_order(ctx) {
        // ngspice diagnoses an improper transfer function and leaves the
        // code-model instance unstamped rather than aborting the analysis.
        None
    } else {
        Some(Arc::new(s_xfer_coefficients(ctx)?))
    };
    ctx.set_resource(
        SXFER_COEFFICIENT_RESOURCE,
        Arc::new(SXferCoefficientResource {
            signature,
            coefficients: coefficients.clone(),
        }),
    );
    Ok(coefficients)
}

fn s_xfer_coefficients_for_context(ctx: &CmContext) -> CmResult<Option<Arc<SXferCoefficients>>> {
    if let Some(resource) = ctx.resource::<SXferCoefficientResource>(SXFER_COEFFICIENT_RESOURCE) {
        if s_xfer_coefficient_signature_matches(ctx, &resource.signature) {
            return Ok(resource.coefficients.clone());
        }
    }
    if s_xfer_has_improper_order(ctx) {
        Ok(None)
    } else {
        s_xfer_coefficients(ctx).map(Arc::new).map(Some)
    }
}

fn evaluate_ascending_polynomial(coefficients: &[Value], s: Complex64) -> Complex64 {
    let mut acc = Complex64::new(0.0, 0.0);
    for coefficient in coefficients.iter().rev() {
        acc = acc * s + *coefficient;
    }
    acc
}

fn s_xfer_gain_at(coefficients: &SXferCoefficients, frequency: Value) -> Complex64 {
    let s = Complex64::new(0.0, 2.0 * PI * frequency);
    let numerator = evaluate_ascending_polynomial(&coefficients.numerator, s) * coefficients.gain;
    let denominator = evaluate_ascending_polynomial(&coefficients.denominator, s);
    if denominator.norm_sqr() <= 1.0e-60 {
        Complex64::new(0.0, 0.0)
    } else {
        numerator / denominator
    }
}

fn s_xfer_dc_gain(ctx: &CmContext) -> Value {
    let Ok(Some(coefficients)) = s_xfer_coefficients_for_context(ctx) else {
        return 0.0;
    };
    let den0 = coefficients.denominator.first().copied().unwrap_or(0.0);
    if den0.abs() <= 1.0e-30 {
        0.0
    } else {
        coefficients.gain * coefficients.numerator.first().copied().unwrap_or(0.0) / den0
    }
}

fn s_xfer_feedthrough(coefficients: &SXferCoefficients) -> Value {
    let order = coefficients.denominator.len().saturating_sub(1);
    if order == 0 {
        return coefficients.numerator.first().copied().unwrap_or(0.0);
    }

    if coefficients.numerator.len() == order + 1 {
        coefficients.numerator[order]
    } else {
        0.0
    }
}

fn ensure_s_xfer_transient_scratch(ctx: &mut CmContext) {
    if ctx
        .resource::<SXferTransientScratchResource>(SXFER_TRANSIENT_SCRATCH_RESOURCE)
        .is_none()
    {
        ctx.set_resource(
            SXFER_TRANSIENT_SCRATCH_RESOURCE,
            Arc::new(Vec::<Value>::new()),
        );
    }
}

fn with_s_xfer_transient_scratch<R>(
    ctx: &mut CmContext,
    f: impl FnOnce(&mut CmContext, &mut Vec<Value>) -> CmResult<R>,
) -> CmResult<R> {
    ensure_s_xfer_transient_scratch(ctx);
    let mut scratch = {
        let scratch = ctx
            .resource_make_mut::<SXferTransientScratchResource>(SXFER_TRANSIENT_SCRATCH_RESOURCE)
            .ok_or_else(|| {
                s_xfer_error("transient scratch is not initialized or has an unexpected type")
            })?;
        std::mem::take(scratch)
    };
    let result = f(ctx, &mut scratch);
    let restore = ctx
        .resource_make_mut::<SXferTransientScratchResource>(SXFER_TRANSIENT_SCRATCH_RESOURCE)
        .ok_or_else(|| {
            s_xfer_error("transient scratch is not initialized or has an unexpected type")
        })
        .map(|slot| {
            *slot = scratch;
        });

    match (result, restore) {
        (Ok(value), Ok(())) => Ok(value),
        (Err(err), Ok(())) => Err(err),
        (_, Err(err)) => Err(err),
    }
}

fn s_xfer_input_partial_for_context(ctx: &CmContext) -> CmResult<Value> {
    let Some(coefficients) = s_xfer_coefficients_for_context(ctx)? else {
        return Ok(0.0);
    };

    let mut scratch = Vec::new();
    let (_, partial) = s_xfer_ngspice_transient_eval(ctx, coefficients.as_ref(), &mut scratch)?;
    Ok(partial)
}

fn s_xfer_ngspice_transient_eval(
    ctx: &CmContext,
    coefficients: &SXferCoefficients,
    state: &mut Vec<Value>,
) -> CmResult<(Value, Value)> {
    let order = coefficients.denominator.len() - 1;
    let input = ctx.input("in") + finite_s_xfer_param(ctx, "in_offset")?;
    let u = coefficients.gain * input;

    if order == 0 {
        let feedthrough = s_xfer_feedthrough(coefficients);
        return Ok((feedthrough * u, feedthrough * coefficients.gain));
    }

    resize_s_xfer_values("transient state scratch", state, order + 1)?;

    state[order] = u;
    for index in 0..order {
        state[order] -= ctx.state_prev(index) * coefficients.denominator[index];
    }

    let dt = ctx.timestep;
    if dt.is_finite() && dt > 0.0 {
        for index in (1..=order).rev() {
            state[index - 1] = ctx.state_prev(index - 1) + dt * state[index];
        }
    } else {
        for index in 0..order {
            state[index] = ctx.state_prev(index);
        }
    }

    let output = coefficients
        .numerator
        .iter()
        .enumerate()
        .map(|(index, coefficient)| coefficient * state.get(index).copied().unwrap_or(0.0))
        .sum();
    let partial = state.get(1).copied().unwrap_or(0.0);
    Ok((output, partial))
}

fn s_xfer_transient_eval(
    ctx: &mut CmContext,
    coefficients: &Arc<SXferCoefficients>,
) -> CmResult<(Value, Value)> {
    with_s_xfer_transient_scratch(ctx, |ctx, state| {
        let (output, partial) = s_xfer_ngspice_transient_eval(ctx, coefficients.as_ref(), state)?;
        if transfer_commits_state(ctx) {
            for (index, value) in state.iter().copied().enumerate() {
                ctx.set_state(index, value);
            }
        }
        Ok((output, partial))
    })
}

fn transfer_commits_state(ctx: &CmContext) -> bool {
    ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe
}

/// Official XSPICE `xfer` AC transfer-function table.
#[derive(Debug, Default)]
pub struct Xfer;

impl CodeModel for Xfer {
    fn name(&self) -> &str {
        "xfer"
    }

    fn description(&self) -> &str {
        "AC transfer function table"
    }

    fn ports(&self) -> &[PortSpec] {
        transfer_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        xfer_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        cache_xfer_table(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.analysis == AnalysisType::Ac {
            return Ok(());
        }

        let gain = xfer_first_real_gain_cached(ctx)?;
        ctx.set_output_with_partial("out", gain * ctx.input("in"), gain);
        Ok(())
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        vec![xfer_first_real_gain(ctx)]
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), xfer_first_real_gain(ctx))]
        } else {
            Vec::new()
        }
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        match xfer_table(ctx) {
            Ok(table) => vec![("in".to_string(), xfer_gain_at(table.as_ref(), frequency))],
            Err(_) => Vec::new(),
        }
    }
}

/// Official XSPICE `s_xfer` s-domain transfer-function block.
#[derive(Debug, Default)]
pub struct SXfer;

impl CodeModel for SXfer {
    fn name(&self) -> &str {
        "s_xfer"
    }

    fn description(&self) -> &str {
        "S-domain transfer function"
    }

    fn ports(&self) -> &[PortSpec] {
        transfer_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        s_xfer_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let Some(coefficients) = cache_s_xfer_coefficients(ctx)? else {
            ctx.allocate_states(0);
            return Ok(());
        };

        let order = coefficients.denominator.len().saturating_sub(1);
        ctx.allocate_states(if order == 0 { 0 } else { order + 1 });
        if order > 0 {
            ensure_s_xfer_transient_scratch(ctx);
        }
        let int_ic = finite_s_xfer_vector_param(ctx, "int_ic")?;
        for index in 0..order {
            let source_index = order - 1 - index;
            let value = int_ic.get(source_index).copied().unwrap_or(0.0);
            ctx.set_initial_state(index, value);
        }
        if order > 0 {
            ctx.set_initial_state(order, 0.0);
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        if ctx.analysis == AnalysisType::Ac {
            return Ok(());
        }
        let Some(coefficients) = s_xfer_coefficients_for_context(ctx)? else {
            ctx.set_output_with_partial("out", 0.0, 0.0);
            return Ok(());
        };

        let (output, partial) = s_xfer_transient_eval(ctx, &coefficients)?;
        ctx.set_output_with_partial("out", output, partial);
        Ok(())
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        vec![s_xfer_dc_gain(ctx)]
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![(
                "in".to_string(),
                s_xfer_input_partial_for_context(ctx).unwrap_or(0.0),
            )]
        } else {
            Vec::new()
        }
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }

    fn output_input_ac_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
        frequency: Value,
    ) -> Vec<(String, Complex64)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        match s_xfer_coefficients_for_context(ctx) {
            Ok(Some(coefficients)) => {
                vec![("in".to_string(), s_xfer_gain_at(&coefficients, frequency))]
            }
            Err(_) => Vec::new(),
            Ok(None) => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::ParamType;

    fn port_summary(
        model: &dyn CodeModel,
    ) -> Vec<(
        &str,
        PortDirection,
        PortType,
        Vec<PortType>,
        bool,
        bool,
        Option<usize>,
        Option<usize>,
    )> {
        model
            .ports()
            .iter()
            .map(|port| {
                (
                    port.name.as_str(),
                    port.direction,
                    port.default_type,
                    port.allowed_types.clone(),
                    port.is_vector,
                    port.null_allowed,
                    port.vector_min_len,
                    port.vector_max_len,
                )
            })
            .collect()
    }

    fn param_summary(
        model: &dyn CodeModel,
    ) -> Vec<(
        &str,
        ParamType,
        Value,
        Option<&str>,
        bool,
        Option<Value>,
        Option<Value>,
        Option<usize>,
        Option<usize>,
        Option<Vec<Value>>,
    )> {
        model
            .parameters()
            .iter()
            .map(|param| {
                (
                    param.name.as_str(),
                    param.param_type,
                    param.default,
                    param.string_default.as_deref(),
                    param.required,
                    param.min,
                    param.max,
                    param.vector_min_len,
                    param.vector_max_len,
                    param.real_vector_default.clone(),
                )
            })
            .collect()
    }

    fn transfer_port_summary() -> Vec<(
        &'static str,
        PortDirection,
        PortType,
        Vec<PortType>,
        bool,
        bool,
        Option<usize>,
        Option<usize>,
    )> {
        let analog_types = vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
        ];
        vec![
            (
                "in",
                PortDirection::In,
                PortType::Voltage,
                analog_types.clone(),
                false,
                false,
                None,
                None,
            ),
            (
                "out",
                PortDirection::Out,
                PortType::Voltage,
                analog_types,
                false,
                false,
                None,
                None,
            ),
        ]
    }

    #[test]
    fn transfer_metadata_matches_ngspice46_interfaces() {
        assert_eq!(port_summary(&Xfer), transfer_port_summary());
        assert_eq!(
            param_summary(&Xfer),
            vec![
                (
                    "table",
                    ParamType::RealVector,
                    0.0,
                    None,
                    false,
                    None,
                    None,
                    Some(3),
                    None,
                    Some(vec![0.0, 0.0, 0.0]),
                ),
                (
                    "file",
                    ParamType::String,
                    0.0,
                    Some(""),
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                (
                    "r_i",
                    ParamType::Boolean,
                    0.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                (
                    "db",
                    ParamType::Boolean,
                    1.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                (
                    "rad",
                    ParamType::Boolean,
                    0.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                (
                    "span",
                    ParamType::Integer,
                    3.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                (
                    "offset",
                    ParamType::Integer,
                    1.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
            ]
        );

        assert_eq!(port_summary(&SXfer), transfer_port_summary());
        assert_eq!(
            param_summary(&SXfer),
            vec![
                (
                    "in_offset",
                    ParamType::Real,
                    0.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                (
                    "gain",
                    ParamType::Real,
                    1.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
                (
                    "num_coeff",
                    ParamType::RealVector,
                    0.0,
                    None,
                    true,
                    None,
                    None,
                    Some(1),
                    None,
                    Some(Vec::new()),
                ),
                (
                    "den_coeff",
                    ParamType::RealVector,
                    0.0,
                    None,
                    true,
                    None,
                    None,
                    Some(1),
                    None,
                    Some(Vec::new()),
                ),
                (
                    "int_ic",
                    ParamType::RealVector,
                    0.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    Some(Vec::new()),
                ),
                (
                    "denormalized_freq",
                    ParamType::Real,
                    1.0,
                    None,
                    false,
                    None,
                    None,
                    None,
                    None,
                    None,
                ),
            ]
        );
    }

    fn xfer_point(frequency: Value, gain: Value) -> XferPoint {
        XferPoint {
            frequency,
            gain: Complex64::new(gain, 0.0),
        }
    }

    fn xfer_table_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_real_vector_param("table", vec![1.0, 20.0, 0.0]);
        ctx.set_param("db", 1.0);
        ctx.set_param("rad", 0.0);
        ctx.set_param("r_i", 0.0);
        ctx.set_param("span", 3.0);
        ctx.set_param("offset", 1.0);
        ctx.set_input_analog("in", 1.0);
        ctx.init_output("out", PortType::Voltage);
        ctx
    }

    fn first_order_lowpass_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("gain", 1.0);
        ctx.set_real_vector_param("num_coeff", vec![1.0]);
        ctx.set_real_vector_param("den_coeff", vec![1.0, 1.0]);
        ctx.set_real_vector_param("int_ic", vec![0.0]);
        ctx.set_param("denormalized_freq", 1.0);
        ctx.set_input_analog("in", 1.0);
        ctx.time_prev = 0.0;
        ctx.time = 0.25;
        ctx.timestep = 0.25;
        ctx.init_output("out", PortType::Voltage);
        ctx
    }

    #[test]
    fn s_xfer_disables_improper_transfer_order_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_real_vector_param("num_coeff", vec![1.0, 0.0]);
        ctx.set_real_vector_param("den_coeff", vec![1.0]);

        SXfer
            .init(&mut ctx)
            .expect("ngspice-compatible improper s_xfer disables the instance");
        assert!(
            s_xfer_coefficients_for_context(&ctx)
                .expect("disabled coefficient lookup remains valid")
                .is_none()
        );
    }

    #[test]
    fn s_xfer_rejects_nonfinite_transient_params() {
        let mut ctx = first_order_lowpass_context();
        ctx.set_real_vector_param("int_ic", vec![f64::NAN]);
        let err = SXfer
            .init(&mut ctx)
            .expect_err("s_xfer must reject nonfinite initial conditions");
        assert!(
            matches!(&err, CmError::InvalidParameter { .. }),
            "s_xfer int_ic produced {err:?}"
        );
        assert!(
            err.to_string().contains("int_ic"),
            "s_xfer error should name rejected int_ic: {err}"
        );

        let mut ctx = first_order_lowpass_context();
        SXfer.init(&mut ctx).expect("s_xfer initializes");
        ctx.set_param("in_offset", f64::INFINITY);
        let err = SXfer
            .evaluate(&mut ctx)
            .expect_err("s_xfer must reject nonfinite input offsets");
        assert!(
            matches!(&err, CmError::InvalidParameter { .. }),
            "s_xfer in_offset produced {err:?}"
        );
        assert!(
            err.to_string().contains("in_offset"),
            "s_xfer error should name rejected in_offset: {err}"
        );
    }

    #[test]
    fn s_xfer_rollbackable_probe_does_not_commit_filter_state() {
        let mut ctx = first_order_lowpass_context();
        SXfer.init(&mut ctx).expect("s_xfer initializes");

        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        SXfer.evaluate(&mut ctx).expect("probes s_xfer transient");
        assert!(
            ctx.output("out") > 0.0,
            "rollbackable s_xfer probe should still compute trial output"
        );
        assert_eq!(
            ctx.state(0),
            0.0,
            "rollbackable s_xfer probe must not commit filter state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        SXfer.evaluate(&mut ctx).expect("commits s_xfer transient");
        assert!(
            ctx.state(0) > 0.0,
            "accepted s_xfer evaluation should commit filter state"
        );
    }

    #[test]
    fn s_xfer_transient_state_scratch_overwrites_and_reuses_buffer() {
        let mut ctx = first_order_lowpass_context();
        SXfer.init(&mut ctx).expect("s_xfer initializes");
        {
            let state = ctx
                .resource_mut::<SXferTransientScratchResource>(SXFER_TRANSIENT_SCRATCH_RESOURCE)
                .expect("s_xfer installs transient scratch for dynamic systems");
            state.resize(16, Value::NAN);
        }

        SXfer.evaluate(&mut ctx).expect("s_xfer overwrites scratch");
        assert!(ctx.output("out").is_finite());
        assert!(ctx.state(0).is_finite());

        let scratch = ctx
            .resource::<SXferTransientScratchResource>(SXFER_TRANSIENT_SCRATCH_RESOURCE)
            .expect("s_xfer transient scratch remains installed");
        let (state_ptr, state_capacity) = {
            let state = scratch.as_ref();
            assert_eq!(state.len(), 2);
            assert!(state[0].is_finite());
            assert!(state[1].is_finite());
            (state.as_ptr(), state.capacity())
        };
        drop(scratch);

        ctx.set_input_analog("in", 0.5);
        SXfer.evaluate(&mut ctx).expect("s_xfer reuses scratch");
        let scratch = ctx
            .resource::<SXferTransientScratchResource>(SXFER_TRANSIENT_SCRATCH_RESOURCE)
            .expect("s_xfer transient scratch remains installed");
        let state = scratch.as_ref();
        assert_eq!(state.as_ptr(), state_ptr);
        assert_eq!(state.capacity(), state_capacity);
        assert_eq!(state.len(), 2);
        assert!(state[0].is_finite());
        assert!(state[1].is_finite());
    }

    #[test]
    fn s_xfer_nonpositive_timestep_keeps_previous_state_on_commit() {
        let mut ctx = first_order_lowpass_context();
        ctx.set_real_vector_param("int_ic", vec![0.5]);
        ctx.timestep = 0.0;
        SXfer.init(&mut ctx).expect("s_xfer initializes");
        ctx.set_state(0, 9.0);

        SXfer
            .evaluate(&mut ctx)
            .expect("s_xfer handles zero timestep fallback");

        assert_eq!(
            ctx.state(0),
            0.5,
            "accepted zero-timestep fallback should restore the previous integrator state"
        );
        assert!(ctx.output("out").is_finite());
    }

    #[test]
    fn s_xfer_partials_compute_from_context_without_prior_evaluate() {
        let mut ctx = first_order_lowpass_context();
        SXfer.init(&mut ctx).expect("s_xfer initializes");

        assert_eq!(
            ctx.partial("out"),
            0.0,
            "test must start without an evaluated output partial"
        );

        let partials = SXfer.output_input_partials(&ctx, "out");
        assert_eq!(partials.len(), 1);
        assert_eq!(partials[0].0, "in");
        assert!(
            (partials[0].1 - 1.0).abs() < 1.0e-12,
            "s_xfer should compute ngspice's pseudo-integrator partial, got {partials:?}"
        );
    }

    #[test]
    fn s_xfer_partials_recompute_ngspice_pseudo_integrator_without_mutating_output() {
        let mut ctx = first_order_lowpass_context();
        SXfer.init(&mut ctx).expect("s_xfer initializes");
        SXfer
            .evaluate(&mut ctx)
            .expect("s_xfer evaluates at original step");
        assert!(
            (ctx.partial("out") - 1.0).abs() < 1.0e-12,
            "baseline partial should match ngspice's pseudo-integrator partial"
        );

        ctx.timestep = 0.5;
        let partials = SXfer.output_input_partials(&ctx, "out");

        assert_eq!(partials.len(), 1);
        assert_eq!(partials[0].0, "in");
        assert!(
            (partials[0].1 - 1.0).abs() < 1.0e-12,
            "s_xfer should recompute the pseudo-integrator partial instead of reusing stale output storage, got {partials:?}"
        );
        assert!(
            (ctx.partial("out") - 1.0).abs() < 1.0e-12,
            "partial lookup should not mutate the stored output partial"
        );
    }

    #[test]
    fn s_xfer_transient_chain_matches_ngspice_controller_canonical_update() {
        let mut ctx = CmContext::new();
        ctx.analysis = AnalysisType::Transient;
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("gain", 1.0);
        ctx.set_real_vector_param("num_coeff", vec![1.0, 0.0]);
        ctx.set_real_vector_param("den_coeff", vec![1.0, 1.0, 1.0]);
        ctx.set_real_vector_param("int_ic", vec![0.2, 0.4]);
        ctx.set_param("denormalized_freq", 1.0);
        ctx.set_input_analog("in", 1.0);
        ctx.timestep = 0.5;
        ctx.init_output("out", PortType::Voltage);

        SXfer.init(&mut ctx).expect("s_xfer initializes");
        SXfer
            .evaluate(&mut ctx)
            .expect("s_xfer evaluates controller-canonical chain");

        assert!((ctx.state(2) - 0.4).abs() < 1.0e-12);
        assert!((ctx.state(1) - 0.4).abs() < 1.0e-12);
        assert!((ctx.state(0) - 0.6).abs() < 1.0e-12);
        assert!((ctx.output("out") - 0.4).abs() < 1.0e-12);
        assert!((ctx.partial("out") - 0.4).abs() < 1.0e-12);
    }

    #[test]
    fn xfer_cached_table_reloads_when_format_param_changes() {
        let mut ctx = xfer_table_context();

        Xfer.init(&mut ctx).expect("xfer initializes");
        let first_table = xfer_table(&ctx).expect("cached xfer table");
        assert_eq!(xfer_first_real_gain(&ctx), 10.0);

        let after_mutable_cache =
            cache_xfer_table(&mut ctx).expect("xfer table after mutable cache refresh");
        assert!(
            Arc::ptr_eq(&first_table, &after_mutable_cache),
            "unchanged xfer parameters should reuse the context table resource"
        );

        ctx.set_real_vector_param("unrelated", vec![1.0, 2.0]);
        let after_unrelated =
            cache_xfer_table(&mut ctx).expect("xfer table after unrelated vector");
        assert!(Arc::ptr_eq(&first_table, &after_unrelated));

        ctx.set_param("db", 0.0);
        assert_eq!(xfer_first_real_gain(&ctx), 20.0);
    }

    #[test]
    fn xfer_rejects_nonfinite_table_format_params() {
        for (param, value) in [
            ("r_i", f64::NAN),
            ("db", f64::INFINITY),
            ("rad", f64::NEG_INFINITY),
            ("span", f64::NAN),
            ("offset", f64::INFINITY),
        ] {
            let mut ctx = xfer_table_context();
            ctx.set_param(param, value);

            let err = Xfer
                .init(&mut ctx)
                .expect_err("xfer must reject nonfinite table format parameters");
            assert!(
                matches!(&err, CmError::InvalidParameter { .. }),
                "xfer {param} produced {err:?}"
            );
            assert!(
                err.to_string().contains(param),
                "xfer error should name rejected parameter {param}: {err}"
            );
        }

        let mut ctx = xfer_table_context();
        Xfer.init(&mut ctx).expect("xfer initializes");
        ctx.set_param("db", f64::NAN);
        Xfer.evaluate(&mut ctx)
            .expect_err("mutated nonfinite xfer format params must fail during evaluation");
    }

    #[test]
    fn xfer_rejects_db_table_values_that_overflow_linear_gain() {
        let mut ctx = xfer_table_context();
        ctx.set_real_vector_param("table", vec![1.0, 7000.0, 0.0]);

        let err = Xfer
            .init(&mut ctx)
            .expect_err("xfer must reject dB values that overflow during conversion");

        assert!(
            matches!(&err, CmError::InvalidParameter { .. }),
            "xfer overflow produced {err:?}"
        );
        assert!(
            err.to_string().contains("finite"),
            "xfer overflow error should explain the finite-gain requirement: {err}"
        );
    }

    #[test]
    fn xfer_cached_touchstone_reloads_when_virtual_file_changes() {
        let file = "virtual://xfer/cache-reloads-on-replace";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(file, "# Hz S RI R 50\n1 1 0\n")
            .expect("register first virtual xfer data");

        let mut ctx = CmContext::new();
        ctx.set_string_param("file", file);
        Xfer.init(&mut ctx).expect("xfer initializes");
        assert_eq!(xfer_first_real_gain(&ctx), 1.0);

        data_file::register_data_file(file, "# Hz S RI R 50\n1 5 0\n")
            .expect("replace virtual xfer data");
        assert_eq!(xfer_first_real_gain(&ctx), 5.0);

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn xfer_rejects_nonfinite_touchstone_format_overrides() {
        let file = "virtual://xfer/nonfinite-touchstone-format-overrides";
        let _ = data_file::unregister_data_file(file);
        data_file::register_data_file(file, "# Hz S RI R 50\n1 1 0\n")
            .expect("register virtual xfer data");

        for (param, value) in [
            ("r_i", f64::NAN),
            ("db", f64::INFINITY),
            ("rad", f64::NEG_INFINITY),
        ] {
            let mut ctx = CmContext::new();
            ctx.set_string_param("file", file);
            ctx.set_param(param, value);
            ctx.mark_param_provided(param);

            let err = Xfer
                .init(&mut ctx)
                .expect_err("file-backed xfer must reject nonfinite format overrides");
            assert!(
                matches!(&err, CmError::InvalidParameter { .. }),
                "xfer {param} produced {err:?}"
            );
            assert!(
                err.to_string().contains(param),
                "xfer error should name rejected parameter {param}: {err}"
            );
        }

        let _ = data_file::unregister_data_file(file);
    }

    #[test]
    fn xfer_gain_uses_monotonic_table_brackets() {
        let table = xfer_table_data(vec![
            xfer_point(1.0, 10.0),
            xfer_point(2.0, 20.0),
            xfer_point(4.0, 40.0),
        ]);

        assert!(table.strictly_increasing_frequency);
        assert_eq!(
            table.last_upper_index.load(Ordering::Relaxed),
            XFER_UNSET_UPPER_INDEX
        );
        assert_eq!(
            xfer_gain_at(&table, 2.0),
            Complex64::new(20.0, 0.0),
            "exact table frequencies should return the matching row"
        );
        assert_eq!(table.last_upper_index.load(Ordering::Relaxed), 1);
        assert_eq!(
            xfer_gain_at(&table, 3.0),
            Complex64::new(30.0, 0.0),
            "monotonic tables should interpolate between the binary-search bracket"
        );
        assert_eq!(table.last_upper_index.load(Ordering::Relaxed), 2);
    }

    #[test]
    fn xfer_gain_cursor_falls_back_for_large_frequency_jumps() {
        let table = xfer_table_data(
            (1..=24)
                .map(|frequency| xfer_point(frequency as Value, frequency as Value))
                .collect(),
        );

        assert_eq!(xfer_gain_at(&table, 2.5), Complex64::new(2.5, 0.0));
        assert_eq!(table.last_upper_index.load(Ordering::Relaxed), 2);
        assert_eq!(xfer_gain_at(&table, 22.5), Complex64::new(22.5, 0.0));
        assert_eq!(
            table.last_upper_index.load(Ordering::Relaxed),
            22,
            "large non-local jumps should land on the binary-search bracket"
        );
    }

    #[test]
    fn xfer_gain_preserves_linear_scan_for_duplicate_or_unordered_tables() {
        let duplicate = xfer_table_data(vec![
            xfer_point(1.0, 10.0),
            xfer_point(2.0, 20.0),
            xfer_point(2.0, 70.0),
            xfer_point(3.0, 30.0),
        ]);
        let unordered = xfer_table_data(vec![
            xfer_point(1.0, 10.0),
            xfer_point(3.0, 30.0),
            xfer_point(2.0, 200.0),
        ]);

        assert!(!duplicate.strictly_increasing_frequency);
        assert_eq!(
            xfer_gain_at(&duplicate, 2.0),
            Complex64::new(20.0, 0.0),
            "duplicate row frequencies should keep the original first-crossing behavior"
        );
        assert!(!unordered.strictly_increasing_frequency);
        assert_eq!(
            xfer_gain_at(&unordered, 2.5),
            Complex64::new(200.0, 0.0),
            "unordered file-compatible tables should keep the original endpoint clamp"
        );
    }

    #[test]
    fn s_xfer_cached_coefficients_reload_when_gain_changes() {
        let mut ctx = CmContext::new();
        ctx.set_param("gain", 2.0);
        ctx.set_param("denormalized_freq", 1.0);
        ctx.set_real_vector_param("num_coeff", vec![1.0]);
        ctx.set_real_vector_param("den_coeff", vec![1.0]);

        SXfer.init(&mut ctx).expect("s_xfer initializes");
        let first = s_xfer_coefficients_for_context(&ctx)
            .expect("cached coefficients")
            .expect("proper s_xfer order");
        assert_eq!(s_xfer_dc_gain(&ctx), 2.0);

        let after_mutable_cache = cache_s_xfer_coefficients(&mut ctx)
            .expect("mutable coefficient cache reuses")
            .expect("proper s_xfer order");
        assert!(
            Arc::ptr_eq(&first, &after_mutable_cache),
            "unchanged s_xfer parameters should reuse the context coefficient resource"
        );

        ctx.set_real_vector_param("unrelated", vec![1.0, 2.0]);
        let after_unrelated_mutable = cache_s_xfer_coefficients(&mut ctx)
            .expect("mutable coefficients after unrelated vector")
            .expect("proper s_xfer order");
        assert!(Arc::ptr_eq(&first, &after_unrelated_mutable));
        let after_unrelated = s_xfer_coefficients_for_context(&ctx)
            .expect("coefficients after unrelated vector")
            .expect("proper s_xfer order");
        assert!(Arc::ptr_eq(&first, &after_unrelated));

        ctx.set_param("gain", 4.0);
        let updated = s_xfer_coefficients_for_context(&ctx)
            .expect("updated coefficients")
            .expect("proper s_xfer order");
        assert!(!Arc::ptr_eq(&first, &updated));
        assert_eq!(s_xfer_dc_gain(&ctx), 4.0);
    }
}

//! Multi-input analog XSPICE code models.

use crate::Value;
use crate::xspice::context::AnalogValue;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
};
use std::sync::{
    Arc, OnceLock,
    atomic::{AtomicUsize, Ordering},
};

const TABLE_RESOURCE: &str = "xspice.multi_input_pwl.table";
const EVAL_RESOURCE: &str = "xspice.multi_input_pwl.eval";
const TABLE_UNSET_UPPER_INDEX: usize = usize::MAX;
const TABLE_CURSOR_LINEAR_STEPS: usize = 8;

#[derive(Debug, Default)]
pub struct MultiInputPwl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MultiInputPwlMode {
    And,
    Nand,
    Or,
    Nor,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct TableEval {
    value: Value,
    slope: Value,
}

#[derive(Debug, Clone, Copy)]
struct TablePoint {
    x: Value,
    y: Value,
}

#[derive(Debug)]
struct TableData {
    points: Vec<TablePoint>,
    strictly_increasing_x: bool,
    last_upper_index: AtomicUsize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct TableSignature {
    x_revision: Option<u64>,
    y_revision: Option<u64>,
}

#[derive(Debug, Clone)]
struct TableResource {
    signature: TableSignature,
    data: CmResult<Arc<TableData>>,
}

#[derive(Debug, Clone, PartialEq)]
struct EvalSignature {
    table: TableSignature,
    mode: MultiInputPwlMode,
    inputs: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct EvalBaseSignature {
    table: TableSignature,
    mode: MultiInputPwlMode,
}

#[derive(Debug, Clone)]
struct EvalResource {
    signature: EvalSignature,
    result: Option<(usize, TableEval)>,
}

fn invalid_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage, PortType::DifferentialCurrent],
                is_vector: true,
                null_allowed: false,
                vector_min_len: Some(2),
                vector_max_len: None,
                description: "Analog input vector".to_string(),
            },
            PortSpec {
                name: "out".to_string(),
                direction: PortDirection::Out,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![PortType::DifferentialVoltage, PortType::DifferentialCurrent],
                is_vector: false,
                null_allowed: false,
                vector_min_len: None,
                vector_max_len: None,
                description: "Analog output".to_string(),
            },
        ]
    })
}

fn parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("x", Vec::new())
                .required()
                .with_vector_min_len(2)
                .with_description("Input lookup points"),
            ParamSpec::real_vector("y", Vec::new())
                .required()
                .with_vector_min_len(2)
                .with_description("Output lookup points"),
            ParamSpec::string("model", "and").with_description("and, nand, or, or nor"),
        ]
    })
}

fn mode(ctx: &CmContext) -> CmResult<MultiInputPwlMode> {
    let raw = ctx.string_param("model").unwrap_or("and");
    if raw.eq_ignore_ascii_case("and") {
        Ok(MultiInputPwlMode::And)
    } else if raw.eq_ignore_ascii_case("nand") {
        Ok(MultiInputPwlMode::Nand)
    } else if raw.eq_ignore_ascii_case("or") {
        Ok(MultiInputPwlMode::Or)
    } else if raw.eq_ignore_ascii_case("nor") {
        Ok(MultiInputPwlMode::Nor)
    } else {
        Err(invalid_param(
            "model",
            format!("unknown model '{raw}', expected and|nand|or|nor"),
        ))
    }
}

fn table_signature(ctx: &CmContext) -> TableSignature {
    TableSignature {
        x_revision: ctx.real_vector_param_revision("x"),
        y_revision: ctx.real_vector_param_revision("y"),
    }
}

fn table_signature_matches(ctx: &CmContext, signature: &TableSignature) -> bool {
    table_signature(ctx) == *signature
}

fn table_uncached(ctx: &CmContext) -> CmResult<TableData> {
    let x = ctx
        .real_vector_param("x")
        .ok_or_else(|| CmError::MissingParameter("x".to_string()))?;
    let y = ctx
        .real_vector_param("y")
        .ok_or_else(|| CmError::MissingParameter("y".to_string()))?;
    let effective_len = x.len().min(y.len());
    if effective_len < 2 {
        return Err(invalid_param(
            "x/y",
            format!("x and y require at least 2 common points, got {effective_len}"),
        ));
    }
    let x = &x[..effective_len];
    let y = &y[..effective_len];

    let mut points = Vec::with_capacity(effective_len);
    for (idx, (&x_value, &y_value)) in x.iter().zip(y).enumerate() {
        if !x_value.is_finite() {
            return Err(invalid_param(
                "x",
                format!("point {idx} must be finite, got {x_value}"),
            ));
        }
        if !y_value.is_finite() {
            return Err(invalid_param(
                "y",
                format!("point {idx} must be finite, got {y_value}"),
            ));
        }
        points.push(TablePoint {
            x: x_value,
            y: y_value,
        });
    }

    Ok(table_data(points))
}

fn table_data(points: Vec<TablePoint>) -> TableData {
    let strictly_increasing_x = points.windows(2).all(|window| window[0].x < window[1].x);
    TableData {
        points,
        strictly_increasing_x,
        last_upper_index: AtomicUsize::new(TABLE_UNSET_UPPER_INDEX),
    }
}

fn cache_table(ctx: &mut CmContext) -> CmResult<Arc<TableData>> {
    if let Some(resource) = ctx.resource::<TableResource>(TABLE_RESOURCE)
        && table_signature_matches(ctx, &resource.signature)
    {
        return resource.data.clone();
    }

    let signature = table_signature(ctx);
    let data = table_uncached(ctx).map(Arc::new);
    ctx.set_resource(
        TABLE_RESOURCE,
        Arc::new(TableResource {
            signature,
            data: data.clone(),
        }),
    );
    data
}

fn table(ctx: &CmContext) -> CmResult<Arc<TableData>> {
    if let Some(resource) = ctx.resource::<TableResource>(TABLE_RESOURCE)
        && table_signature_matches(ctx, &resource.signature)
    {
        return resource.data.clone();
    }

    table_uncached(ctx).map(Arc::new)
}

fn input_values(ctx: &CmContext) -> &[AnalogValue] {
    ctx.input_analog_vector_values("in").unwrap_or(&[])
}

fn collect_input_values(inputs: &[AnalogValue]) -> Vec<Value> {
    inputs.iter().map(|input| input.value).collect()
}

fn eval_base_signature(ctx: &CmContext) -> CmResult<EvalBaseSignature> {
    Ok(EvalBaseSignature {
        table: table_signature(ctx),
        mode: mode(ctx)?,
    })
}

fn eval_signature_from_inputs(base: EvalBaseSignature, inputs: &[AnalogValue]) -> EvalSignature {
    EvalSignature {
        table: base.table,
        mode: base.mode,
        inputs: collect_input_values(inputs),
    }
}

fn eval_signature(ctx: &CmContext) -> CmResult<EvalSignature> {
    Ok(eval_signature_from_inputs(
        eval_base_signature(ctx)?,
        input_values(ctx),
    ))
}

fn eval_inputs_match(cached: &[Value], inputs: &[AnalogValue]) -> bool {
    cached.len() == inputs.len()
        && cached
            .iter()
            .zip(inputs)
            .all(|(cached, input)| *cached == input.value)
}

fn eval_resource_matches(
    resource: &EvalResource,
    base: EvalBaseSignature,
    inputs: &[AnalogValue],
) -> bool {
    resource.signature.table == base.table
        && resource.signature.mode == base.mode
        && eval_inputs_match(&resource.signature.inputs, inputs)
}

fn controlling_input<I>(inputs: I, mode: MultiInputPwlMode) -> Option<(usize, Value)>
where
    I: IntoIterator<Item = (usize, Value)>,
{
    match mode {
        MultiInputPwlMode::And | MultiInputPwlMode::Nand => inputs
            .into_iter()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        MultiInputPwlMode::Or | MultiInputPwlMode::Nor => inputs
            .into_iter()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
    }
}

fn forward_output(table: &[TablePoint], input: Value) -> TableEval {
    let last = table.len() - 1;
    if input <= table[0].x {
        return TableEval {
            value: table[0].y,
            slope: 0.0,
        };
    }
    if input >= table[last].x {
        return TableEval {
            value: table[last].y,
            slope: 0.0,
        };
    }

    for idx in 1..table.len() {
        if input > table[idx - 1].x && input <= table[idx].x {
            let slope = (table[idx].y - table[idx - 1].y) / (table[idx].x - table[idx - 1].x);
            return TableEval {
                value: table[idx].y + slope * (input - table[idx].x),
                slope,
            };
        }
    }

    TableEval {
        value: table[last].y,
        slope: 0.0,
    }
}

fn table_interval_contains(points: &[TablePoint], upper: usize, input: Value) -> bool {
    debug_assert!(upper > 0);
    debug_assert!(upper < points.len());
    points[upper - 1].x < input && input <= points[upper].x
}

fn table_upper_index_binary(points: &[TablePoint], input: Value) -> usize {
    points.partition_point(|point| point.x < input)
}

fn table_upper_index_with_cursor(table: &TableData, input: Value) -> usize {
    let points = table.points.as_slice();
    let point_count = points.len();
    let mut upper = table.last_upper_index.load(Ordering::Relaxed);

    if upper == TABLE_UNSET_UPPER_INDEX || upper == 0 || upper >= point_count {
        upper = table_upper_index_binary(points, input);
        table.last_upper_index.store(upper, Ordering::Relaxed);
        return upper;
    }

    if table_interval_contains(points, upper, input) {
        return upper;
    }

    let mut steps = 0;
    if input > points[upper].x {
        while upper + 1 < point_count
            && input > points[upper].x
            && steps < TABLE_CURSOR_LINEAR_STEPS
        {
            upper += 1;
            steps += 1;
        }
    } else {
        while upper > 1 && input <= points[upper - 1].x && steps < TABLE_CURSOR_LINEAR_STEPS {
            upper -= 1;
            steps += 1;
        }
    }

    if !table_interval_contains(points, upper, input) {
        upper = table_upper_index_binary(points, input);
    }
    table.last_upper_index.store(upper, Ordering::Relaxed);
    upper
}

fn forward_output_increasing(table: &TableData, input: Value) -> TableEval {
    let points = table.points.as_slice();
    let last = points.len() - 1;
    if input <= points[0].x {
        return TableEval {
            value: points[0].y,
            slope: 0.0,
        };
    }
    if input >= points[last].x {
        return TableEval {
            value: points[last].y,
            slope: 0.0,
        };
    }

    let upper = table_upper_index_with_cursor(table, input);
    let lower = upper - 1;
    let slope = (points[upper].y - points[lower].y) / (points[upper].x - points[lower].x);
    TableEval {
        value: points[upper].y + slope * (input - points[upper].x),
        slope,
    }
}

fn reverse_output(table: &[TablePoint], input: Value) -> TableEval {
    let last = table.len() - 1;
    if input <= table[0].x {
        return TableEval {
            value: table[last].y,
            slope: 0.0,
        };
    }
    if input >= table[last].x {
        return TableEval {
            value: table[0].y,
            slope: 0.0,
        };
    }

    for idx in 1..table.len() {
        if input > table[idx - 1].x && input <= table[idx].x {
            return TableEval {
                value: table[last - idx].y,
                slope: 0.0,
            };
        }
    }

    TableEval {
        value: table[0].y,
        slope: 0.0,
    }
}

fn reverse_output_increasing(table: &TableData, input: Value) -> TableEval {
    let points = table.points.as_slice();
    let last = points.len() - 1;
    if input <= points[0].x {
        return TableEval {
            value: points[last].y,
            slope: 0.0,
        };
    }
    if input >= points[last].x {
        return TableEval {
            value: points[0].y,
            slope: 0.0,
        };
    }

    let upper = table_upper_index_with_cursor(table, input);
    TableEval {
        value: points[last - upper].y,
        slope: 0.0,
    }
}

fn evaluate_multi_input_from_values<I>(
    table: &TableData,
    mode: MultiInputPwlMode,
    input_len: usize,
    inputs: I,
) -> CmResult<Option<(usize, TableEval)>>
where
    I: IntoIterator<Item = (usize, Value)>,
{
    if input_len < 2 {
        return Err(CmError::PortCountMismatch {
            expected: 2,
            actual: input_len,
        });
    }

    let Some((index, controlling_input)) = controlling_input(inputs, mode) else {
        return Ok(None);
    };

    let result = match (mode, table.strictly_increasing_x) {
        (MultiInputPwlMode::And | MultiInputPwlMode::Or, true) => {
            forward_output_increasing(table, controlling_input)
        }
        (MultiInputPwlMode::And | MultiInputPwlMode::Or, false) => {
            let points = table.points.as_slice();
            forward_output(points, controlling_input)
        }
        (MultiInputPwlMode::Nand | MultiInputPwlMode::Nor, true) => {
            reverse_output_increasing(table, controlling_input)
        }
        (MultiInputPwlMode::Nand | MultiInputPwlMode::Nor, false) => {
            let points = table.points.as_slice();
            reverse_output(points, controlling_input)
        }
    };

    Ok(Some((index, result)))
}

fn evaluate_multi_input_with_signature(
    table: &TableData,
    signature: &EvalSignature,
) -> CmResult<Option<(usize, TableEval)>> {
    evaluate_multi_input_from_values(
        table,
        signature.mode,
        signature.inputs.len(),
        signature.inputs.iter().copied().enumerate(),
    )
}

fn evaluate_multi_input_with_inputs(
    table: &TableData,
    mode: MultiInputPwlMode,
    inputs: &[AnalogValue],
) -> CmResult<Option<(usize, TableEval)>> {
    evaluate_multi_input_from_values(
        table,
        mode,
        inputs.len(),
        inputs.iter().map(|input| input.value).enumerate(),
    )
}

fn evaluate_multi_input(ctx: &CmContext) -> CmResult<Option<(usize, TableEval)>> {
    let base = eval_base_signature(ctx)?;
    let inputs = input_values(ctx);
    if let Some(resource) = ctx.resource::<EvalResource>(EVAL_RESOURCE)
        && eval_resource_matches(&resource, base, inputs)
    {
        return Ok(resource.result);
    }

    let table = table(ctx)?;
    evaluate_multi_input_with_inputs(&table, base.mode, inputs)
}

fn evaluate_multi_input_cached(ctx: &mut CmContext) -> CmResult<Option<(usize, TableEval)>> {
    let base = eval_base_signature(ctx)?;
    if let Some(resource) = ctx.resource::<EvalResource>(EVAL_RESOURCE)
        && eval_resource_matches(&resource, base, input_values(ctx))
    {
        return Ok(resource.result);
    }

    let signature = eval_signature_from_inputs(base, input_values(ctx));
    let table = cache_table(ctx)?;
    let result = evaluate_multi_input_with_signature(&table, &signature)?;
    ctx.set_resource(EVAL_RESOURCE, Arc::new(EvalResource { signature, result }));
    Ok(result)
}

impl CodeModel for MultiInputPwl {
    fn name(&self) -> &str {
        "multi_input_pwl"
    }

    fn description(&self) -> &str {
        "Multi-input piecewise-linear analog gate"
    }

    fn ports(&self) -> &[PortSpec] {
        ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        cache_table(ctx)?;
        mode(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let output = evaluate_multi_input_cached(ctx)?.map_or(0.0, |(_, result)| result.value);
        ctx.set_output_with_partial("out", output, 0.0);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }

    fn output_input_vector_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
    ) -> Vec<(String, usize, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }
        match evaluate_multi_input(ctx) {
            Ok(Some((index, result))) if result.slope.is_finite() && result.slope != 0.0 => {
                vec![("in".to_string(), index, result.slope)]
            }
            _ => Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::context::AnalogValue;

    fn set_inputs(ctx: &mut CmContext, values: &[Value]) {
        ctx.set_input_analog_vector_from_fn("in", values.len(), |index| {
            AnalogValue::new(values[index])
        });
    }

    #[test]
    fn multi_input_table_cache_reloads_when_params_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("x", vec![0.0, 1.0, 2.0]);
        ctx.set_real_vector_param("y", vec![0.0, 10.0, 20.0]);

        let first = cache_table(&mut ctx).expect("table caches");
        let second = cache_table(&mut ctx).expect("table reuses cache");
        assert!(
            Arc::ptr_eq(&first, &second),
            "unchanged multi_input_pwl table parameters should reuse the parsed table"
        );
        assert_eq!(first.points[2].y, 20.0);

        ctx.set_real_vector_param("unrelated", vec![99.0]);
        let after_unrelated = cache_table(&mut ctx).expect("unrelated vector preserves cache");
        assert!(
            Arc::ptr_eq(&first, &after_unrelated),
            "unrelated vector parameters should not refresh the parsed table"
        );

        ctx.set_real_vector_param("y", vec![0.0, 10.0, 30.0]);
        let updated = cache_table(&mut ctx).expect("updated table caches");
        assert!(
            !Arc::ptr_eq(&first, &updated),
            "changed multi_input_pwl table parameters must refresh the parsed table"
        );
        assert_eq!(updated.points[2].y, 30.0);
    }

    #[test]
    fn multi_input_eval_cache_reuses_current_result_until_inputs_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("x", vec![0.0, 1.0, 2.0]);
        ctx.set_real_vector_param("y", vec![0.0, 10.0, 20.0]);
        ctx.set_string_param("model", "or");
        set_inputs(&mut ctx, &[0.5, 1.5, 1.0]);

        let initial = evaluate_multi_input_cached(&mut ctx)
            .expect("evaluation caches")
            .expect("controlling input is present");
        assert_eq!(
            initial,
            (
                1,
                TableEval {
                    value: 15.0,
                    slope: 10.0,
                }
            )
        );

        let signature = eval_signature(&ctx).expect("current signature");
        let sentinel = Some((
            2,
            TableEval {
                value: 123.0,
                slope: 456.0,
            },
        ));
        ctx.set_resource(
            EVAL_RESOURCE,
            Arc::new(EvalResource {
                signature,
                result: sentinel,
            }),
        );

        assert_eq!(
            evaluate_multi_input(&ctx).expect("read-only path reuses cache"),
            sentinel
        );

        set_inputs(&mut ctx, &[0.5, 0.75, 1.0]);
        assert_eq!(
            evaluate_multi_input(&ctx).expect("changed inputs invalidate cache"),
            Some((
                2,
                TableEval {
                    value: 10.0,
                    slope: 10.0,
                }
            ))
        );
    }

    #[test]
    fn multi_input_eval_resource_match_compares_current_input_slice() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("x", vec![0.0, 1.0, 2.0]);
        ctx.set_real_vector_param("y", vec![0.0, 10.0, 20.0]);
        ctx.set_string_param("model", "or");
        set_inputs(&mut ctx, &[0.5, 1.5, 1.0]);

        let signature = eval_signature(&ctx).expect("current signature");
        let base = eval_base_signature(&ctx).expect("current base signature");
        let resource = EvalResource {
            signature,
            result: Some((
                1,
                TableEval {
                    value: 15.0,
                    slope: 10.0,
                },
            )),
        };

        assert!(
            eval_resource_matches(&resource, base, input_values(&ctx)),
            "matching inputs should hit the eval cache without building a new signature"
        );

        set_inputs(&mut ctx, &[0.5, 0.75, 1.0]);
        assert!(
            !eval_resource_matches(&resource, base, input_values(&ctx)),
            "changed inputs must invalidate the eval cache"
        );
    }

    #[test]
    fn multi_input_eval_cache_invalidates_when_model_changes() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("x", vec![0.0, 1.0, 2.0]);
        ctx.set_real_vector_param("y", vec![0.0, 10.0, 20.0]);
        ctx.set_string_param("model", "and");
        set_inputs(&mut ctx, &[0.5, 1.5]);

        let initial = evaluate_multi_input_cached(&mut ctx)
            .expect("evaluation caches")
            .expect("and controlling input is present");
        assert_eq!(
            initial,
            (
                0,
                TableEval {
                    value: 5.0,
                    slope: 10.0,
                }
            )
        );

        let signature = eval_signature(&ctx).expect("current signature");
        let sentinel = Some((
            1,
            TableEval {
                value: 99.0,
                slope: 99.0,
            },
        ));
        ctx.set_resource(
            EVAL_RESOURCE,
            Arc::new(EvalResource {
                signature,
                result: sentinel,
            }),
        );

        assert_eq!(
            evaluate_multi_input(&ctx).expect("matching model reuses cache"),
            sentinel
        );

        ctx.set_string_param("model", "nand");
        assert_eq!(
            evaluate_multi_input(&ctx).expect("changed model invalidates cache"),
            Some((
                0,
                TableEval {
                    value: 10.0,
                    slope: 0.0,
                }
            ))
        );
    }

    #[test]
    fn multi_input_uses_cursor_lookup_for_strictly_increasing_tables() {
        let data = table_data(vec![
            TablePoint { x: 0.0, y: 0.0 },
            TablePoint { x: 1.0, y: 10.0 },
            TablePoint { x: 2.0, y: 30.0 },
        ]);

        assert!(data.strictly_increasing_x);
        assert_eq!(
            data.last_upper_index.load(Ordering::Relaxed),
            TABLE_UNSET_UPPER_INDEX
        );

        let forward = forward_output_increasing(&data, 1.5);
        assert_eq!(forward.value, 20.0);
        assert_eq!(forward.slope, 20.0);
        assert_eq!(data.last_upper_index.load(Ordering::Relaxed), 2);

        let exact = forward_output_increasing(&data, 1.0);
        assert_eq!(
            exact,
            TableEval {
                value: 10.0,
                slope: 10.0,
            },
            "exact table knots must keep the original left-segment slope"
        );
        assert_eq!(data.last_upper_index.load(Ordering::Relaxed), 1);

        let reverse = reverse_output_increasing(&data, 0.5);
        assert_eq!(reverse.value, 10.0);
        assert_eq!(reverse.slope, 0.0);
    }

    #[test]
    fn multi_input_lookup_cursor_falls_back_for_large_jumps() {
        let data = table_data(
            (1..=24)
                .map(|x| TablePoint {
                    x: x as Value,
                    y: x as Value,
                })
                .collect(),
        );

        assert_eq!(forward_output_increasing(&data, 2.5).value, 2.5);
        assert_eq!(data.last_upper_index.load(Ordering::Relaxed), 2);
        assert_eq!(forward_output_increasing(&data, 22.5).value, 22.5);
        assert_eq!(
            data.last_upper_index.load(Ordering::Relaxed),
            22,
            "large non-local jumps should land on the binary-search bracket"
        );
    }

    #[test]
    fn multi_input_preserves_linear_scan_for_descending_tables() {
        let data = table_data(vec![
            TablePoint { x: 1.0, y: 10.0 },
            TablePoint { x: 0.0, y: 20.0 },
        ]);

        assert!(!data.strictly_increasing_x);
        assert_eq!(forward_output(&data.points, 0.0).value, 10.0);
        assert_eq!(forward_output(&data.points, 2.0).value, 20.0);
        assert_eq!(reverse_output(&data.points, 0.0).value, 20.0);
        assert_eq!(reverse_output(&data.points, 2.0).value, 10.0);
    }
}

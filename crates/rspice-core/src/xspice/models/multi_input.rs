//! Multi-input analog XSPICE code models.

use crate::Value;
use crate::xspice::context::AnalogValue;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
};
use std::sync::{Arc, OnceLock};

const TABLE_RESOURCE: &str = "xspice.multi_input_pwl.table";

#[derive(Debug, Default)]
pub struct MultiInputPwl;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MultiInputPwlMode {
    And,
    Nand,
    Or,
    Nor,
}

#[derive(Debug, Clone, Copy)]
struct TableEval {
    value: Value,
    slope: Value,
}

#[derive(Debug, Clone, Copy)]
struct TablePoint {
    x: Value,
    y: Value,
}

#[derive(Debug, Clone, PartialEq)]
struct TableSignature {
    x_values: Vec<Value>,
    y_values: Vec<Value>,
}

#[derive(Debug, Clone)]
struct TableResource {
    signature: TableSignature,
    points: CmResult<Arc<Vec<TablePoint>>>,
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
    match raw.to_ascii_lowercase().as_str() {
        "and" => Ok(MultiInputPwlMode::And),
        "nand" => Ok(MultiInputPwlMode::Nand),
        "or" => Ok(MultiInputPwlMode::Or),
        "nor" => Ok(MultiInputPwlMode::Nor),
        _ => Err(invalid_param(
            "model",
            format!("unknown model '{raw}', expected and|nand|or|nor"),
        )),
    }
}

fn table_signature(ctx: &CmContext) -> TableSignature {
    TableSignature {
        x_values: ctx.real_vector_param("x").unwrap_or(&[]).to_vec(),
        y_values: ctx.real_vector_param("y").unwrap_or(&[]).to_vec(),
    }
}

fn table_uncached(ctx: &CmContext) -> CmResult<Vec<TablePoint>> {
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

    Ok(points)
}

fn cache_table(ctx: &mut CmContext) -> CmResult<Arc<Vec<TablePoint>>> {
    let signature = table_signature(ctx);
    if let Some(resource) = ctx.resource::<TableResource>(TABLE_RESOURCE)
        && resource.signature == signature
    {
        return resource.points.clone();
    }

    let points = table_uncached(ctx).map(Arc::new);
    ctx.set_resource(
        TABLE_RESOURCE,
        Arc::new(TableResource {
            signature,
            points: points.clone(),
        }),
    );
    points
}

fn table(ctx: &CmContext) -> CmResult<Arc<Vec<TablePoint>>> {
    let signature = table_signature(ctx);
    if let Some(resource) = ctx.resource::<TableResource>(TABLE_RESOURCE)
        && resource.signature == signature
    {
        return resource.points.clone();
    }

    table_uncached(ctx).map(Arc::new)
}

fn controlling_input(inputs: &[AnalogValue], mode: MultiInputPwlMode) -> Option<(usize, Value)> {
    match mode {
        MultiInputPwlMode::And | MultiInputPwlMode::Nand => inputs
            .iter()
            .map(|input| input.value)
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        MultiInputPwlMode::Or | MultiInputPwlMode::Nor => inputs
            .iter()
            .map(|input| input.value)
            .enumerate()
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

fn evaluate_multi_input_with_table(
    ctx: &CmContext,
    table: &[TablePoint],
) -> CmResult<Option<(usize, TableEval)>> {
    let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
    if inputs.len() < 2 {
        return Err(CmError::PortCountMismatch {
            expected: 2,
            actual: inputs.len(),
        });
    }

    let mode = mode(ctx)?;
    let Some((index, controlling_input)) = controlling_input(inputs, mode) else {
        return Ok(None);
    };

    let result = match mode {
        MultiInputPwlMode::And | MultiInputPwlMode::Or => forward_output(table, controlling_input),
        MultiInputPwlMode::Nand | MultiInputPwlMode::Nor => {
            reverse_output(table, controlling_input)
        }
    };

    Ok(Some((index, result)))
}

fn evaluate_multi_input(ctx: &CmContext) -> CmResult<Option<(usize, TableEval)>> {
    let table = table(ctx)?;
    evaluate_multi_input_with_table(ctx, &table)
}

fn evaluate_multi_input_cached(ctx: &mut CmContext) -> CmResult<Option<(usize, TableEval)>> {
    let table = cache_table(ctx)?;
    evaluate_multi_input_with_table(ctx, &table)
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
        assert_eq!(first[2].y, 20.0);

        ctx.set_real_vector_param("y", vec![0.0, 10.0, 30.0]);
        let updated = cache_table(&mut ctx).expect("updated table caches");
        assert!(
            !Arc::ptr_eq(&first, &updated),
            "changed multi_input_pwl table parameters must refresh the parsed table"
        );
        assert_eq!(updated[2].y, 30.0);
    }
}

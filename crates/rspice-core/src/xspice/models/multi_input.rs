//! Multi-input analog XSPICE code models.

use crate::Value;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
};
use std::sync::OnceLock;

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

fn table(ctx: &CmContext) -> CmResult<(&[Value], &[Value])> {
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
    }
    Ok((x, y))
}

fn controlling_input(inputs: &[Value], mode: MultiInputPwlMode) -> Option<(usize, Value)> {
    match mode {
        MultiInputPwlMode::And | MultiInputPwlMode::Nand => inputs
            .iter()
            .copied()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
        MultiInputPwlMode::Or | MultiInputPwlMode::Nor => inputs
            .iter()
            .copied()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)),
    }
}

fn forward_output(x_table: &[Value], y_table: &[Value], input: Value) -> TableEval {
    let last = x_table.len() - 1;
    if input <= x_table[0] {
        return TableEval {
            value: y_table[0],
            slope: 0.0,
        };
    }
    if input >= x_table[last] {
        return TableEval {
            value: y_table[last],
            slope: 0.0,
        };
    }

    for idx in 1..x_table.len() {
        if input > x_table[idx - 1] && input <= x_table[idx] {
            let slope = (y_table[idx] - y_table[idx - 1]) / (x_table[idx] - x_table[idx - 1]);
            return TableEval {
                value: y_table[idx] + slope * (input - x_table[idx]),
                slope,
            };
        }
    }

    TableEval {
        value: y_table[last],
        slope: 0.0,
    }
}

fn reverse_output(x_table: &[Value], y_table: &[Value], input: Value) -> TableEval {
    let last = x_table.len() - 1;
    if input <= x_table[0] {
        return TableEval {
            value: y_table[last],
            slope: 0.0,
        };
    }
    if input >= x_table[last] {
        return TableEval {
            value: y_table[0],
            slope: 0.0,
        };
    }

    for idx in 1..x_table.len() {
        if input > x_table[idx - 1] && input <= x_table[idx] {
            return TableEval {
                value: y_table[last - idx],
                slope: 0.0,
            };
        }
    }

    TableEval {
        value: y_table[0],
        slope: 0.0,
    }
}

fn evaluate_multi_input(ctx: &CmContext) -> CmResult<Option<(usize, TableEval)>> {
    let (x_table, y_table) = table(ctx)?;
    let inputs = ctx.input_vector("in");
    if inputs.len() < 2 {
        return Err(CmError::PortCountMismatch {
            expected: 2,
            actual: inputs.len(),
        });
    }

    let mode = mode(ctx)?;
    let Some((index, controlling_input)) = controlling_input(&inputs, mode) else {
        return Ok(None);
    };

    let result = match mode {
        MultiInputPwlMode::And | MultiInputPwlMode::Or => {
            forward_output(x_table, y_table, controlling_input)
        }
        MultiInputPwlMode::Nand | MultiInputPwlMode::Nor => {
            reverse_output(x_table, y_table, controlling_input)
        }
    };

    Ok(Some((index, result)))
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
        table(ctx)?;
        mode(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let output = evaluate_multi_input(ctx)?.map_or(0.0, |(_, result)| result.value);
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

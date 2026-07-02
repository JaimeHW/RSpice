//! Analog Behavioral Code Models
//!
//! Implements fundamental analog behavioral blocks used in mixed-signal simulation.

use crate::xspice::context::AnalogValue;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, EvaluationPhase, ParamSpec, PortDirection, PortSpec,
    PortType,
};
use crate::{Complex64, Value};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

const MULT_TRANSFER_RESOURCE: &str = "xspice.analog.mult.transfer";
const DIVIDE_TRANSFER_RESOURCE: &str = "xspice.analog.divide.transfer";
const LIMIT_TRANSFER_RESOURCE: &str = "xspice.analog.limit.transfer";
const CLIMIT_TRANSFER_RESOURCE: &str = "xspice.analog.climit.transfer";

fn scalar_analog_port(
    name: &str,
    direction: PortDirection,
    default_type: PortType,
    allowed_types: Vec<PortType>,
    null_allowed: bool,
    description: &str,
) -> PortSpec {
    PortSpec {
        name: name.to_string(),
        direction,
        default_type,
        allowed_types,
        is_vector: false,
        null_allowed,
        vector_min_len: None,
        vector_max_len: None,
        description: description.to_string(),
    }
}

fn broad_scalar_analog_input_port(name: &str, description: &str) -> PortSpec {
    scalar_analog_port(
        name,
        PortDirection::In,
        PortType::Voltage,
        vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
            PortType::VoltageName,
        ],
        false,
        description,
    )
}

fn scalar_analog_output_port(name: &str, description: &str) -> PortSpec {
    scalar_analog_port(
        name,
        PortDirection::Out,
        PortType::Voltage,
        vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
        ],
        false,
        description,
    )
}

//=============================================================================
// Gain Block
//=============================================================================

/// Gain code model: out = gain * (in + in_offset) + out_offset
///
/// # Parameters
/// - `gain` - Voltage gain (default: 1.0)
/// - `in_offset` - Input offset voltage (default: 0.0)
/// - `out_offset` - Output offset voltage (default: 0.0)
///
/// # Ports
/// - `in` - Analog voltage input
/// - `out` - Analog voltage output
#[derive(Debug, Default)]
pub struct Gain;

impl CodeModel for Gain {
    fn name(&self) -> &str {
        "gain"
    }

    fn description(&self) -> &str {
        "Voltage gain block: out = gain * (in + in_offset) + out_offset"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
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
                        PortType::VoltageName,
                    ],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Analog input".to_string(),
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
                    description: "Analog output".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_offset", 0.0).with_description("Input offset voltage"),
                ParamSpec::real("gain", 1.0).with_description("Voltage gain factor"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param("gain");
        let in_offset = ctx.param("in_offset");
        let out_offset = ctx.param("out_offset");

        let v_in = ctx.input("in");
        let v_out = gain * (v_in + in_offset) + out_offset;

        // Provide both operating-point output and linearized gain.
        ctx.set_output_with_partial("out", v_out, gain);

        Ok(())
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        vec![ctx.param("gain")]
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.param("gain"))]
        } else {
            Vec::new()
        }
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

//=============================================================================
// Summer
//=============================================================================

/// Analog summer: out = out_gain * sum(in_gain[i] * (in[i] + in_offset[i])) + out_offset
///
/// # Parameters
/// - `in_offset` - Offset for each input (default: 0.0)
/// - `in_gain` - Gain for each input (default: 1.0)
/// - `out_gain` - Output gain (default: 1.0)
/// - `out_offset` - Output offset (default: 0.0)
///
/// # Ports
/// - `in` - Vector analog voltage input
/// - `out` - Analog voltage output
#[derive(Debug, Default)]
pub struct Summer;

impl CodeModel for Summer {
    fn name(&self) -> &str {
        "summer"
    }

    fn description(&self) -> &str {
        "Analog summer with per-input offsets/gains and output gain"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
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
                        PortType::VoltageName,
                    ],
                    is_vector: true,
                    null_allowed: false,
                    vector_min_len: Some(2),
                    vector_max_len: None,
                    description: "Vector of analog inputs".to_string(),
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
                    description: "Summed analog output".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real_vector("in_offset", Vec::new())
                    .with_description("Offset applied to each input, defaulting each input to 0.0"),
                ParamSpec::real_vector("in_gain", Vec::new())
                    .with_description("Gain applied to each input, defaulting each input to 1.0"),
                ParamSpec::real("out_gain", 1.0).with_description("Output gain factor"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        validate_analog_vector_params(ctx, "summer")
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let out_gain = ctx.param("out_gain");
        let out_offset = ctx.param("out_offset");

        let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
        let in_gain = analog_vector_param_values(ctx, "in_gain", inputs.len(), "summer")?;
        let in_offset = analog_vector_param_values(ctx, "in_offset", inputs.len(), "summer")?;
        let sum: Value = inputs
            .iter()
            .enumerate()
            .map(|(index, input)| {
                let gain = analog_vector_param_at(in_gain, index, 1.0);
                let offset = analog_vector_param_at(in_offset, index, 0.0);
                gain * (input.value + offset)
            })
            .sum();
        let v_out = sum * out_gain + out_offset;

        ctx.set_output("out", v_out);
        Ok(())
    }

    fn output_input_vector_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
    ) -> Vec<(String, usize, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }

        let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
        let Ok(in_gain) = analog_vector_param_values(ctx, "in_gain", inputs.len(), "summer") else {
            return Vec::new();
        };
        let out_gain = ctx.param("out_gain");
        (0..inputs.len())
            .map(|index| {
                (
                    "in".to_string(),
                    index,
                    analog_vector_param_at(in_gain, index, 1.0) * out_gain,
                )
            })
            .collect()
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

fn analog_vector_param_values<'a>(
    ctx: &'a CmContext,
    name: &str,
    width: usize,
    model_name: &str,
) -> CmResult<Option<&'a [Value]>> {
    let supplied = ctx.real_vector_param(name).unwrap_or(&[]);
    if supplied.is_empty() {
        return Ok(None);
    }
    validate_analog_vector_param_width(name, supplied.len(), width, model_name)?;
    for (index, value) in supplied.iter().copied().enumerate() {
        if !value.is_finite() {
            return Err(CmError::InvalidParameter {
                name: name.to_string(),
                message: format!("element {index} must be finite, got {value}"),
            });
        }
    }
    Ok(Some(supplied))
}

fn analog_vector_param_at(values: Option<&[Value]>, index: usize, default_value: Value) -> Value {
    values
        .and_then(|values| values.get(index).copied())
        .unwrap_or(default_value)
}

fn validate_analog_vector_param_width(
    name: &str,
    len: usize,
    width: usize,
    model_name: &str,
) -> CmResult<()> {
    if len != width {
        return Err(CmError::InvalidParameter {
            name: name.to_string(),
            message: format!("vector length {len} must match {model_name} input width {width}"),
        });
    }
    Ok(())
}

fn validate_analog_vector_params(ctx: &CmContext, model_name: &str) -> CmResult<()> {
    let width = ctx.port_width("in");
    for name in ["in_offset", "in_gain"] {
        let supplied = ctx.real_vector_param(name).unwrap_or(&[]);
        if !supplied.is_empty() {
            validate_analog_vector_param_width(name, supplied.len(), width, model_name)?;
        }
    }
    Ok(())
}

//=============================================================================
// Multiplier
//=============================================================================

/// Analog multiplier: out = out_gain * product(in_gain[i] * (in[i] + in_offset[i])) + out_offset
///
/// # Parameters
/// - `in_offset` - Offset for each input (default: 0.0)
/// - `in_gain` - Gain for each input (default: 1.0)
/// - `out_gain` - Output gain factor (default: 1.0)
/// - `out_offset` - Output offset (default: 0.0)
///
/// # Ports
/// - `in` - Vector analog input
/// - `out` - Analog voltage output
#[derive(Debug, Default)]
pub struct Multiplier;

impl CodeModel for Multiplier {
    fn name(&self) -> &str {
        "mult"
    }

    fn description(&self) -> &str {
        "Analog vector-input multiplier with per-input offsets/gains"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
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
                        PortType::VoltageName,
                    ],
                    is_vector: true,
                    null_allowed: false,
                    vector_min_len: Some(2),
                    vector_max_len: None,
                    description: "Vector of analog inputs".to_string(),
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
                    description: "Product output".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real_vector("in_offset", Vec::new())
                    .with_description("Offset applied to each input, defaulting each input to 0.0"),
                ParamSpec::real_vector("in_gain", Vec::new())
                    .with_description("Gain applied to each input, defaulting each input to 1.0"),
                ParamSpec::real("out_gain", 1.0).with_description("Output gain factor"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        validate_analog_vector_params(ctx, "mult")
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let transfer = cache_mult_transfer(ctx)?;

        ctx.set_output("out", mult_output_from_transfer(transfer.as_ref()));
        Ok(())
    }

    fn output_input_vector_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
    ) -> Vec<(String, usize, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }

        let Ok(transfer) = mult_transfer_for_context(ctx) else {
            return Vec::new();
        };
        mult_partials_from_transfer(transfer.as_ref())
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

#[derive(Debug, Clone, PartialEq)]
struct MultTransferSignature {
    inputs: Vec<Value>,
    in_gain_revision: Option<u64>,
    in_offset_revision: Option<u64>,
    out_gain: Value,
    out_offset: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MultTransferBaseSignature {
    in_gain_revision: Option<u64>,
    in_offset_revision: Option<u64>,
    out_gain: Value,
    out_offset: Value,
}

#[derive(Debug, Clone, PartialEq)]
struct MultTransfer {
    accumulate_in: Value,
    transfer_gain: Value,
    out_offset: Value,
    shifted_inputs: Vec<Value>,
}

#[derive(Debug, Clone)]
struct MultTransferResource {
    signature: MultTransferSignature,
    transfer: Arc<MultTransfer>,
}

fn mult_transfer_base_signature(ctx: &CmContext) -> MultTransferBaseSignature {
    MultTransferBaseSignature {
        in_gain_revision: ctx.real_vector_param_revision("in_gain"),
        in_offset_revision: ctx.real_vector_param_revision("in_offset"),
        out_gain: ctx.param("out_gain"),
        out_offset: ctx.param("out_offset"),
    }
}

fn mult_transfer_signature_from_inputs(
    base: MultTransferBaseSignature,
    inputs: &[AnalogValue],
) -> MultTransferSignature {
    MultTransferSignature {
        inputs: inputs.iter().map(|input| input.value).collect(),
        in_gain_revision: base.in_gain_revision,
        in_offset_revision: base.in_offset_revision,
        out_gain: base.out_gain,
        out_offset: base.out_offset,
    }
}

fn mult_transfer_signature(ctx: &CmContext) -> MultTransferSignature {
    mult_transfer_signature_from_inputs(
        mult_transfer_base_signature(ctx),
        ctx.input_analog_vector_values("in").unwrap_or(&[]),
    )
}

fn mult_transfer_inputs_match(cached: &[Value], inputs: &[AnalogValue]) -> bool {
    cached.len() == inputs.len()
        && cached
            .iter()
            .zip(inputs)
            .all(|(cached, input)| *cached == input.value)
}

fn mult_transfer_resource_matches(
    resource: &MultTransferResource,
    base: MultTransferBaseSignature,
    inputs: &[AnalogValue],
) -> bool {
    resource.signature.in_gain_revision == base.in_gain_revision
        && resource.signature.in_offset_revision == base.in_offset_revision
        && resource.signature.out_gain == base.out_gain
        && resource.signature.out_offset == base.out_offset
        && mult_transfer_inputs_match(&resource.signature.inputs, inputs)
}

fn mult_transfer_from_context_with_signature(
    ctx: &CmContext,
) -> CmResult<(MultTransferSignature, Arc<MultTransfer>)> {
    let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
    if inputs.len() < 2 {
        return Err(CmError::EvaluationError(format!(
            "mult requires at least 2 inputs, got {}",
            inputs.len()
        )));
    }

    let in_gain = analog_vector_param_values(ctx, "in_gain", inputs.len(), "mult")?;
    let in_offset = analog_vector_param_values(ctx, "in_offset", inputs.len(), "mult")?;
    let out_gain = ctx.param("out_gain");
    let out_offset = ctx.param("out_offset");

    let mut accumulate_gain = 1.0;
    let mut accumulate_in = 1.0;
    let mut signature_inputs = Vec::with_capacity(inputs.len());
    let mut shifted_inputs = Vec::with_capacity(inputs.len());
    for (index, input) in inputs.iter().enumerate() {
        let gain = analog_vector_param_at(in_gain, index, 1.0);
        let offset = analog_vector_param_at(in_offset, index, 0.0);
        let shifted = input.value + offset;
        signature_inputs.push(input.value);
        accumulate_gain *= gain;
        accumulate_in *= shifted;
        shifted_inputs.push(shifted);
    }

    let transfer_gain = accumulate_gain * out_gain;
    Ok((
        MultTransferSignature {
            inputs: signature_inputs,
            in_gain_revision: ctx.real_vector_param_revision("in_gain"),
            in_offset_revision: ctx.real_vector_param_revision("in_offset"),
            out_gain,
            out_offset,
        },
        Arc::new(MultTransfer {
            accumulate_in,
            transfer_gain,
            out_offset,
            shifted_inputs,
        }),
    ))
}

fn mult_transfer_from_context(ctx: &CmContext) -> CmResult<Arc<MultTransfer>> {
    let (_, transfer) = mult_transfer_from_context_with_signature(ctx)?;
    Ok(transfer)
}

fn mult_transfer_for_context(ctx: &CmContext) -> CmResult<Arc<MultTransfer>> {
    let base = mult_transfer_base_signature(ctx);
    let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
    if let Some(resource) = ctx.resource::<MultTransferResource>(MULT_TRANSFER_RESOURCE)
        && mult_transfer_resource_matches(&resource, base, inputs)
    {
        return Ok(Arc::clone(&resource.transfer));
    }

    mult_transfer_from_context(ctx)
}

fn cache_mult_transfer(ctx: &mut CmContext) -> CmResult<Arc<MultTransfer>> {
    let base = mult_transfer_base_signature(ctx);
    let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
    if let Some(resource) = ctx.resource::<MultTransferResource>(MULT_TRANSFER_RESOURCE)
        && mult_transfer_resource_matches(&resource, base, inputs)
    {
        return Ok(Arc::clone(&resource.transfer));
    }

    let (signature, transfer) = mult_transfer_from_context_with_signature(ctx)?;
    ctx.set_resource(
        MULT_TRANSFER_RESOURCE,
        Arc::new(MultTransferResource {
            signature,
            transfer: Arc::clone(&transfer),
        }),
    );
    Ok(transfer)
}

fn mult_output_from_transfer(transfer: &MultTransfer) -> Value {
    transfer.accumulate_in * transfer.transfer_gain + transfer.out_offset
}

fn mult_output_from_context(ctx: &CmContext) -> CmResult<Value> {
    let transfer = mult_transfer_for_context(ctx)?;
    Ok(mult_output_from_transfer(transfer.as_ref()))
}

fn mult_partials_from_transfer(transfer: &MultTransfer) -> Vec<(String, usize, Value)> {
    let mut partials = Vec::with_capacity(transfer.shifted_inputs.len());
    let mut zero_count = 0usize;
    let mut nonzero_product = 1.0;
    for shifted in transfer.shifted_inputs.iter().copied() {
        if shifted == 0.0 {
            zero_count += 1;
        } else {
            nonzero_product *= shifted;
        }
    }

    for (index, shifted) in transfer.shifted_inputs.iter().copied().enumerate() {
        let partial = if !transfer.transfer_gain.is_finite() {
            0.0
        } else if zero_count == 0
            && transfer.accumulate_in != 0.0
            && transfer.accumulate_in.is_finite()
            && shifted != 0.0
        {
            transfer.accumulate_in / shifted * transfer.transfer_gain
        } else if zero_count == 1 && shifted == 0.0 && nonzero_product.is_finite() {
            nonzero_product * transfer.transfer_gain
        } else {
            0.0
        };
        partials.push(("in".to_string(), index, partial));
    }
    partials
}

fn mult_partials_from_context(ctx: &CmContext) -> CmResult<Vec<(String, usize, Value)>> {
    let transfer = mult_transfer_for_context(ctx)?;
    Ok(mult_partials_from_transfer(transfer.as_ref()))
}

//=============================================================================
// Divider
//=============================================================================

/// Analog divider: out = out_offset + out_gain * ((num + num_offset) * num_gain) /
/// ((den + den_offset) * den_gain), with ngspice-compatible denominator limiting.
///
/// # Parameters
/// - `num_offset` - Numerator offset (default: 0.0)
/// - `num_gain` - Numerator gain (default: 1.0)
/// - `den_offset` - Denominator offset (default: 0.0)
/// - `den_gain` - Denominator gain (default: 1.0)
/// - `den_lower_limit` - Minimum denominator magnitude (default: 1e-10)
/// - `den_domain` - Denominator smoothing domain (default: 1e-16)
/// - `fraction` - Treat `den_domain` as a fraction of `den_lower_limit` (default: false)
/// - `out_gain` - Output gain (default: 1.0)
/// - `out_offset` - Output offset (default: 0.0)
///
/// # Ports
/// - `num` - Numerator input
/// - `den` - Denominator input
/// - `out` - Quotient output
#[derive(Debug, Default)]
pub struct Divider;

impl CodeModel for Divider {
    fn name(&self) -> &str {
        "divider"
    }

    fn description(&self) -> &str {
        "Analog divider with input offsets/gains and smoothed denominator limiting"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec {
                    name: "num".to_string(),
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
                    description: "Numerator input".to_string(),
                },
                PortSpec {
                    name: "den".to_string(),
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
                    description: "Denominator input".to_string(),
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
                    description: "Quotient output".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("num_offset", 0.0).with_description("Numerator offset"),
                ParamSpec::real("num_gain", 1.0).with_description("Numerator gain"),
                ParamSpec::real("den_offset", 0.0).with_description("Denominator offset"),
                ParamSpec::real("den_gain", 1.0).with_description("Denominator gain"),
                ParamSpec::real("den_lower_limit", 1.0e-10)
                    .with_description("Minimum denominator magnitude, clamped to 1e-10"),
                ParamSpec::real("den_domain", 1.0e-16)
                    .with_description("Denominator smoothing domain"),
                ParamSpec::boolean("fraction", false)
                    .with_description("Treat den_domain as a fraction of den_lower_limit"),
                ParamSpec::real("out_gain", 1.0).with_description("Output gain factor"),
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let transfer = cache_divide_transfer(ctx);
        ctx.set_output("out", transfer.output);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if !output_port.eq_ignore_ascii_case("out") {
            return Vec::new();
        }

        let transfer = divide_transfer_for_context(ctx);
        if !transfer.num_partial.is_finite() || !transfer.den_partial.is_finite() {
            return Vec::new();
        }

        vec![
            ("num".to_string(), transfer.num_partial),
            ("den".to_string(), transfer.den_partial),
        ]
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DivideTransferSignature {
    num: Value,
    den: Value,
    num_offset: Value,
    num_gain: Value,
    den_offset: Value,
    den_gain: Value,
    den_lower_limit: Value,
    den_domain: Value,
    fraction: bool,
    out_gain: Value,
    out_offset: Value,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct DivideTransfer {
    output: Value,
    num_partial: Value,
    den_partial: Value,
}

#[derive(Debug, Clone, Copy)]
struct DivideTransferResource {
    signature: DivideTransferSignature,
    transfer: DivideTransfer,
}

fn divide_transfer_signature(ctx: &CmContext) -> DivideTransferSignature {
    DivideTransferSignature {
        num: ctx.input("num"),
        den: ctx.input("den"),
        num_offset: ctx.param("num_offset"),
        num_gain: ctx.param("num_gain"),
        den_offset: ctx.param("den_offset"),
        den_gain: ctx.param("den_gain"),
        den_lower_limit: ctx.param("den_lower_limit"),
        den_domain: ctx.param("den_domain"),
        fraction: ctx.param("fraction") > 0.5,
        out_gain: ctx.param("out_gain"),
        out_offset: ctx.param("out_offset"),
    }
}

fn divide_transfer_from_signature(signature: DivideTransferSignature) -> DivideTransfer {
    let den_lower_limit = signature.den_lower_limit.max(1.0e-10);
    let den_domain = if signature.fraction {
        signature.den_domain * den_lower_limit
    } else {
        signature.den_domain
    };

    let numerator = (signature.num + signature.num_offset) * signature.num_gain;
    let denominator = (signature.den + signature.den_offset) * signature.den_gain;
    let (limited_den, den_partial) =
        divide_limited_denominator(denominator, den_lower_limit, den_domain);

    let output = signature.out_offset + signature.out_gain * numerator / limited_den;
    let num_partial = signature.out_gain * signature.num_gain / limited_den;
    let den_partial = -signature.out_gain * numerator * signature.den_gain * den_partial
        / (limited_den * limited_den);
    DivideTransfer {
        output,
        num_partial,
        den_partial,
    }
}

fn divide_transfer_for_context(ctx: &CmContext) -> DivideTransfer {
    let signature = divide_transfer_signature(ctx);
    if let Some(resource) = ctx.resource::<DivideTransferResource>(DIVIDE_TRANSFER_RESOURCE)
        && resource.signature == signature
    {
        return resource.transfer;
    }

    divide_transfer_from_signature(signature)
}

fn cache_divide_transfer(ctx: &mut CmContext) -> DivideTransfer {
    let signature = divide_transfer_signature(ctx);
    if let Some(resource) = ctx.resource::<DivideTransferResource>(DIVIDE_TRANSFER_RESOURCE)
        && resource.signature == signature
    {
        return resource.transfer;
    }

    let transfer = divide_transfer_from_signature(signature);
    ctx.set_resource(
        DIVIDE_TRANSFER_RESOURCE,
        Arc::new(DivideTransferResource {
            signature,
            transfer,
        }),
    );
    transfer
}

fn divide_limited_denominator(
    denominator: Value,
    den_lower_limit: Value,
    den_domain: Value,
) -> (Value, Value) {
    let threshold_upper = den_lower_limit + den_domain;
    let threshold_lower = den_lower_limit - den_domain;

    if denominator < threshold_upper && denominator >= 0.0 {
        if den_domain > 0.0 && denominator > threshold_lower {
            smooth_corner(
                denominator,
                den_lower_limit,
                den_lower_limit,
                den_domain,
                0.0,
                1.0,
            )
        } else {
            (den_lower_limit, 0.0)
        }
    } else if denominator > -threshold_upper && denominator < 0.0 {
        if den_domain > 0.0 && denominator < -threshold_lower {
            smooth_corner(
                denominator,
                -den_lower_limit,
                -den_lower_limit,
                den_domain,
                0.0,
                1.0,
            )
        } else {
            (-den_lower_limit, 0.0)
        }
    } else {
        (denominator, 1.0)
    }
}

//=============================================================================
// Limiter
//=============================================================================

/// Limiter with gain: out = clamp(gain * (in + in_offset), out_lower, out_upper)
/// with ngspice-compatible parabolic smoothing at both limits.
///
/// # Parameters
/// - `gain` - Gain in linear region (default: 1.0)
/// - `in_offset` - Input offset (default: 0.0)
/// - `out_lower_limit` - Lower output limit (default: -1e12)
/// - `out_upper_limit` - Upper output limit (default: 1e12)
/// - `limit_range` - Smoothing range (default: 1e-6)
/// - `fraction` - Treat `limit_range` as a fraction of the output span (default: false)
///
/// # Ports
/// - `in` - Analog voltage input
/// - `out` - Analog voltage output (limited)
#[derive(Debug, Default)]
pub struct Limiter;

impl CodeModel for Limiter {
    fn name(&self) -> &str {
        "limit"
    }

    fn description(&self) -> &str {
        "Limiter with gain and ngspice-compatible smoothing"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
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
                        PortType::VoltageName,
                    ],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Analog input".to_string(),
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
                    description: "Limited analog output".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_offset", 0.0).with_description("Input offset voltage"),
                ParamSpec::real("gain", 1.0).with_description("Gain in linear region"),
                ParamSpec::real("out_lower_limit", -1e12)
                    .required()
                    .with_description("Lower output limit"),
                ParamSpec::real("out_upper_limit", 1e12)
                    .required()
                    .with_description("Upper output limit"),
                ParamSpec::real("limit_range", 1.0e-6)
                    .with_description("Smoothing range for limit transitions"),
                ParamSpec::boolean("fraction", false)
                    .with_description("Treat limit_range as a fraction of the output span"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let transfer = cache_limit_transfer(ctx);
        ctx.set_output_with_partial("out", transfer.output, transfer.in_partial);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), limit_transfer_for_context(ctx).in_partial)]
        } else {
            Vec::new()
        }
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LimitTransferSignature {
    input: Value,
    in_offset: Value,
    out_lower_limit: Value,
    out_upper_limit: Value,
    limit_range: Value,
    gain: Value,
    fraction: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct LimitTransfer {
    output: Value,
    in_partial: Value,
}

#[derive(Debug, Clone, Copy)]
struct LimitTransferResource {
    signature: LimitTransferSignature,
    transfer: LimitTransfer,
}

fn limit_transfer_signature(ctx: &CmContext) -> LimitTransferSignature {
    LimitTransferSignature {
        input: ctx.input("in"),
        in_offset: ctx.param("in_offset"),
        out_lower_limit: ctx.param("out_lower_limit"),
        out_upper_limit: ctx.param("out_upper_limit"),
        limit_range: ctx.param("limit_range"),
        gain: ctx.param("gain"),
        fraction: ctx.param("fraction") > 0.5,
    }
}

fn limit_transfer_from_signature(signature: LimitTransferSignature) -> LimitTransfer {
    let (output, in_partial) = limit_transfer(
        signature.input,
        signature.in_offset,
        signature.out_lower_limit,
        signature.out_upper_limit,
        signature.limit_range,
        signature.gain,
        signature.fraction,
    );
    LimitTransfer { output, in_partial }
}

fn limit_transfer_from_context(ctx: &CmContext) -> (Value, Value) {
    let transfer = limit_transfer_for_context(ctx);
    (transfer.output, transfer.in_partial)
}

fn limit_transfer_for_context(ctx: &CmContext) -> LimitTransfer {
    let signature = limit_transfer_signature(ctx);
    if let Some(resource) = ctx.resource::<LimitTransferResource>(LIMIT_TRANSFER_RESOURCE)
        && resource.signature == signature
    {
        return resource.transfer;
    }

    limit_transfer_from_signature(signature)
}

fn cache_limit_transfer(ctx: &mut CmContext) -> LimitTransfer {
    let signature = limit_transfer_signature(ctx);
    if let Some(resource) = ctx.resource::<LimitTransferResource>(LIMIT_TRANSFER_RESOURCE)
        && resource.signature == signature
    {
        return resource.transfer;
    }

    let transfer = limit_transfer_from_signature(signature);
    ctx.set_resource(
        LIMIT_TRANSFER_RESOURCE,
        Arc::new(LimitTransferResource {
            signature,
            transfer,
        }),
    );
    transfer
}

fn limit_transfer(
    input: Value,
    in_offset: Value,
    out_lower_limit: Value,
    out_upper_limit: Value,
    mut limit_range: Value,
    gain: Value,
    fraction: bool,
) -> (Value, Value) {
    if fraction {
        limit_range *= out_upper_limit - out_lower_limit;
    }

    let threshold_upper = out_upper_limit - limit_range;
    let threshold_lower = out_lower_limit + limit_range;
    let raw = gain * (in_offset + input);

    if raw < threshold_lower {
        if raw > out_lower_limit - limit_range && limit_range > 0.0 {
            let (limited_out, partial) =
                smooth_corner(raw, out_lower_limit, out_lower_limit, limit_range, 0.0, 1.0);
            (limited_out, gain * partial)
        } else {
            (out_lower_limit, 0.0)
        }
    } else if raw > threshold_upper {
        if raw < out_upper_limit + limit_range && limit_range > 0.0 {
            let (limited_out, partial) =
                smooth_corner(raw, out_upper_limit, out_upper_limit, limit_range, 1.0, 0.0);
            (limited_out, gain * partial)
        } else {
            (out_upper_limit, 0.0)
        }
    } else {
        (raw, gain)
    }
}

//=============================================================================
// Controlled Limiter
//=============================================================================

/// Controlled limiter: out = clamp(gain * (in + in_offset), lower, upper)
///
/// The lower and upper limits are controlled by analog input ports, matching
/// ngspice's XSPICE `climit` code model.
#[derive(Debug, Default)]
pub struct ControlledLimiter;

impl CodeModel for ControlledLimiter {
    fn name(&self) -> &str {
        "climit"
    }

    fn description(&self) -> &str {
        "Controlled limiter block with analog upper and lower limit inputs"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                broad_scalar_analog_input_port("in", "Analog input"),
                broad_scalar_analog_input_port("cntl_upper", "Upper limit control input"),
                broad_scalar_analog_input_port("cntl_lower", "Lower limit control input"),
                scalar_analog_output_port("out", "Controlled limited output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_offset", 0.0).with_description("Input offset voltage"),
                ParamSpec::real("gain", 1.0).with_description("Linear-region gain"),
                ParamSpec::real("upper_delta", 0.0)
                    .with_description("Delta subtracted from the upper control input"),
                ParamSpec::real("lower_delta", 0.0)
                    .with_description("Delta added to the lower control input"),
                ParamSpec::real("limit_range", 1.0e-6)
                    .with_description("Smoothing range near the controlled limits"),
                ParamSpec::boolean("fraction", false)
                    .with_description("Treat limit_range as a fraction of the limit span"),
            ]
        })
    }

    fn init(&self, _ctx: &mut CmContext) -> CmResult<()> {
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let transfer = cache_climit_transfer(ctx);

        ctx.set_output_with_partial("out", transfer.output, transfer.in_partial);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            let transfer = climit_transfer_for_context(ctx);
            vec![
                ("in".to_string(), transfer.in_partial),
                ("cntl_lower".to_string(), transfer.lower_partial),
                ("cntl_upper".to_string(), transfer.upper_partial),
            ]
        } else {
            Vec::new()
        }
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

//=============================================================================
// Hysteresis Block
//=============================================================================

/// Official XSPICE `hyst` code model.
#[derive(Debug, Default)]
pub struct HysteresisBlock;

const HYST_STATE: usize = 0;
const HYST_UNINITIALIZED: i64 = i64::MIN;
const HYST_RISING: i64 = 1;
const HYST_FALLING: i64 = 0;

#[derive(Debug, Clone, Copy)]
struct HystParams {
    in_low: Value,
    in_high: Value,
    hyst: Value,
    out_lower_limit: Value,
    out_upper_limit: Value,
    input_domain: Value,
}

fn hyst_error(message: impl Into<String>) -> CmError {
    CmError::EvaluationError(format!("hyst: {}", message.into()))
}

fn hyst_params(ctx: &CmContext) -> CmResult<HystParams> {
    let in_low = ctx.param("in_low");
    let in_high = ctx.param("in_high");
    let raw_hyst = ctx.param("hyst");
    let out_lower_limit = ctx.param("out_lower_limit");
    let out_upper_limit = ctx.param("out_upper_limit");
    let mut input_domain = ctx.param("input_domain");

    for (name, value) in [
        ("in_low", in_low),
        ("in_high", in_high),
        ("hyst", raw_hyst),
        ("out_lower_limit", out_lower_limit),
        ("out_upper_limit", out_upper_limit),
        ("input_domain", input_domain),
    ] {
        if !value.is_finite() {
            return Err(hyst_error(format!("{name} must be finite, got {value}")));
        }
    }
    if in_high == in_low {
        return Err(hyst_error(format!(
            "in_high must differ from in_low, got in_low={in_low}, in_high={in_high}"
        )));
    }
    let hyst = raw_hyst.max(0.0);

    if ctx.param("fraction") > 0.5 {
        input_domain *= in_high - in_low;
    }

    Ok(HystParams {
        in_low,
        in_high,
        hyst,
        out_lower_limit,
        out_upper_limit,
        input_domain,
    })
}

fn hyst_transfer(input: Value, old_state: i64, params: HystParams) -> (Value, Value, i64) {
    let slope =
        (params.out_upper_limit - params.out_lower_limit) / (params.in_high - params.in_low);
    let x_rise_linear = params.in_low + params.hyst;
    let x_rise_zero = params.in_high + params.hyst;
    let x_fall_linear = params.in_high - params.hyst;
    let x_fall_zero = params.in_low - params.hyst;
    let domain = params.input_domain;

    let mut state = old_state;
    if state == HYST_UNINITIALIZED {
        state = if input < x_rise_zero + domain {
            HYST_RISING
        } else {
            HYST_FALLING
        };
    }

    if state == HYST_RISING {
        if input <= x_rise_linear - domain {
            (params.out_lower_limit, 0.0, HYST_RISING)
        } else if domain > 0.0 && input <= x_rise_linear + domain {
            let (out, partial) = smooth_corner(
                input,
                x_rise_linear,
                params.out_lower_limit,
                domain,
                0.0,
                slope,
            );
            (out, partial, HYST_RISING)
        } else if input <= x_rise_zero - domain {
            (
                (input - x_rise_linear) * slope + params.out_lower_limit,
                slope,
                HYST_RISING,
            )
        } else if domain > 0.0 && input <= x_rise_zero + domain {
            let (out, partial) = smooth_corner(
                input,
                x_rise_zero,
                params.out_upper_limit,
                domain,
                slope,
                0.0,
            );
            (out, partial, HYST_RISING)
        } else {
            (params.out_upper_limit, 0.0, HYST_FALLING)
        }
    } else if input >= x_fall_linear + domain {
        (params.out_upper_limit, 0.0, HYST_FALLING)
    } else if domain > 0.0 && input >= x_fall_linear - domain {
        let (out, partial) = smooth_corner(
            input,
            x_fall_linear,
            params.out_upper_limit,
            domain,
            slope,
            0.0,
        );
        (out, partial, HYST_FALLING)
    } else if input >= x_fall_zero + domain {
        (
            (input - x_fall_zero) * slope + params.out_lower_limit,
            slope,
            HYST_FALLING,
        )
    } else if domain > 0.0 && input >= x_fall_zero - domain {
        let (out, partial) = smooth_corner(
            input,
            x_fall_zero,
            params.out_lower_limit,
            domain,
            0.0,
            slope,
        );
        (out, partial, HYST_FALLING)
    } else {
        (params.out_lower_limit, 0.0, HYST_RISING)
    }
}

impl CodeModel for HysteresisBlock {
    fn name(&self) -> &str {
        "hyst"
    }

    fn description(&self) -> &str {
        "Hysteresis block"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                broad_scalar_analog_input_port("in", "Analog input"),
                scalar_analog_output_port("out", "Analog output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_low", 0.0).with_description("Input low value"),
                ParamSpec::real("in_high", 1.0).with_description("Input high value"),
                ParamSpec::real("hyst", 0.1)
                    .with_description("Hysteresis width, clamped to nonnegative"),
                ParamSpec::real("out_lower_limit", 0.0).with_description("Output lower limit"),
                ParamSpec::real("out_upper_limit", 1.0).with_description("Output upper limit"),
                ParamSpec::real("input_domain", 0.01).with_description("Input smoothing domain"),
                ParamSpec::boolean("fraction", true)
                    .with_description("Treat input_domain as a fraction of input range"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        hyst_params(ctx)?;
        ctx.allocate_int_states(1);
        ctx.set_int_state(HYST_STATE, HYST_UNINITIALIZED);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let params = hyst_params(ctx)?;
        let (out, partial, state) =
            hyst_transfer(ctx.input("in"), ctx.int_state(HYST_STATE), params);
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_int_state(HYST_STATE, state);
        }
        ctx.set_output_with_partial("out", out, partial);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
        } else {
            Vec::new()
        }
    }
}

//=============================================================================
// Slew-Rate Follower
//=============================================================================

/// Official XSPICE `slew` code model.
#[derive(Debug, Default)]
pub struct SlewRateFollower;

const SLEW_INPUT: usize = 0;
const SLEW_OUTPUT: usize = 1;

fn slew_rate_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(CmError::EvaluationError(format!(
            "slew: {name} must be finite, got {value}"
        )));
    }
    Ok(value)
}

fn slew_step(
    input: Value,
    old_input: Value,
    old_output: Value,
    dt: Value,
    rise_slope: Value,
    fall_slope: Value,
) -> (Value, Value) {
    if dt <= 0.0 || !dt.is_finite() {
        return (input, 1.0);
    }

    let slope = (input - old_input) / dt;
    if slope >= 0.0 {
        let out_slew = old_output + rise_slope * dt;
        if input < old_output - fall_slope * dt {
            (old_output - fall_slope * dt, 0.0)
        } else if slope > rise_slope || (slope < rise_slope && out_slew <= input) {
            (out_slew, 0.0)
        } else {
            (input, 1.0)
        }
    } else {
        let out_slew = old_output - fall_slope * dt;
        if input > old_output + rise_slope * dt {
            (old_output + rise_slope * dt, 0.0)
        } else if -slope > fall_slope || (-slope < fall_slope && out_slew > input) {
            (out_slew, 0.0)
        } else {
            (input, 1.0)
        }
    }
}

impl CodeModel for SlewRateFollower {
    fn name(&self) -> &str {
        "slew"
    }

    fn description(&self) -> &str {
        "Slew-rate limited follower"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                broad_scalar_analog_input_port("in", "Analog input"),
                scalar_analog_output_port("out", "Slewed output"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("rise_slope", 1.0e-9).with_description("Rising slew limit"),
                ParamSpec::real("fall_slope", 1.0e-9).with_description("Falling slew limit"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        slew_rate_param(ctx, "rise_slope")?;
        slew_rate_param(ctx, "fall_slope")?;
        ctx.allocate_states(2);
        ctx.set_initial_state(SLEW_INPUT, 0.0);
        ctx.set_initial_state(SLEW_OUTPUT, 0.0);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input("in");
        let rise_slope = slew_rate_param(ctx, "rise_slope")?;
        let fall_slope = slew_rate_param(ctx, "fall_slope")?;

        let (output, partial) = if ctx.is_dc() || ctx.time == 0.0 {
            (input, 1.0)
        } else {
            slew_step(
                input,
                ctx.state_prev(SLEW_INPUT),
                ctx.state_prev(SLEW_OUTPUT),
                (ctx.time - ctx.time_prev).max(0.0),
                rise_slope,
                fall_slope,
            )
        };

        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            if ctx.is_dc() || ctx.time == 0.0 {
                ctx.set_initial_state(SLEW_INPUT, input);
                ctx.set_initial_state(SLEW_OUTPUT, output);
            } else {
                ctx.set_state(SLEW_INPUT, input);
                ctx.set_state(SLEW_OUTPUT, output);
            }
        }
        ctx.set_output_with_partial("out", output, partial);
        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
        } else {
            Vec::new()
        }
    }
}

//=============================================================================
// Analog Delay Line
//=============================================================================

/// Analog delay line compatible with the official XSPICE `delay` interface.
#[derive(Debug, Default)]
pub struct AnalogDelayLine;

const DELAY_WRITE_INDEX: usize = 0;
const DELAY_COUNT: usize = 1;
const DELAY_INITIALIZED: usize = 2;
const DELAY_TIME_EPSILON: Value = 1.0e-18;

fn delay_buffer_size(ctx: &CmContext) -> CmResult<usize> {
    let value = if ctx.param_was_provided("buffer_size") {
        ctx.param("buffer_size")
    } else if let (Some(tstop), Some(tstep)) =
        (ctx.transient_stop_time(), ctx.transient_step_hint())
    {
        (tstop / tstep).trunc() + 1.0
    } else {
        ctx.param("buffer_size")
    };
    if !value.is_finite() {
        return Err(CmError::EvaluationError(format!(
            "delay: buffer_size must be finite, got {value}"
        )));
    }
    Ok(value.round().max(1.0) as usize)
}

fn delay_state_count(buffer_size: usize) -> CmResult<usize> {
    buffer_size
        .checked_mul(2)
        .ok_or_else(|| CmError::EvaluationError("delay: buffer_size is too large".to_string()))
}

fn controlled_delay_bound(ctx: &CmContext, value: Value) -> Value {
    let mut bound = value.max(0.0);
    if let Some(tstop) = ctx.transient_stop_time()
        && bound > tstop
    {
        bound = tstop;
    }
    bound
}

fn delay_time_slot(index: usize) -> usize {
    index
}

fn delay_value_slot(buffer_size: usize, index: usize) -> usize {
    buffer_size + index
}

fn newest_delay_index(write_index: usize, count: usize, buffer_size: usize) -> usize {
    if count == 0 {
        0
    } else {
        (write_index + buffer_size - 1) % buffer_size
    }
}

fn oldest_delay_index(write_index: usize, count: usize, buffer_size: usize) -> usize {
    if count < buffer_size { 0 } else { write_index }
}

fn delay_ring_index(oldest: usize, offset: usize, buffer_size: usize) -> usize {
    (oldest + offset) % buffer_size
}

fn delay_bracketing_indices(
    ctx: &CmContext,
    buffer_size: usize,
    oldest: usize,
    count: usize,
    target_time: Value,
) -> (usize, usize) {
    let mut low = 1;
    let mut high = count - 1;
    while low < high {
        let mid = low + (high - low) / 2;
        let current = delay_ring_index(oldest, mid, buffer_size);
        if target_time <= ctx.state(delay_time_slot(current)) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }

    (
        delay_ring_index(oldest, low - 1, buffer_size),
        delay_ring_index(oldest, low, buffer_size),
    )
}

fn append_delay_sample(ctx: &mut CmContext, buffer_size: usize, value: Value) {
    if ctx.int_state(DELAY_INITIALIZED) == 0 {
        ctx.set_state(delay_time_slot(0), ctx.time);
        ctx.set_state(delay_value_slot(buffer_size, 0), value);
        ctx.set_int_state(DELAY_WRITE_INDEX, if buffer_size == 1 { 0 } else { 1 });
        ctx.set_int_state(DELAY_COUNT, 1);
        ctx.set_int_state(DELAY_INITIALIZED, 1);
        return;
    }

    let write_index = ctx.int_state(DELAY_WRITE_INDEX) as usize;
    let count = ctx.int_state(DELAY_COUNT) as usize;
    let newest = newest_delay_index(write_index, count, buffer_size);
    let newest_time = ctx.state(delay_time_slot(newest));

    if (ctx.time - newest_time).abs() <= DELAY_TIME_EPSILON {
        ctx.set_state(delay_value_slot(buffer_size, newest), value);
    } else if ctx.time > newest_time {
        ctx.set_state(delay_time_slot(write_index), ctx.time);
        ctx.set_state(delay_value_slot(buffer_size, write_index), value);
        ctx.set_int_state(DELAY_WRITE_INDEX, ((write_index + 1) % buffer_size) as i64);
        ctx.set_int_state(DELAY_COUNT, count.saturating_add(1).min(buffer_size) as i64);
    }
}

fn interpolate_delay_history(ctx: &CmContext, buffer_size: usize, target_time: Value) -> Value {
    let count = ctx.int_state(DELAY_COUNT) as usize;
    if count == 0 {
        return 0.0;
    }

    let write_index = ctx.int_state(DELAY_WRITE_INDEX) as usize;
    let oldest = oldest_delay_index(write_index, count, buffer_size);
    let first = oldest;
    let first_time = ctx.state(delay_time_slot(first));
    let first_value = ctx.state(delay_value_slot(buffer_size, first));
    if count == 1 || target_time <= first_time {
        return first_value;
    }

    let newest = newest_delay_index(write_index, count, buffer_size);
    let newest_time = ctx.state(delay_time_slot(newest));
    let newest_value = ctx.state(delay_value_slot(buffer_size, newest));
    if target_time >= newest_time {
        return newest_value;
    }
    if !target_time.is_finite() {
        return newest_value;
    }

    let (prev, current) = delay_bracketing_indices(ctx, buffer_size, oldest, count, target_time);
    let prev_time = ctx.state(delay_time_slot(prev));
    let current_time = ctx.state(delay_time_slot(current));
    let prev_value = ctx.state(delay_value_slot(buffer_size, prev));
    let current_value = ctx.state(delay_value_slot(buffer_size, current));
    let span = current_time - prev_time;
    if span.abs() <= Value::EPSILON {
        return current_value;
    }
    let alpha = (target_time - prev_time) / span;
    prev_value + alpha * (current_value - prev_value)
}

fn effective_delay(ctx: &CmContext) -> Value {
    let delay = if ctx.param("has_delay_cnt") > 0.5 {
        let raw_delmax_provided = ctx.param_was_provided("delmax");
        let raw_delmax = if raw_delmax_provided {
            ctx.param("delmax")
        } else {
            ctx.transient_stop_time()
                .unwrap_or_else(|| ctx.param("delmax"))
        };
        let delmin = if raw_delmax_provided && raw_delmax < 0.0 {
            // ngspice's cm_delay negative-delmax branch resets tdelmin before
            // reconciling max < min, so both bounds collapse to zero.
            0.0
        } else {
            controlled_delay_bound(ctx, ctx.param("delmin"))
        };
        let delmax = controlled_delay_bound(ctx, raw_delmax).max(delmin);
        let control = ctx.input("cntrl").clamp(0.0, 1.0);
        delmin + (delmax - delmin) * control
    } else {
        ctx.param("delay").max(0.0)
    };

    let cutoff_step = ctx.transient_step_hint().unwrap_or(ctx.timestep);
    if cutoff_step.is_finite() && cutoff_step > 0.0 && delay < cutoff_step {
        0.0
    } else {
        delay
    }
}

impl CodeModel for AnalogDelayLine {
    fn name(&self) -> &str {
        "delay"
    }

    fn description(&self) -> &str {
        "Analog delay line"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                scalar_analog_port(
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    vec![
                        PortType::Voltage,
                        PortType::DifferentialVoltage,
                        PortType::VoltageName,
                    ],
                    false,
                    "Analog input",
                ),
                scalar_analog_port(
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    vec![PortType::Voltage, PortType::DifferentialVoltage],
                    false,
                    "Delayed output",
                ),
                scalar_analog_port(
                    "cntrl",
                    PortDirection::In,
                    PortType::Voltage,
                    vec![
                        PortType::Voltage,
                        PortType::DifferentialVoltage,
                        PortType::Current,
                        PortType::DifferentialCurrent,
                    ],
                    true,
                    "Delay control input",
                ),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("delay", 0.0).with_description("Delay time"),
                ParamSpec::integer("buffer_size", 1024)
                    .with_description("Delay history buffer size, clamped to at least 1"),
                ParamSpec::boolean("has_delay_cnt", false)
                    .with_description("Use the control input for delay time"),
                ParamSpec::real("delmin", 0.0)
                    .with_description("Minimum controlled delay, clamped to nonnegative"),
                ParamSpec::real("delmax", 0.0)
                    .with_description("Maximum controlled delay, clamped to nonnegative"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        let buffer_size = delay_buffer_size(ctx)?;
        ctx.allocate_states(delay_state_count(buffer_size)?);
        ctx.allocate_int_states(3);
        ctx.set_int_state(DELAY_WRITE_INDEX, 0);
        ctx.set_int_state(DELAY_COUNT, 0);
        ctx.set_int_state(DELAY_INITIALIZED, 0);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input("in");
        if ctx.is_dc() || ctx.is_ac() {
            ctx.set_output_with_partial("out", input, 1.0);
            return Ok(());
        }

        let buffer_size = delay_buffer_size(ctx)?;
        ctx.allocate_states(delay_state_count(buffer_size)?);
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            append_delay_sample(ctx, buffer_size, input);
        }

        let delay = effective_delay(ctx);
        let output = if delay <= DELAY_TIME_EPSILON {
            input
        } else {
            interpolate_delay_history(ctx, buffer_size, ctx.time - delay)
        };
        let partial = if delay <= DELAY_TIME_EPSILON {
            1.0
        } else {
            0.0
        };
        ctx.set_output_with_partial("out", output, partial);
        Ok(())
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
        } else {
            Vec::new()
        }
    }
}

//=============================================================================
// Analog State Return
//=============================================================================

/// Analog state return compatible with the official XSPICE `astate` interface.
#[derive(Debug, Default)]
pub struct AnalogStateReturn;

const ASTATE_HISTORY_DEPTH: usize = 3;

fn astate_number(ctx: &CmContext) -> CmResult<usize> {
    let value = ctx.param("astate_no");
    if !value.is_finite() {
        return Err(CmError::EvaluationError(format!(
            "astate: astate_no must be finite, got {value}"
        )));
    }

    let number = value.round().clamp(0.0, ASTATE_HISTORY_DEPTH as Value);
    Ok(number as usize)
}

fn update_astate_history(ctx: &mut CmContext, input: Value) {
    ctx.set_state(2, ctx.state_prev(1));
    ctx.set_state(1, ctx.state_prev(0));
    ctx.set_state(0, input);
}

impl CodeModel for AnalogStateReturn {
    fn name(&self) -> &str {
        "astate"
    }

    fn description(&self) -> &str {
        "Analog state return"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                scalar_analog_port(
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    vec![
                        PortType::Voltage,
                        PortType::DifferentialVoltage,
                        PortType::VoltageName,
                        PortType::Current,
                        PortType::DifferentialCurrent,
                    ],
                    false,
                    "Analog input",
                ),
                scalar_analog_output_port("out", "Selected previous analog input state"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::integer("astate_no", 1)
                    .with_description("Previous input state number to return, clamped to 0..3"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(ASTATE_HISTORY_DEPTH);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let input = ctx.input("in");
        let state_number = astate_number(ctx)?;

        if ctx.is_dc() || ctx.is_ac() || state_number == 0 {
            ctx.set_output_with_partial("out", input, 1.0);
            return Ok(());
        }

        let output = ctx.state_prev(state_number - 1);
        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            update_astate_history(ctx, input);
        }
        ctx.set_output_with_partial("out", output, 0.0);
        Ok(())
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
        } else {
            Vec::new()
        }
    }
}

//=============================================================================
// One-Shot Pulse Source
//=============================================================================

/// Analog one-shot compatible with the official XSPICE `oneshot` interface.
#[derive(Debug, Default)]
pub struct AnalogOneShot;

const ONESHOT_T1: usize = 0;
const ONESHOT_T2: usize = 1;
const ONESHOT_T3: usize = 2;
const ONESHOT_T4: usize = 3;
const ONESHOT_SET: usize = 4;
const ONESHOT_STATE: usize = 5;
const ONESHOT_CLOCK: usize = 6;
const ONESHOT_OUTPUT_OLD: usize = 7;
const ONESHOT_LOCKED: usize = 8;
const ONESHOT_TRAN_INIT: usize = 9;
const ONESHOT_STATE_COUNT: usize = 10;
const ONESHOT_MIN_EDGE_TIME: Value = 1.0e-12;
const ONESHOT_TABLE_RESOURCE: &str = "xspice.oneshot.table";
const ONESHOT_TABLE_UNSET_UPPER_INDEX: usize = usize::MAX;
const ONESHOT_TABLE_CURSOR_LINEAR_STEPS: usize = 8;

#[derive(Debug, Clone, Copy)]
struct OneShotPoint {
    control: Value,
    pulse_width: Value,
}

#[derive(Debug)]
struct OneShotTableData {
    points: Vec<OneShotPoint>,
    strictly_increasing_control: bool,
    last_upper_index: AtomicUsize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct OneShotTableSignature {
    controls_revision: Option<u64>,
    widths_revision: Option<u64>,
}

#[derive(Debug, Clone)]
struct OneShotTableResource {
    signature: OneShotTableSignature,
    result: CmResult<Option<Arc<OneShotTableData>>>,
}

fn analog_signal_port(name: &str, direction: PortDirection, nullable: bool) -> PortSpec {
    let allowed_types = match direction {
        PortDirection::In => vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::VoltageName,
            PortType::Current,
            PortType::DifferentialCurrent,
        ],
        PortDirection::Out => vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
        ],
        PortDirection::InOut => vec![PortType::Voltage, PortType::DifferentialVoltage],
    };
    scalar_analog_port(
        name,
        direction,
        PortType::Voltage,
        allowed_types,
        nullable,
        "",
    )
}

fn oneshot_ports() -> &'static [PortSpec] {
    use std::sync::OnceLock;
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            analog_signal_port("clk", PortDirection::In, false).with_description("Clock input"),
            scalar_analog_port(
                "cntl_in",
                PortDirection::In,
                PortType::Voltage,
                vec![
                    PortType::Voltage,
                    PortType::VoltageName,
                    PortType::DifferentialVoltage,
                    PortType::Current,
                    PortType::DifferentialCurrent,
                ],
                true,
                "Pulse-width control input",
            ),
            analog_signal_port("clear", PortDirection::In, true).with_description("Clear input"),
            analog_signal_port("out", PortDirection::Out, false).with_description("Pulse output"),
        ]
    })
}

fn oneshot_params() -> &'static [ParamSpec] {
    use std::sync::OnceLock;
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("cntl_array", vec![0.0, 1.0])
                .with_vector_min_len(2)
                .with_description("Control input table"),
            ParamSpec::real_vector("pw_array", vec![1.0e-6, 0.999_999_9])
                .with_vector_min_len(2)
                .with_description("Pulse-width table"),
            ParamSpec::real("clk_trig", 0.5).with_description("Clock trigger threshold"),
            ParamSpec::boolean("pos_edge_trig", true)
                .with_description("Trigger on positive edge when true"),
            ParamSpec::real("out_low", 0.0).with_description("Low output level"),
            ParamSpec::real("out_high", 1.0).with_description("High output level"),
            ParamSpec::real("rise_time", 1.0e-9).with_description("Output rise time"),
            ParamSpec::real("rise_delay", 1.0e-9)
                .with_description("Delay from trigger to rising edge"),
            ParamSpec::real("fall_delay", 1.0e-9)
                .with_description("Delay from pulse-width end to falling edge"),
            ParamSpec::real("fall_time", 1.0e-9).with_description("Output fall time"),
            ParamSpec::boolean("retrig", false).with_description("Allow retriggering"),
        ]
    })
}

fn oneshot_invalid_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn oneshot_table_signature(ctx: &CmContext) -> OneShotTableSignature {
    OneShotTableSignature {
        controls_revision: ctx.real_vector_param_revision("cntl_array"),
        widths_revision: ctx.real_vector_param_revision("pw_array"),
    }
}

fn oneshot_table_signature_matches(ctx: &CmContext, signature: &OneShotTableSignature) -> bool {
    oneshot_table_signature(ctx) == *signature
}

fn oneshot_table_optional_uncached(ctx: &CmContext) -> CmResult<Option<Vec<OneShotPoint>>> {
    let controls = ctx
        .real_vector_param("cntl_array")
        .ok_or_else(|| CmError::MissingParameter("cntl_array".to_string()))?;
    let widths = ctx
        .real_vector_param("pw_array")
        .ok_or_else(|| CmError::MissingParameter("pw_array".to_string()))?;

    if controls.len() != widths.len() {
        return Ok(None);
    }
    if controls.len() < 2 {
        return Err(oneshot_invalid_param(
            "cntl_array",
            format!(
                "cntl_array and pw_array require at least 2 points, got {}",
                controls.len()
            ),
        ));
    }

    let mut table: Vec<OneShotPoint> = Vec::with_capacity(controls.len());
    for (idx, (&control, &pulse_width)) in controls.iter().zip(widths).enumerate() {
        if !control.is_finite() {
            return Err(oneshot_invalid_param(
                "cntl_array",
                format!("point {idx} must be finite, got {control}"),
            ));
        }
        if !pulse_width.is_finite() {
            return Err(oneshot_invalid_param(
                "pw_array",
                format!("point {idx} must be finite, got {pulse_width}"),
            ));
        }
        table.push(OneShotPoint {
            control,
            pulse_width: pulse_width.max(0.0),
        });
    }

    Ok(Some(table))
}

fn oneshot_table_data(points: Vec<OneShotPoint>) -> OneShotTableData {
    let strictly_increasing_control = points
        .windows(2)
        .all(|pair| pair[0].control < pair[1].control);
    OneShotTableData {
        points,
        strictly_increasing_control,
        last_upper_index: AtomicUsize::new(ONESHOT_TABLE_UNSET_UPPER_INDEX),
    }
}

fn oneshot_table_optional(ctx: &mut CmContext) -> CmResult<Option<Arc<OneShotTableData>>> {
    if let Some(resource) = ctx.resource::<OneShotTableResource>(ONESHOT_TABLE_RESOURCE)
        && oneshot_table_signature_matches(ctx, &resource.signature)
    {
        return resource.result.clone();
    }

    let signature = oneshot_table_signature(ctx);
    let result = oneshot_table_optional_uncached(ctx)
        .map(|table| table.map(oneshot_table_data).map(Arc::new));
    ctx.set_resource(
        ONESHOT_TABLE_RESOURCE,
        Arc::new(OneShotTableResource {
            signature,
            result: result.clone(),
        }),
    );
    result
}

fn interpolate_oneshot_pulse_width_linear_scan(table: &[OneShotPoint], control: Value) -> Value {
    let first = table[0];
    let last = table[table.len() - 1];
    if control <= first.control {
        interpolate_oneshot_segment(table[0], table[1], control).max(0.0)
    } else if control >= last.control {
        interpolate_oneshot_segment(table[table.len() - 2], last, control)
    } else {
        let mut pulse_width = None;
        for window in table.windows(2) {
            let left = window[0];
            let right = window[1];
            if control >= left.control && control < right.control {
                pulse_width = Some(interpolate_oneshot_segment(left, right, control));
            }
        }
        pulse_width.unwrap_or(last.pulse_width)
    }
}

fn oneshot_interval_contains(points: &[OneShotPoint], upper_index: usize, control: Value) -> bool {
    debug_assert!(upper_index > 0);
    debug_assert!(upper_index < points.len());
    points[upper_index - 1].control <= control && control <= points[upper_index].control
}

fn oneshot_upper_index_binary(points: &[OneShotPoint], control: Value) -> usize {
    points.partition_point(|point| point.control <= control)
}

fn oneshot_upper_index_with_cursor(table: &OneShotTableData, control: Value) -> usize {
    let points = table.points.as_slice();
    let point_count = points.len();
    let mut upper_index = table.last_upper_index.load(Ordering::Relaxed);

    if upper_index == ONESHOT_TABLE_UNSET_UPPER_INDEX
        || upper_index == 0
        || upper_index >= point_count
    {
        upper_index = oneshot_upper_index_binary(points, control);
        table.last_upper_index.store(upper_index, Ordering::Relaxed);
        return upper_index;
    }

    if oneshot_interval_contains(points, upper_index, control) {
        return upper_index;
    }

    let mut steps = 0;
    if control > points[upper_index].control {
        while upper_index + 1 < point_count
            && control > points[upper_index].control
            && steps < ONESHOT_TABLE_CURSOR_LINEAR_STEPS
        {
            upper_index += 1;
            steps += 1;
        }
    } else {
        while upper_index > 1
            && control < points[upper_index - 1].control
            && steps < ONESHOT_TABLE_CURSOR_LINEAR_STEPS
        {
            upper_index -= 1;
            steps += 1;
        }
    }

    if !oneshot_interval_contains(points, upper_index, control) {
        upper_index = oneshot_upper_index_binary(points, control);
    }
    table.last_upper_index.store(upper_index, Ordering::Relaxed);
    upper_index
}

fn interpolate_oneshot_pulse_width(table: &OneShotTableData, control: Value) -> Value {
    let points = table.points.as_slice();
    if !table.strictly_increasing_control {
        return interpolate_oneshot_pulse_width_linear_scan(points, control);
    }

    let first = points[0];
    let last = points[points.len() - 1];
    if control <= first.control {
        interpolate_oneshot_segment(points[0], points[1], control).max(0.0)
    } else if control >= last.control {
        interpolate_oneshot_segment(points[points.len() - 2], last, control)
    } else {
        let upper = oneshot_upper_index_with_cursor(table, control);
        interpolate_oneshot_segment(points[upper - 1], points[upper], control)
    }
}

fn interpolate_oneshot_segment(left: OneShotPoint, right: OneShotPoint, control: Value) -> Value {
    let span = right.control - left.control;
    if span.abs() <= Value::EPSILON {
        return left.pulse_width;
    }
    let alpha = (control - left.control) / span;
    left.pulse_width + alpha * (right.pulse_width - left.pulse_width)
}

fn oneshot_levels(ctx: &CmContext) -> CmResult<(Value, Value)> {
    let low = ctx.param("out_low");
    let high = ctx.param("out_high");
    if !low.is_finite() {
        return Err(oneshot_invalid_param(
            "out_low",
            format!("value must be finite, got {low}"),
        ));
    }
    if !high.is_finite() {
        return Err(oneshot_invalid_param(
            "out_high",
            format!("value must be finite, got {high}"),
        ));
    }
    Ok((low, high))
}

fn oneshot_finite_param(ctx: &CmContext, name: &str) -> CmResult<Value> {
    let value = ctx.param(name);
    if !value.is_finite() {
        return Err(oneshot_invalid_param(
            name,
            format!("value must be finite, got {value}"),
        ));
    }
    Ok(value)
}

fn reset_oneshot_state(ctx: &mut CmContext, output_low: Value) {
    ctx.set_state(ONESHOT_T1, -1.0);
    ctx.set_state(ONESHOT_T2, -1.0);
    ctx.set_state(ONESHOT_T3, -1.0);
    ctx.set_state(ONESHOT_T4, -1.0);
    ctx.set_state(ONESHOT_SET, 0.0);
    ctx.set_state(ONESHOT_STATE, 0.0);
    ctx.set_state(ONESHOT_CLOCK, 0.0);
    ctx.set_state(ONESHOT_OUTPUT_OLD, output_low);
    ctx.set_state(ONESHOT_LOCKED, 0.0);
    ctx.set_state(ONESHOT_TRAN_INIT, 0.0);
}

fn request_oneshot_breakpoints(ctx: &mut CmContext, times: &[Value]) {
    for &time in times {
        if time >= ctx.time - 1.0e-18 {
            ctx.request_breakpoint(time);
        }
    }
}

fn oneshot_output_below_or_at_official(value: Value, target: Value) -> bool {
    value - target < 1.0e-20
}

fn oneshot_edge_time(
    ctx: &CmContext,
    old_clock: Value,
    clock: Value,
    trigger: Value,
    has_previous_sample: bool,
) -> Value {
    if !has_previous_sample
        || !ctx.time_prev.is_finite()
        || !ctx.time.is_finite()
        || ctx.time <= ctx.time_prev
        || !old_clock.is_finite()
        || !clock.is_finite()
    {
        return ctx.time;
    }

    let delta_clock = clock - old_clock;
    if delta_clock.abs() <= Value::EPSILON {
        return ctx.time;
    }

    let alpha = ((trigger - old_clock) / delta_clock).clamp(0.0, 1.0);
    ctx.time_prev + alpha * (ctx.time - ctx.time_prev)
}

impl CodeModel for AnalogOneShot {
    fn name(&self) -> &str {
        "oneshot"
    }

    fn description(&self) -> &str {
        "Analog one-shot pulse generator"
    }

    fn ports(&self) -> &[PortSpec] {
        oneshot_ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        oneshot_params()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        if oneshot_table_optional(ctx)?.is_none() {
            return Ok(());
        }
        let (output_low, _) = oneshot_levels(ctx)?;
        for name in [
            "clk_trig",
            "rise_delay",
            "fall_delay",
            "rise_time",
            "fall_time",
        ] {
            oneshot_finite_param(ctx, name)?;
        }
        ctx.allocate_states(ONESHOT_STATE_COUNT);
        reset_oneshot_state(ctx, output_low);
        for index in 0..ONESHOT_STATE_COUNT {
            ctx.set_initial_state(index, ctx.state(index));
        }
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let Some(table) = oneshot_table_optional(ctx)? else {
            return Ok(());
        };
        let (output_low, output_high) = oneshot_levels(ctx)?;
        let trigger = oneshot_finite_param(ctx, "clk_trig")?;
        let rise_delay = oneshot_finite_param(ctx, "rise_delay")?;
        let fall_delay = oneshot_finite_param(ctx, "fall_delay")?;
        let rise_time = oneshot_finite_param(ctx, "rise_time")?.max(ONESHOT_MIN_EDGE_TIME);
        let fall_time = oneshot_finite_param(ctx, "fall_time")?.max(ONESHOT_MIN_EDGE_TIME);

        if ctx.is_dc() {
            reset_oneshot_state(ctx, output_low);
            ctx.set_output_with_partial("out", output_low, 0.0);
            return Ok(());
        }
        if ctx.is_ac() {
            ctx.set_output_with_partial("out", 0.0, 0.0);
            return Ok(());
        }

        let mut time1 = ctx.state_prev(ONESHOT_T1);
        let mut time2 = ctx.state_prev(ONESHOT_T2);
        let mut time3 = ctx.state_prev(ONESHOT_T3);
        let mut time4 = ctx.state_prev(ONESHOT_T4);
        let mut set = ctx.state_prev(ONESHOT_SET) > 0.5;
        let mut state = ctx.state_prev(ONESHOT_STATE) > 0.5;
        let mut locked = ctx.state_prev(ONESHOT_LOCKED) > 0.5;
        let previous_output = ctx.state_prev(ONESHOT_OUTPUT_OLD);
        let has_previous_sample = ctx.state_prev(ONESHOT_TRAN_INIT) > 0.5;
        let old_clock = if has_previous_sample {
            ctx.state_prev(ONESHOT_CLOCK)
        } else {
            0.0
        };
        let clock = ctx.input("clk");
        let commit_state = ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe;
        let mut trigger_time = ctx.time;

        let mut output = output_low;
        if ctx.port_width("clear") > 0 && ctx.input("clear") > trigger {
            time1 = -1.0;
            time2 = -1.0;
            time3 = -1.0;
            time4 = -1.0;
            set = false;
            state = false;
            locked = false;
        } else {
            let control = if ctx.port_width("cntl_in") > 0 {
                ctx.input("cntl_in")
            } else {
                0.0
            };
            let pulse_width = interpolate_oneshot_pulse_width(table.as_ref(), control);
            let positive_edge = ctx.param("pos_edge_trig") > 0.5;
            let retrigger = ctx.param("retrig") > 0.5;

            if positive_edge {
                if !set {
                    if clock > old_clock && clock > trigger {
                        trigger_time =
                            oneshot_edge_time(ctx, old_clock, clock, trigger, has_previous_sample);
                        state = true;
                        set = true;
                    }
                } else if clock < old_clock && clock < trigger {
                    set = false;
                }
            } else if !set {
                if clock < old_clock && clock < trigger {
                    trigger_time =
                        oneshot_edge_time(ctx, old_clock, clock, trigger, has_previous_sample);
                    state = true;
                    set = true;
                }
            } else if clock > old_clock && clock > trigger {
                set = false;
            }

            if state && oneshot_output_below_or_at_official(previous_output, output_low) && !locked
            {
                time1 = trigger_time + rise_delay;
                time2 = time1 + rise_time;
                time3 = time2 + pulse_width + fall_delay;
                time4 = time3 + fall_time;
                if !retrigger {
                    locked = true;
                }
                if commit_state {
                    request_oneshot_breakpoints(ctx, &[time1, time2, time3, time4]);
                }
                state = false;
            } else if state
                && oneshot_output_below_or_at_official(previous_output, output_high)
                && !locked
            {
                time3 = trigger_time + pulse_width + rise_delay + fall_delay + rise_time;
                time4 = time3 + fall_time;
                if commit_state {
                    request_oneshot_breakpoints(ctx, &[time3, time4]);
                }
                state = false;
            }

            if state && locked {
                state = false;
            }

            if ctx.time < time1 {
                output = output_low;
            } else if time1 <= ctx.time && ctx.time < time2 {
                output = output_low
                    + ((ctx.time - time1) / (time2 - time1)) * (output_high - output_low);
            } else if time2 <= ctx.time && ctx.time < time3 {
                output = output_high;
            } else if time3 <= ctx.time && ctx.time < time4 {
                output = output_high
                    + ((ctx.time - time3) / (time4 - time3)) * (output_low - output_high);
            } else {
                output = output_low;
                if !retrigger {
                    locked = false;
                }
            }
        }

        if commit_state {
            ctx.set_state(ONESHOT_T1, time1);
            ctx.set_state(ONESHOT_T2, time2);
            ctx.set_state(ONESHOT_T3, time3);
            ctx.set_state(ONESHOT_T4, time4);
            ctx.set_state(ONESHOT_SET, if set { 1.0 } else { 0.0 });
            ctx.set_state(ONESHOT_STATE, if state { 1.0 } else { 0.0 });
            ctx.set_state(ONESHOT_CLOCK, clock);
            ctx.set_state(ONESHOT_OUTPUT_OLD, output);
            ctx.set_state(ONESHOT_LOCKED, if locked { 1.0 } else { 0.0 });
            ctx.set_state(ONESHOT_TRAN_INIT, 1.0);
        }
        ctx.set_output_with_partial("out", output, 0.0);
        Ok(())
    }

    fn ac_gain(&self, _ctx: &CmContext) -> Vec<Value> {
        vec![0.0]
    }

    fn excludes_output_from_transient_voltage_lte(&self, output_port: &str) -> bool {
        output_port.eq_ignore_ascii_case("out")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::xspice::AnalogValue;
    use crate::xspice::ParamType;

    fn param_summary(model: &dyn CodeModel) -> Vec<(&str, ParamType, Value, bool)> {
        model
            .parameters()
            .iter()
            .map(|param| {
                (
                    param.name.as_str(),
                    param.param_type,
                    param.default,
                    param.required,
                )
            })
            .collect()
    }

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

    fn v_vd_i_id_vnam_types() -> Vec<PortType> {
        vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
            PortType::VoltageName,
        ]
    }

    fn v_vd_vnam_i_id_types() -> Vec<PortType> {
        vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::VoltageName,
            PortType::Current,
            PortType::DifferentialCurrent,
        ]
    }

    fn v_vnam_vd_i_id_types() -> Vec<PortType> {
        vec![
            PortType::Voltage,
            PortType::VoltageName,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
        ]
    }

    fn v_vd_i_id_types() -> Vec<PortType> {
        vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::Current,
            PortType::DifferentialCurrent,
        ]
    }

    fn v_vd_vnam_types() -> Vec<PortType> {
        vec![
            PortType::Voltage,
            PortType::DifferentialVoltage,
            PortType::VoltageName,
        ]
    }

    fn v_vd_types() -> Vec<PortType> {
        vec![PortType::Voltage, PortType::DifferentialVoltage]
    }

    fn assert_analog_ports(
        model: &dyn CodeModel,
        expected: &[(&str, PortDirection, bool, Option<usize>)],
    ) {
        let ports = model.ports();
        assert_eq!(
            ports
                .iter()
                .map(|port| port.name.as_str())
                .collect::<Vec<_>>(),
            expected
                .iter()
                .map(|(name, _, _, _)| *name)
                .collect::<Vec<_>>()
        );
        for (port, (_, direction, is_vector, min_len)) in ports.iter().zip(expected) {
            assert_eq!(port.direction, *direction, "{} direction", port.name);
            assert_eq!(
                port.default_type,
                PortType::Voltage,
                "{} default type",
                port.name
            );
            assert_eq!(port.is_vector, *is_vector, "{} vector flag", port.name);
            assert!(!port.null_allowed, "{} nullability", port.name);
            assert_eq!(port.vector_min_len, *min_len, "{} min length", port.name);
            assert_eq!(port.vector_max_len, None, "{} max length", port.name);

            if *direction == PortDirection::Out {
                assert_eq!(
                    port.allowed_types,
                    vec![
                        PortType::Voltage,
                        PortType::DifferentialVoltage,
                        PortType::Current,
                        PortType::DifferentialCurrent,
                    ],
                    "{} output allowed types",
                    port.name
                );
            } else {
                assert_eq!(
                    port.allowed_types,
                    vec![
                        PortType::Voltage,
                        PortType::DifferentialVoltage,
                        PortType::Current,
                        PortType::DifferentialCurrent,
                        PortType::VoltageName,
                    ],
                    "{} input allowed types",
                    port.name
                );
            }
        }
    }

    #[test]
    fn gain_metadata_matches_ngspice46_interface() {
        assert_analog_ports(
            &Gain,
            &[
                ("in", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&Gain),
            vec![
                ("in_offset", ParamType::Real, 0.0, false),
                ("gain", ParamType::Real, 1.0, false),
                ("out_offset", ParamType::Real, 0.0, false),
            ]
        );
    }

    #[test]
    fn vector_arithmetic_metadata_matches_ngspice46_interfaces() {
        for model in [&Summer as &dyn CodeModel, &Multiplier] {
            assert_analog_ports(
                model,
                &[
                    ("in", PortDirection::In, true, Some(2)),
                    ("out", PortDirection::Out, false, None),
                ],
            );
            assert_eq!(
                param_summary(model),
                vec![
                    ("in_offset", ParamType::RealVector, 0.0, false),
                    ("in_gain", ParamType::RealVector, 0.0, false),
                    ("out_gain", ParamType::Real, 1.0, false),
                    ("out_offset", ParamType::Real, 0.0, false),
                ],
                "{}",
                model.name()
            );
        }
    }

    #[test]
    fn divide_metadata_matches_ngspice46_interface() {
        assert_eq!(DivideAlias.name(), "divide");
        assert_analog_ports(
            &DivideAlias,
            &[
                ("num", PortDirection::In, false, None),
                ("den", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&DivideAlias),
            vec![
                ("num_offset", ParamType::Real, 0.0, false),
                ("num_gain", ParamType::Real, 1.0, false),
                ("den_offset", ParamType::Real, 0.0, false),
                ("den_gain", ParamType::Real, 1.0, false),
                ("den_lower_limit", ParamType::Real, 1.0e-10, false),
                ("den_domain", ParamType::Real, 1.0e-16, false),
                ("fraction", ParamType::Boolean, 0.0, false),
                ("out_gain", ParamType::Real, 1.0, false),
                ("out_offset", ParamType::Real, 0.0, false),
            ]
        );
    }

    #[test]
    fn limiter_metadata_matches_ngspice46_interfaces() {
        assert_analog_ports(
            &Limiter,
            &[
                ("in", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&Limiter),
            vec![
                ("in_offset", ParamType::Real, 0.0, false),
                ("gain", ParamType::Real, 1.0, false),
                ("out_lower_limit", ParamType::Real, -1.0e12, true),
                ("out_upper_limit", ParamType::Real, 1.0e12, true),
                ("limit_range", ParamType::Real, 1.0e-6, false),
                ("fraction", ParamType::Boolean, 0.0, false),
            ]
        );

        assert_analog_ports(
            &ControlledLimiter,
            &[
                ("in", PortDirection::In, false, None),
                ("cntl_upper", PortDirection::In, false, None),
                ("cntl_lower", PortDirection::In, false, None),
                ("out", PortDirection::Out, false, None),
            ],
        );
        assert_eq!(
            param_summary(&ControlledLimiter),
            vec![
                ("in_offset", ParamType::Real, 0.0, false),
                ("gain", ParamType::Real, 1.0, false),
                ("upper_delta", ParamType::Real, 0.0, false),
                ("lower_delta", ParamType::Real, 0.0, false),
                ("limit_range", ParamType::Real, 1.0e-6, false),
                ("fraction", ParamType::Boolean, 0.0, false),
            ]
        );
    }

    #[test]
    fn analog_dynamic_metadata_matches_ngspice46_interfaces() {
        assert_eq!(
            port_summary(&HysteresisBlock),
            vec![
                (
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_i_id_vnam_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    v_vd_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
            ]
        );
        assert_eq!(
            param_summary(&HysteresisBlock),
            vec![
                ("in_low", ParamType::Real, 0.0, false),
                ("in_high", ParamType::Real, 1.0, false),
                ("hyst", ParamType::Real, 0.1, false),
                ("out_lower_limit", ParamType::Real, 0.0, false),
                ("out_upper_limit", ParamType::Real, 1.0, false),
                ("input_domain", ParamType::Real, 0.01, false),
                ("fraction", ParamType::Boolean, 1.0, false),
            ]
        );

        assert_eq!(
            port_summary(&SlewRateFollower),
            vec![
                (
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_i_id_vnam_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    v_vd_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
            ]
        );
        assert_eq!(
            param_summary(&SlewRateFollower),
            vec![
                ("rise_slope", ParamType::Real, 1.0e-9, false),
                ("fall_slope", ParamType::Real, 1.0e-9, false),
            ]
        );

        assert_eq!(
            port_summary(&AnalogDelayLine),
            vec![
                (
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_vnam_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    v_vd_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "cntrl",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_i_id_types(),
                    false,
                    true,
                    None,
                    None,
                ),
            ]
        );
        assert_eq!(
            param_summary(&AnalogDelayLine),
            vec![
                ("delay", ParamType::Real, 0.0, false),
                ("buffer_size", ParamType::Integer, 1024.0, false),
                ("has_delay_cnt", ParamType::Boolean, 0.0, false),
                ("delmin", ParamType::Real, 0.0, false),
                ("delmax", ParamType::Real, 0.0, false),
            ]
        );

        assert_eq!(
            port_summary(&AnalogStateReturn),
            vec![
                (
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_vnam_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    v_vd_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
            ]
        );
        assert_eq!(
            param_summary(&AnalogStateReturn),
            vec![("astate_no", ParamType::Integer, 1.0, false)]
        );
    }

    #[test]
    fn oneshot_metadata_matches_ngspice46_interface() {
        assert_eq!(
            port_summary(&AnalogOneShot),
            vec![
                (
                    "clk",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_vnam_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "cntl_in",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vnam_vd_i_id_types(),
                    false,
                    true,
                    None,
                    None,
                ),
                (
                    "clear",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_vnam_i_id_types(),
                    false,
                    true,
                    None,
                    None,
                ),
                (
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    v_vd_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
            ]
        );

        let params = AnalogOneShot.parameters();
        assert_eq!(
            param_summary(&AnalogOneShot),
            vec![
                ("cntl_array", ParamType::RealVector, 0.0, false),
                ("pw_array", ParamType::RealVector, 0.0, false),
                ("clk_trig", ParamType::Real, 0.5, false),
                ("pos_edge_trig", ParamType::Boolean, 1.0, false),
                ("out_low", ParamType::Real, 0.0, false),
                ("out_high", ParamType::Real, 1.0, false),
                ("rise_time", ParamType::Real, 1.0e-9, false),
                ("rise_delay", ParamType::Real, 1.0e-9, false),
                ("fall_delay", ParamType::Real, 1.0e-9, false),
                ("fall_time", ParamType::Real, 1.0e-9, false),
                ("retrig", ParamType::Boolean, 0.0, false),
            ]
        );
        assert_eq!(
            params[0].real_vector_default.as_deref(),
            Some(&[0.0, 1.0][..])
        );
        assert_eq!(params[0].vector_min_len, Some(2));
        assert_eq!(
            params[1].real_vector_default.as_deref(),
            Some(&[1.0e-6, 0.999_999_9][..])
        );
        assert_eq!(params[1].vector_min_len, Some(2));
    }

    #[test]
    fn analog_integrator_metadata_matches_ngspice46_interfaces() {
        assert_eq!(IntegratorAlias.name(), "int");
        assert_eq!(
            port_summary(&IntegratorAlias),
            vec![
                (
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_i_id_vnam_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    v_vd_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
            ]
        );
        assert_eq!(
            param_summary(&IntegratorAlias),
            vec![
                ("in_offset", ParamType::Real, 0.0, false),
                ("gain", ParamType::Real, 1.0, false),
                ("out_lower_limit", ParamType::Real, -1.0e12, true),
                ("out_upper_limit", ParamType::Real, 1.0e12, true),
                ("limit_range", ParamType::Real, 1.0e-6, false),
                ("out_ic", ParamType::Real, 0.0, false),
            ]
        );

        assert_eq!(DifferentiatorAlias.name(), "d_dt");
        assert_eq!(
            port_summary(&DifferentiatorAlias),
            vec![
                (
                    "in",
                    PortDirection::In,
                    PortType::Voltage,
                    v_vd_i_id_vnam_types(),
                    false,
                    false,
                    None,
                    None,
                ),
                (
                    "out",
                    PortDirection::Out,
                    PortType::Voltage,
                    v_vd_i_id_types(),
                    false,
                    false,
                    None,
                    None,
                ),
            ]
        );
        assert_eq!(
            param_summary(&DifferentiatorAlias),
            vec![
                ("out_offset", ParamType::Real, 0.0, false),
                ("gain", ParamType::Real, 1.0, false),
                ("out_lower_limit", ParamType::Real, -1.0e12, true),
                ("out_upper_limit", ParamType::Real, 1.0e12, true),
                ("limit_range", ParamType::Real, 1.0e-6, false),
            ]
        );
    }

    fn oneshot_point(control: Value, pulse_width: Value) -> OneShotPoint {
        OneShotPoint {
            control,
            pulse_width,
        }
    }

    fn mismatched_oneshot_context() -> CmContext {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("pw_array", vec![1.0e-9, 2.0e-9, 3.0e-9]);
        ctx.set_param("clk_trig", 0.5);
        ctx.set_param("out_low", -2.0);
        ctx.set_param("out_high", 3.0);
        ctx.set_param("rise_delay", 1.0e-9);
        ctx.set_param("fall_delay", 1.0e-9);
        ctx.set_param("rise_time", 1.0e-9);
        ctx.set_param("fall_time", 1.0e-9);
        ctx.init_output("out", PortType::Voltage);
        ctx
    }

    #[test]
    fn oneshot_ignores_mismatched_control_tables_like_ngspice() {
        let mut ctx = mismatched_oneshot_context();

        AnalogOneShot
            .init(&mut ctx)
            .expect("ngspice reports mismatched oneshot tables but does not fail init");
        AnalogOneShot
            .evaluate(&mut ctx)
            .expect("ngspice returns without fatal error on mismatched oneshot tables");

        assert_eq!(ctx.output("out"), 0.0);
    }

    #[test]
    fn oneshot_table_cache_reloads_when_params_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("pw_array", vec![1.0e-9, 2.0e-9]);

        let first = oneshot_table_optional(&mut ctx)
            .expect("oneshot table loads")
            .expect("table is present");
        let second = oneshot_table_optional(&mut ctx)
            .expect("oneshot table reloads")
            .expect("table is present");
        assert!(
            Arc::ptr_eq(&first, &second),
            "unchanged oneshot table parameters should reuse the parsed table"
        );
        assert!((interpolate_oneshot_pulse_width(&first, 0.5) - 1.5e-9).abs() < 1.0e-21);

        ctx.set_real_vector_param("unrelated", vec![42.0]);
        let after_unrelated = oneshot_table_optional(&mut ctx)
            .expect("unrelated vector preserves oneshot table")
            .expect("table is present");
        assert!(
            Arc::ptr_eq(&first, &after_unrelated),
            "unrelated vector parameters should not refresh the parsed oneshot table"
        );

        ctx.set_real_vector_param("pw_array", vec![2.0e-9, 4.0e-9]);
        let updated = oneshot_table_optional(&mut ctx)
            .expect("updated oneshot table loads")
            .expect("table is present");
        assert!(
            !Arc::ptr_eq(&first, &updated),
            "changed oneshot table parameters must refresh the parsed table"
        );
        assert!((interpolate_oneshot_pulse_width(&updated, 0.5) - 3.0e-9).abs() < 1.0e-21);
    }

    #[test]
    fn oneshot_interpolation_uses_monotonic_brackets() {
        let table = oneshot_table_data(vec![
            oneshot_point(0.0, 1.0e-9),
            oneshot_point(1.0, 3.0e-9),
            oneshot_point(3.0, 7.0e-9),
        ]);

        assert!(table.strictly_increasing_control);
        assert_eq!(
            table.last_upper_index.load(Ordering::Relaxed),
            ONESHOT_TABLE_UNSET_UPPER_INDEX
        );
        assert!(
            (interpolate_oneshot_pulse_width(&table, 2.0) - 5.0e-9).abs() < 1.0e-21,
            "strictly increasing oneshot controls should interpolate from the binary-search bracket"
        );
        assert_eq!(table.last_upper_index.load(Ordering::Relaxed), 2);
        assert_eq!(
            interpolate_oneshot_pulse_width(&table, 1.0),
            3.0e-9,
            "exact interior controls should return the matching row"
        );
        assert_eq!(
            table.last_upper_index.load(Ordering::Relaxed),
            2,
            "exact controls may reuse the current bracket when the value is unchanged"
        );
    }

    #[test]
    fn oneshot_control_table_cursor_falls_back_for_large_jumps() {
        let table = oneshot_table_data(
            (1..=24)
                .map(|control| oneshot_point(control as Value, control as Value * 1.0e-9))
                .collect(),
        );

        assert!((interpolate_oneshot_pulse_width(&table, 2.5) - 2.5e-9).abs() < 1.0e-21);
        assert_eq!(table.last_upper_index.load(Ordering::Relaxed), 2);
        assert!((interpolate_oneshot_pulse_width(&table, 22.5) - 22.5e-9).abs() < 1.0e-21);
        assert_eq!(
            table.last_upper_index.load(Ordering::Relaxed),
            22,
            "large non-local control jumps should land on the binary-search bracket"
        );
    }

    #[test]
    fn oneshot_interpolation_uses_last_matching_segment_like_ngspice() {
        let table = oneshot_table_data(vec![
            oneshot_point(0.0, 0.0),
            oneshot_point(1.0, 100.0e-9),
            oneshot_point(0.5, 50.0e-9),
            oneshot_point(2.0, 300.0e-9),
        ]);

        assert!(!table.strictly_increasing_control);
        let pulse_width = interpolate_oneshot_pulse_width(&table, 0.75);

        assert!(
            (pulse_width - 91.666_666_666_666_66e-9).abs() < 1.0e-21,
            "ngspice scans every matching oneshot segment and keeps the later pulse width, got {pulse_width}"
        );
    }

    #[test]
    fn oneshot_extrapolation_only_clamps_low_control_path_like_ngspice() {
        let falling_table =
            oneshot_table_data(vec![oneshot_point(0.0, 1.0e-9), oneshot_point(1.0, 0.5e-9)]);
        let rising_table =
            oneshot_table_data(vec![oneshot_point(0.0, 0.5e-9), oneshot_point(1.0, 1.0e-9)]);

        assert!(
            (interpolate_oneshot_pulse_width(&falling_table, -3.0) - 2.5e-9).abs() < 1.0e-21,
            "positive low-control extrapolation should be preserved"
        );
        assert_eq!(
            interpolate_oneshot_pulse_width(&rising_table, -2.0),
            0.0,
            "negative low-control extrapolation should clamp to zero"
        );
        assert!(
            (interpolate_oneshot_pulse_width(&falling_table, 4.0) + 1.0e-9).abs() < 1.0e-21,
            "ngspice only clamps the low-control oneshot pulse-width extrapolation path"
        );
    }

    #[test]
    fn oneshot_interpolation_preserves_linear_scan_for_descending_tables() {
        let table =
            oneshot_table_data(vec![oneshot_point(1.0, 1.0e-9), oneshot_point(0.0, 2.0e-9)]);

        assert!(!table.strictly_increasing_control);
        assert!(
            (interpolate_oneshot_pulse_width(&table, 0.5) - 1.5e-9).abs() < 1.0e-21,
            "descending oneshot controls should keep the original endpoint segment behavior"
        );
    }

    #[test]
    fn oneshot_does_not_commit_rollbackable_probe_trigger() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.time = 1.0e-9;
        ctx.set_real_vector_param("cntl_array", vec![0.0, 1.0]);
        ctx.set_real_vector_param("pw_array", vec![1.0e-9, 1.0e-9]);
        ctx.set_param("clk_trig", 0.5);
        ctx.set_param("pos_edge_trig", 1.0);
        ctx.set_param("retrig", 0.0);
        ctx.set_param("out_low", 0.0);
        ctx.set_param("out_high", 5.0);
        ctx.set_param("rise_delay", 0.2e-9);
        ctx.set_param("rise_time", 0.2e-9);
        ctx.set_param("fall_delay", 0.1e-9);
        ctx.set_param("fall_time", 0.2e-9);
        ctx.init_output("out", PortType::Voltage);

        AnalogOneShot.init(&mut ctx).expect("oneshot init");
        ctx.set_input_analog("clk", 1.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        AnalogOneShot
            .evaluate(&mut ctx)
            .expect("probe oneshot trigger");

        assert_eq!(ctx.state(ONESHOT_T1), -1.0);
        assert_eq!(ctx.state(ONESHOT_T2), -1.0);
        assert_eq!(ctx.state(ONESHOT_CLOCK), 0.0);
        assert_eq!(
            ctx.take_requested_breakpoints(),
            Vec::<Value>::new(),
            "rollbackable oneshot trigger must not leave transient breakpoints behind"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        AnalogOneShot
            .evaluate(&mut ctx)
            .expect("accepted oneshot trigger");

        assert!((ctx.state(ONESHOT_T1) - 1.2e-9).abs() < 1.0e-18);
        assert!((ctx.state(ONESHOT_T2) - 1.4e-9).abs() < 1.0e-18);
        assert_eq!(ctx.state(ONESHOT_CLOCK), 1.0);
        assert_eq!(ctx.take_requested_breakpoints().len(), 4);
    }

    #[test]
    fn delay_does_not_record_rollbackable_probe_samples() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.timestep = 0.5e-9;
        ctx.set_param("delay", 1.0e-9);
        ctx.set_param("buffer_size", 8.0);
        ctx.init_output("out", PortType::Voltage);

        AnalogDelayLine.init(&mut ctx).expect("delay init");

        ctx.time = 0.0;
        ctx.set_input_analog("in", 0.0);
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        AnalogDelayLine
            .evaluate(&mut ctx)
            .expect("record first accepted sample");
        ctx.advance_state();

        ctx.time = 2.0e-9;
        ctx.set_input_analog("in", 2.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        AnalogDelayLine
            .evaluate(&mut ctx)
            .expect("probe future sample");

        ctx.time = 1.0e-9;
        ctx.set_input_analog("in", 1.0);
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        AnalogDelayLine
            .evaluate(&mut ctx)
            .expect("record next accepted sample");

        let recorded_time = ctx.state(delay_time_slot(1));
        let recorded_value = ctx.state(delay_value_slot(8, 1));
        assert!(
            (recorded_time - 1.0e-9).abs() <= 1.0e-18,
            "rollbackable probe poisoned delay history slot with time {recorded_time}"
        );
        assert!(
            (recorded_value - 1.0).abs() <= 1.0e-12,
            "rollbackable probe poisoned delay history slot with value {recorded_value}"
        );
    }

    #[test]
    fn delay_binary_history_lookup_interpolates_wrapped_ring_buffers() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("buffer_size", 4.0);

        AnalogDelayLine.init(&mut ctx).expect("delay init");
        for index in 0..5 {
            ctx.time = index as Value;
            append_delay_sample(&mut ctx, 4, 10.0 * index as Value);
        }

        assert_eq!(ctx.int_state(DELAY_WRITE_INDEX), 1);
        assert_eq!(ctx.int_state(DELAY_COUNT), 4);
        assert_eq!(oldest_delay_index(1, 4, 4), 1);

        let output = interpolate_delay_history(&ctx, 4, 2.5);
        assert!(
            (output - 25.0).abs() <= 1.0e-12,
            "wrapped delay history should interpolate between ring slots 2 and 3, got {output}"
        );
    }

    #[test]
    fn mult_output_and_partials_share_accumulator_math_without_extra_output_work() {
        let mut ctx = CmContext::new();
        ctx.set_input_analog_vector_from_fn("in", 3, |index| {
            AnalogValue::new([1.0, 2.0, 3.0][index])
        });
        ctx.set_real_vector_param("in_gain", vec![2.0, 3.0, 4.0]);
        ctx.set_real_vector_param("in_offset", vec![0.5, -1.0, 0.0]);
        ctx.set_param("out_gain", 2.0);
        ctx.set_param("out_offset", 1.0);

        let output = mult_output_from_context(&ctx).expect("mult output evaluates");
        assert_eq!(output, 217.0);

        let partials = mult_partials_from_context(&ctx).expect("mult partials evaluate");
        assert_eq!(
            partials,
            vec![
                ("in".to_string(), 0, 144.0),
                ("in".to_string(), 1, 216.0),
                ("in".to_string(), 2, 72.0),
            ]
        );
    }

    #[test]
    fn mult_transfer_resource_match_compares_current_input_slice() {
        let mut ctx = CmContext::new();
        ctx.set_input_analog_vector_from_fn("in", 3, |index| {
            AnalogValue::new([1.0, 2.0, 3.0][index])
        });
        ctx.set_real_vector_param("in_gain", vec![2.0, 3.0, 4.0]);
        ctx.set_real_vector_param("in_offset", vec![0.5, -1.0, 0.0]);
        ctx.set_param("out_gain", 2.0);
        ctx.set_param("out_offset", 1.0);

        let signature = mult_transfer_signature(&ctx);
        let base = mult_transfer_base_signature(&ctx);
        let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
        let resource = MultTransferResource {
            signature,
            transfer: Arc::new(MultTransfer {
                accumulate_in: -2.0,
                transfer_gain: -3.0,
                out_offset: -4.0,
                shifted_inputs: vec![-5.0, -6.0, -7.0],
            }),
        };

        assert!(
            mult_transfer_resource_matches(&resource, base, inputs),
            "matching mult inputs should hit the transfer cache without rebuilding a signature"
        );

        ctx.set_input_analog_vector_from_fn("in", 3, |index| {
            AnalogValue::new([1.0, 4.0, 3.0][index])
        });
        let changed_inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
        assert!(
            !mult_transfer_resource_matches(&resource, base, changed_inputs),
            "changed mult inputs must invalidate the transfer cache"
        );
    }

    #[test]
    fn mult_transfer_cache_reuses_evaluated_transfer_until_inputs_change() {
        let mut ctx = CmContext::new();
        ctx.set_input_analog_vector_from_fn("in", 3, |index| {
            AnalogValue::new([1.0, 2.0, 3.0][index])
        });
        ctx.set_real_vector_param("in_gain", vec![2.0, 3.0, 4.0]);
        ctx.set_real_vector_param("in_offset", vec![0.5, -1.0, 0.0]);
        ctx.set_param("out_gain", 2.0);
        ctx.set_param("out_offset", 1.0);

        let first = cache_mult_transfer(&mut ctx).expect("mult transfer caches");
        let cached = mult_transfer_for_context(&ctx).expect("cached mult transfer");
        assert!(
            Arc::ptr_eq(&cached, &first),
            "cached immutable mult transfer should reuse the stored Arc"
        );

        ctx.set_param("unrelated", 42.0);
        let cached_after_unrelated =
            mult_transfer_for_context(&ctx).expect("unrelated param preserves mult cache");
        assert!(
            Arc::ptr_eq(&cached_after_unrelated, &first),
            "unrelated context changes should not invalidate the mult transfer cache"
        );

        let signature = mult_transfer_signature(&ctx);
        let sentinel = MultTransfer {
            accumulate_in: -2.0,
            transfer_gain: -3.0,
            out_offset: -4.0,
            shifted_inputs: vec![-5.0, -6.0, -7.0],
        };
        let sentinel = Arc::new(sentinel);
        ctx.set_resource(
            MULT_TRANSFER_RESOURCE,
            Arc::new(MultTransferResource {
                signature,
                transfer: Arc::clone(&sentinel),
            }),
        );
        let cached_sentinel =
            mult_transfer_for_context(&ctx).expect("matching signature reuses mult cache");
        assert!(
            Arc::ptr_eq(&cached_sentinel, &sentinel),
            "matching signatures should reuse the cached mult transfer"
        );
        let mutable_cached_sentinel =
            cache_mult_transfer(&mut ctx).expect("matching signature reuses mutable mult cache");
        assert!(
            Arc::ptr_eq(&mutable_cached_sentinel, &sentinel),
            "matching signatures should reuse the cached mult transfer in the mutable path"
        );

        ctx.set_input_analog_vector_from_fn("in", 3, |index| {
            AnalogValue::new([1.0, 4.0, 3.0][index])
        });
        let updated = cache_mult_transfer(&mut ctx).expect("changed input recomputes transfer");
        assert_ne!(
            updated.as_ref(),
            sentinel.as_ref(),
            "changed mult inputs must invalidate the cached transfer"
        );
        assert_eq!(
            updated.as_ref(),
            mult_transfer_from_context(&ctx)
                .expect("direct mult transfer")
                .as_ref()
        );
    }

    #[test]
    fn limit_partials_compute_from_context_without_prior_evaluate() {
        let mut ctx = CmContext::new();
        ctx.set_input_analog("in", 0.25);
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("out_lower_limit", 0.0);
        ctx.set_param("out_upper_limit", 10.0);
        ctx.set_param("limit_range", 0.0);
        ctx.set_param("gain", 2.0);
        ctx.set_param("fraction", 0.0);

        assert_eq!(
            Limiter.output_input_partials(&ctx, "out"),
            vec![("in".to_string(), 2.0)],
            "limit partials should not depend on a previous evaluate call"
        );
    }

    #[test]
    fn limit_transfer_cache_reuses_evaluated_transfer_until_inputs_change() {
        let mut ctx = CmContext::new();
        ctx.set_input_analog("in", 0.5);
        ctx.set_param("in_offset", 0.25);
        ctx.set_param("out_lower_limit", 0.0);
        ctx.set_param("out_upper_limit", 2.0);
        ctx.set_param("limit_range", 0.2);
        ctx.set_param("gain", 1.5);
        ctx.set_param("fraction", 0.0);

        let first = cache_limit_transfer(&mut ctx);
        assert_eq!(limit_transfer_for_context(&ctx), first);

        ctx.set_param("unrelated", 42.0);
        assert_eq!(
            limit_transfer_for_context(&ctx),
            first,
            "unrelated context changes should not invalidate the limit transfer cache"
        );

        let signature = limit_transfer_signature(&ctx);
        let sentinel = LimitTransfer {
            output: -123.0,
            in_partial: -456.0,
        };
        ctx.set_resource(
            LIMIT_TRANSFER_RESOURCE,
            Arc::new(LimitTransferResource {
                signature,
                transfer: sentinel,
            }),
        );
        assert_eq!(
            limit_transfer_for_context(&ctx),
            sentinel,
            "matching signatures should reuse the cached limit transfer"
        );
        assert_eq!(
            cache_limit_transfer(&mut ctx),
            sentinel,
            "matching signatures should reuse the cached limit transfer in the mutable path"
        );

        ctx.set_input_analog("in", 0.75);
        let updated = cache_limit_transfer(&mut ctx);
        assert_ne!(
            updated, sentinel,
            "changed limit inputs must invalidate the cached transfer"
        );
        assert_eq!(
            updated,
            limit_transfer_from_signature(limit_transfer_signature(&ctx))
        );
    }

    #[test]
    fn divide_transfer_cache_reuses_evaluated_transfer_until_inputs_change() {
        let mut ctx = CmContext::new();
        ctx.set_input_analog("num", 3.0);
        ctx.set_input_analog("den", 1.0);
        ctx.set_param("num_offset", 1.0);
        ctx.set_param("num_gain", 2.0);
        ctx.set_param("den_offset", 1.0);
        ctx.set_param("den_gain", 4.0);
        ctx.set_param("den_lower_limit", 1.0e-10);
        ctx.set_param("den_domain", 0.0);
        ctx.set_param("fraction", 0.0);
        ctx.set_param("out_gain", 3.0);
        ctx.set_param("out_offset", 5.0);

        let first = cache_divide_transfer(&mut ctx);
        assert_eq!(divide_transfer_for_context(&ctx), first);

        ctx.set_param("unrelated", 42.0);
        assert_eq!(
            divide_transfer_for_context(&ctx),
            first,
            "unrelated context changes should not invalidate the divide transfer cache"
        );

        let signature = divide_transfer_signature(&ctx);
        let sentinel = DivideTransfer {
            output: -123.0,
            num_partial: -456.0,
            den_partial: -789.0,
        };
        ctx.set_resource(
            DIVIDE_TRANSFER_RESOURCE,
            Arc::new(DivideTransferResource {
                signature,
                transfer: sentinel,
            }),
        );
        assert_eq!(
            divide_transfer_for_context(&ctx),
            sentinel,
            "matching signatures should reuse the cached divide transfer"
        );
        assert_eq!(
            cache_divide_transfer(&mut ctx),
            sentinel,
            "matching signatures should reuse the cached divide transfer in the mutable path"
        );

        ctx.set_input_analog("den", 2.0);
        let updated = cache_divide_transfer(&mut ctx);
        assert_ne!(
            updated, sentinel,
            "changed divide inputs must invalidate the cached transfer"
        );
        assert_eq!(
            updated,
            divide_transfer_from_signature(divide_transfer_signature(&ctx))
        );
    }

    #[test]
    fn climit_transfer_cache_reuses_evaluated_transfer_until_control_changes() {
        let mut ctx = CmContext::new();
        ctx.set_input_analog("in", 1.85);
        ctx.set_input_analog("cntl_lower", 0.0);
        ctx.set_input_analog("cntl_upper", 2.0);
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("lower_delta", 0.0);
        ctx.set_param("upper_delta", 0.0);
        ctx.set_param("limit_range", 0.5);
        ctx.set_param("gain", 1.0);
        ctx.set_param("fraction", 0.0);

        let first = cache_climit_transfer(&mut ctx);
        assert_eq!(climit_transfer_for_context(&ctx), first);

        ctx.set_param("unrelated", 42.0);
        assert_eq!(
            climit_transfer_for_context(&ctx),
            first,
            "unrelated context changes should not invalidate the climit transfer cache"
        );

        let signature = climit_transfer_signature(&ctx);
        let sentinel = ClimitTransfer {
            output: -123.0,
            in_partial: -456.0,
            lower_partial: -789.0,
            upper_partial: -1011.0,
        };
        ctx.set_resource(
            CLIMIT_TRANSFER_RESOURCE,
            Arc::new(ClimitTransferResource {
                signature,
                transfer: sentinel,
            }),
        );
        assert_eq!(
            climit_transfer_for_context(&ctx),
            sentinel,
            "matching signatures should reuse the cached climit transfer"
        );
        assert_eq!(
            cache_climit_transfer(&mut ctx),
            sentinel,
            "matching signatures should reuse the cached climit transfer in the mutable path"
        );

        ctx.set_input_analog("cntl_upper", 3.0);
        let updated = cache_climit_transfer(&mut ctx);
        assert_ne!(
            updated, sentinel,
            "changed climit controls must invalidate the cached transfer"
        );
        assert_eq!(
            updated,
            climit_transfer_from_signature(climit_transfer_signature(&ctx))
        );
    }

    #[test]
    fn delay_negative_controlled_delmax_collapses_min_bound_like_ngspice() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.timestep = 0.1e-9;
        ctx.set_transient_run_context(Some(0.1e-9), Some(2.0e-9));
        ctx.set_param("has_delay_cnt", 1.0);
        ctx.set_param("delmin", 1.0e-9);
        ctx.set_param("delmax", -1.0e-9);
        ctx.mark_param_provided("delmax");
        ctx.set_input_analog("cntrl", 0.5);

        let delay = effective_delay(&ctx);

        assert_eq!(
            delay, 0.0,
            "ngspice cm_delay resets tdelmin when a provided delmax is negative"
        );
    }

    #[test]
    fn slew_does_not_commit_rollbackable_probe_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("rise_slope", 1.0e9);
        ctx.set_param("fall_slope", 1.0e9);
        ctx.init_output("out", PortType::Voltage);

        SlewRateFollower.init(&mut ctx).expect("slew init");
        ctx.time = 0.0;
        ctx.set_input_analog("in", 0.0);
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        SlewRateFollower
            .evaluate(&mut ctx)
            .expect("accepted initial slew sample");
        ctx.advance_state();

        ctx.time = 1.0e-9;
        ctx.set_input_analog("in", 10.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        SlewRateFollower
            .evaluate(&mut ctx)
            .expect("probe slew sample");

        assert_eq!(
            (ctx.state(SLEW_INPUT), ctx.state(SLEW_OUTPUT)),
            (0.0, 0.0),
            "rollbackable slew probe must not commit input/output memory"
        );
        assert!((ctx.output("out") - 1.0).abs() < 1.0e-12);

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        SlewRateFollower
            .evaluate(&mut ctx)
            .expect("accepted slew sample");

        assert_eq!((ctx.state(SLEW_INPUT), ctx.state(SLEW_OUTPUT)), (10.0, 1.0));
    }

    #[test]
    fn integrator_does_not_commit_rollbackable_probe_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.timestep = 1.0;
        ctx.time = 1.0;
        ctx.set_param("gain", 1.0);
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("out_ic", 0.0);
        ctx.set_param("out_lower_limit", -10.0);
        ctx.set_param("out_upper_limit", 10.0);
        ctx.set_param("limit_range", 0.0);
        ctx.init_output("out", PortType::Voltage);

        Integrator.init(&mut ctx).expect("int init");
        ctx.set_input_analog("in", 2.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        Integrator
            .evaluate(&mut ctx)
            .expect("probe integrator sample");

        assert!((ctx.output("out") - 1.0).abs() < 1.0e-12);
        assert_eq!(
            (ctx.state(0), ctx.state(1)),
            (0.0, 0.0),
            "rollbackable integrator probe must not commit integrated output/input memory"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        Integrator
            .evaluate(&mut ctx)
            .expect("accepted integrator sample");

        assert_eq!((ctx.state(0), ctx.state(1)), (1.0, 2.0));
    }

    #[test]
    fn differentiator_does_not_commit_rollbackable_probe_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.timestep = 1.0;
        ctx.time = 1.0;
        ctx.set_param("gain", 1.0);
        ctx.set_param("out_offset", 0.0);
        ctx.set_param("out_lower_limit", -10.0);
        ctx.set_param("out_upper_limit", 10.0);
        ctx.set_param("limit_range", 0.0);
        ctx.init_output("out", PortType::Voltage);

        Differentiator.init(&mut ctx).expect("d_dt init");
        ctx.set_input_analog("in", 2.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        Differentiator
            .evaluate(&mut ctx)
            .expect("probe differentiator sample");

        assert!((ctx.output("out") - 2.0).abs() < 1.0e-12);
        assert_eq!(
            ctx.state(0),
            0.0,
            "rollbackable differentiator probe must not commit input memory"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        Differentiator
            .evaluate(&mut ctx)
            .expect("accepted differentiator sample");

        assert_eq!(ctx.state(0), 2.0);
    }

    #[test]
    fn sample_hold_does_not_commit_rollbackable_probe_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("cntl_th", 0.5);
        ctx.set_param("out_ic", 1.0);
        ctx.init_output("out", PortType::Voltage);

        SampleHold.init(&mut ctx).expect("s_h init");
        ctx.set_input_analog("cntl", 1.0);
        ctx.set_input_analog("in", 7.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        SampleHold.evaluate(&mut ctx).expect("probe s_h sample");

        assert_eq!(ctx.output("out"), 7.0);
        assert_eq!(
            ctx.state(0),
            1.0,
            "rollbackable sample-and-hold probe must not commit held value"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        SampleHold.evaluate(&mut ctx).expect("accepted s_h sample");

        assert_eq!(ctx.state(0), 7.0);
    }

    #[test]
    fn astate_does_not_commit_rollbackable_probe_history() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("astate_no", 1.0);
        ctx.init_output("out", PortType::Voltage);

        AnalogStateReturn.init(&mut ctx).expect("astate init");
        ctx.set_initial_state(0, 1.0);
        ctx.set_initial_state(1, 0.5);
        ctx.set_initial_state(2, 0.25);

        ctx.set_input_analog("in", 2.0);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        AnalogStateReturn
            .evaluate(&mut ctx)
            .expect("probe astate history update");

        assert_eq!(ctx.output("out"), 1.0);
        assert_eq!(
            (ctx.state(0), ctx.state(1), ctx.state(2)),
            (1.0, 0.5, 0.25),
            "rollbackable astate probe must not shift remembered samples"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        AnalogStateReturn
            .evaluate(&mut ctx)
            .expect("accepted astate history update");

        assert_eq!((ctx.state(0), ctx.state(1), ctx.state(2)), (2.0, 1.0, 0.5));
    }

    #[test]
    fn hyst_does_not_commit_rollbackable_probe_branch_state() {
        let mut ctx = CmContext::new();
        ctx.analysis = crate::xspice::AnalysisType::Transient;
        ctx.set_param("in_low", 0.0);
        ctx.set_param("in_high", 1.0);
        ctx.set_param("hyst", 0.2);
        ctx.set_param("out_lower_limit", 0.0);
        ctx.set_param("out_upper_limit", 10.0);
        ctx.set_param("input_domain", 0.0);
        ctx.set_param("fraction", 0.0);
        ctx.init_output("out", PortType::Voltage);

        HysteresisBlock.init(&mut ctx).expect("hyst init");

        ctx.set_input_analog("in", 0.0);
        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        HysteresisBlock
            .evaluate(&mut ctx)
            .expect("records initial rising branch");
        assert_eq!(ctx.int_state(HYST_STATE), HYST_RISING);

        ctx.set_input_analog("in", 1.3);
        ctx.set_evaluation_phase(EvaluationPhase::RollbackableProbe);
        HysteresisBlock
            .evaluate(&mut ctx)
            .expect("probes falling branch");
        assert_eq!(
            ctx.int_state(HYST_STATE),
            HYST_RISING,
            "rollbackable hyst probe must not advance remembered branch state"
        );

        ctx.set_evaluation_phase(EvaluationPhase::AcceptedStep);
        HysteresisBlock
            .evaluate(&mut ctx)
            .expect("commits accepted falling branch");
        assert_eq!(ctx.int_state(HYST_STATE), HYST_FALLING);
    }
}

#[allow(clippy::too_many_arguments)]
pub(super) fn climit_transfer(
    input: Value,
    in_offset: Value,
    cntl_upper: Value,
    cntl_lower: Value,
    lower_delta: Value,
    upper_delta: Value,
    mut limit_range: Value,
    gain: Value,
    fraction: bool,
) -> (Value, Value, Value, Value) {
    let out_lower_limit = cntl_lower + lower_delta;
    let out_upper_limit = cntl_upper - upper_delta;

    if fraction {
        limit_range *= out_upper_limit - out_lower_limit;
    }
    if !limit_range.is_finite() {
        limit_range = 0.0;
    }

    let threshold_upper = out_upper_limit - limit_range;
    let threshold_lower = out_lower_limit + limit_range;
    if threshold_upper < threshold_lower {
        return (0.0, 0.0, 0.0, 0.0);
    }

    let raw = gain * (in_offset + input);
    if raw < threshold_lower {
        if raw > out_lower_limit - limit_range && limit_range > 0.0 {
            let (limited_out, mut pout_pin) =
                smooth_corner(raw, out_lower_limit, out_lower_limit, limit_range, 0.0, 1.0);
            pout_pin *= gain;
            let (pout_pcntl_lower, _) =
                smooth_discontinuity(raw, out_lower_limit, 1.0, threshold_lower, 0.0);
            (limited_out, pout_pin, pout_pcntl_lower, 0.0)
        } else {
            (out_lower_limit, 0.0, 1.0, 0.0)
        }
    } else if raw > threshold_upper {
        if raw < out_upper_limit + limit_range && limit_range > 0.0 {
            let (limited_out, mut pout_pin) =
                smooth_corner(raw, out_upper_limit, out_upper_limit, limit_range, 1.0, 0.0);
            pout_pin *= gain;
            let (pout_pcntl_upper, _) =
                smooth_discontinuity(raw, threshold_upper, 0.0, out_upper_limit, 1.0);
            (limited_out, pout_pin, 0.0, pout_pcntl_upper)
        } else {
            (out_upper_limit, 0.0, 0.0, 1.0)
        }
    } else {
        (raw, gain, 0.0, 0.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ClimitTransferSignature {
    input: Value,
    in_offset: Value,
    cntl_upper: Value,
    cntl_lower: Value,
    lower_delta: Value,
    upper_delta: Value,
    limit_range: Value,
    gain: Value,
    fraction: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct ClimitTransfer {
    output: Value,
    in_partial: Value,
    lower_partial: Value,
    upper_partial: Value,
}

#[derive(Debug, Clone, Copy)]
struct ClimitTransferResource {
    signature: ClimitTransferSignature,
    transfer: ClimitTransfer,
}

fn climit_transfer_signature(ctx: &CmContext) -> ClimitTransferSignature {
    ClimitTransferSignature {
        input: ctx.input("in"),
        in_offset: ctx.param("in_offset"),
        cntl_upper: ctx.input("cntl_upper"),
        cntl_lower: ctx.input("cntl_lower"),
        lower_delta: ctx.param("lower_delta"),
        upper_delta: ctx.param("upper_delta"),
        limit_range: ctx.param("limit_range"),
        gain: ctx.param("gain"),
        fraction: ctx.param("fraction") > 0.5,
    }
}

fn climit_transfer_from_signature(signature: ClimitTransferSignature) -> ClimitTransfer {
    let (output, in_partial, lower_partial, upper_partial) = climit_transfer(
        signature.input,
        signature.in_offset,
        signature.cntl_upper,
        signature.cntl_lower,
        signature.lower_delta,
        signature.upper_delta,
        signature.limit_range,
        signature.gain,
        signature.fraction,
    );
    ClimitTransfer {
        output,
        in_partial,
        lower_partial,
        upper_partial,
    }
}

fn climit_transfer_for_context(ctx: &CmContext) -> ClimitTransfer {
    let signature = climit_transfer_signature(ctx);
    if let Some(resource) = ctx.resource::<ClimitTransferResource>(CLIMIT_TRANSFER_RESOURCE)
        && resource.signature == signature
    {
        return resource.transfer;
    }

    climit_transfer_from_signature(signature)
}

fn cache_climit_transfer(ctx: &mut CmContext) -> ClimitTransfer {
    let signature = climit_transfer_signature(ctx);
    if let Some(resource) = ctx.resource::<ClimitTransferResource>(CLIMIT_TRANSFER_RESOURCE)
        && resource.signature == signature
    {
        return resource.transfer;
    }

    let transfer = climit_transfer_from_signature(signature);
    ctx.set_resource(
        CLIMIT_TRANSFER_RESOURCE,
        Arc::new(ClimitTransferResource {
            signature,
            transfer,
        }),
    );
    transfer
}

pub(super) fn smooth_corner(
    x_input: Value,
    x_center: Value,
    y_center: Value,
    domain: Value,
    lower_slope: Value,
    upper_slope: Value,
) -> (Value, Value) {
    let x_upper = x_center + domain;
    let y_upper = y_center + upper_slope * domain;
    let a = ((upper_slope - lower_slope) / 4.0) / domain;
    let b = upper_slope - 2.0 * a * x_upper;
    let c = y_upper - a * x_upper * x_upper - b * x_upper;
    let y_output = a * x_input * x_input + b * x_input + c;
    let dy_dx = 2.0 * a * x_input + b;
    (y_output, dy_dx)
}

pub(super) fn smooth_discontinuity(
    x_input: Value,
    x_lower: Value,
    y_lower: Value,
    x_upper: Value,
    y_upper: Value,
) -> (Value, Value) {
    let x_center = (x_upper + x_lower) / 2.0;
    let y_center = (y_upper + y_lower) / 2.0;
    let center_slope = 2.0 * (y_upper - y_lower) / (x_upper - x_lower);

    if x_input < x_lower {
        (y_lower, 0.0)
    } else if x_input < x_center {
        let a = center_slope / (x_upper - x_lower);
        let b = center_slope - 2.0 * a * x_center;
        let c = y_center - a * x_center * x_center - b * x_center;
        (
            a * x_input * x_input + b * x_input + c,
            2.0 * a * x_input + b,
        )
    } else if x_input < x_upper {
        let a = -center_slope / (x_upper - x_lower);
        let b = -2.0 * a * x_upper;
        let c = y_upper - a * x_upper * x_upper - b * x_upper;
        (
            a * x_input * x_input + b * x_input + c,
            2.0 * a * x_input + b,
        )
    } else {
        (y_upper, 0.0)
    }
}

//=============================================================================
// Integrator
//=============================================================================

/// Continuous-time integrator: out = gain * integral(in) + out_ic
///
/// # Parameters
/// - `in_offset` - Input offset (default: 0.0)
/// - `gain` - Integration gain (default: 1.0)
/// - `out_lower_limit` - Lower saturation (default: -1e12)
/// - `out_upper_limit` - Upper saturation (default: 1e12)
/// - `limit_range` - Smoothing range near saturation limits (default: 1e-6)
/// - `out_ic` - Initial condition (default: 0.0)
///
/// # Ports
/// - `in` - Input to integrate
/// - `out` - Integrated output
#[derive(Debug, Default)]
pub struct Integrator;

impl CodeModel for Integrator {
    fn name(&self) -> &str {
        "integrator"
    }

    fn description(&self) -> &str {
        "Continuous-time integrator with saturation limits"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
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
                        PortType::VoltageName,
                    ],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Input to integrate".to_string(),
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
                    description: "Integrated output".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("in_offset", 0.0).with_description("Input offset"),
                ParamSpec::real("gain", 1.0).with_description("Integration gain (1/time_constant)"),
                ParamSpec::real("out_lower_limit", -1e12)
                    .required()
                    .with_description("Lower output saturation limit"),
                ParamSpec::real("out_upper_limit", 1e12)
                    .required()
                    .with_description("Upper output saturation limit"),
                ParamSpec::real("limit_range", 1.0e-6)
                    .with_description("Smoothing range near output limits"),
                ParamSpec::real("out_ic", 0.0).with_description("Initial output value"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        // State layout:
        // state[0] = integrated output
        // state[1] = previous input sample for trapezoidal integration
        ctx.allocate_states(2);

        // Set initial condition
        let ic = ctx.param("out_ic");
        ctx.set_state(0, ic);
        ctx.set_state(1, 0.0);
        ctx.advance_state();

        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param("gain");
        let in_offset = ctx.param("in_offset");
        let lower_limit = ctx.param("out_lower_limit");
        let upper_limit = ctx.param("out_upper_limit");
        let (lower, upper) = if lower_limit <= upper_limit {
            (lower_limit, upper_limit)
        } else {
            (upper_limit, lower_limit)
        };

        let v_in_raw = ctx.input("in");
        let v_in = if v_in_raw.is_finite() {
            v_in_raw + in_offset
        } else {
            in_offset
        };
        let dt = ctx.timestep;
        let prev_out = ctx.state_prev(0);
        let prev_in = ctx.state_prev(1);

        let (raw_out, raw_partial) =
            if ctx.is_dc() || ctx.time == 0.0 || !dt.is_finite() || dt <= 0.0 {
                (prev_out, 0.0)
            } else {
                // Trapezoidal integration:
                // y[n] = y[n-1] + gain * dt * (x[n] + x[n-1]) / 2
                let delta = 0.5 * gain * dt * (v_in + prev_in);
                let partial = if v_in_raw.is_finite() {
                    0.5 * gain * dt
                } else {
                    0.0
                };
                (prev_out + delta, partial)
            };
        let (new_out, limit_partial) = limit_transfer(
            raw_out,
            0.0,
            lower,
            upper,
            ctx.param("limit_range"),
            1.0,
            false,
        );

        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(0, new_out);
            ctx.set_state(1, v_in);
        }
        ctx.set_output_with_partial("out", new_out, raw_partial * limit_partial);

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
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
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() || omega == 0.0 {
            return Vec::new();
        }
        vec![(
            "in".to_string(),
            Complex64::new(0.0, -ctx.param("gain") / omega),
        )]
    }
}

//=============================================================================
// Differentiator
//=============================================================================

/// Continuous-time differentiator: out = gain * d(in)/dt
///
/// # Parameters
/// - `out_offset` - Output offset (default: 0.0)
/// - `gain` - Differentiation gain (default: 1.0)
/// - `out_lower_limit` - Lower saturation (default: -1e12)
/// - `out_upper_limit` - Upper saturation (default: 1e12)
/// - `limit_range` - Smoothing range near saturation limits (default: 1e-6)
///
/// # Ports
/// - `in` - Input to differentiate
/// - `out` - Differentiated output
#[derive(Debug, Default)]
pub struct Differentiator;

impl CodeModel for Differentiator {
    fn name(&self) -> &str {
        "differentiator"
    }

    fn description(&self) -> &str {
        "Continuous-time differentiator with saturation limits"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
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
                        PortType::VoltageName,
                    ],
                    is_vector: false,
                    null_allowed: false,
                    vector_min_len: None,
                    vector_max_len: None,
                    description: "Input to differentiate".to_string(),
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
                    description: "Differentiated output".to_string(),
                },
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("out_offset", 0.0).with_description("Output offset voltage"),
                ParamSpec::real("gain", 1.0).with_description("Differentiation gain"),
                ParamSpec::real("out_lower_limit", -1e12)
                    .required()
                    .with_description("Lower output saturation limit"),
                ParamSpec::real("out_upper_limit", 1e12)
                    .required()
                    .with_description("Upper output saturation limit"),
                ParamSpec::real("limit_range", 1.0e-6)
                    .with_description("Smoothing range near output limits"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        // Allocate state for previous input
        ctx.allocate_states(1);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let gain = ctx.param("gain");
        let out_offset = ctx.param("out_offset");
        let lower = ctx.param("out_lower_limit");
        let upper = ctx.param("out_upper_limit");
        let limit_range = ctx.param("limit_range");

        let v_in = ctx.input("in");
        let dt = ctx.timestep;
        let prev_in = ctx.state_prev(0);

        let (raw_out, raw_partial) = if ctx.is_dc() || ctx.time == 0.0 {
            (0.0, 0.0)
        } else {
            // Backward difference: dy/dt â‰ˆ (y[n] - y[n-1]) / dt
            if dt > 1e-18 {
                (gain * (v_in - prev_in) / dt + out_offset, gain / dt)
            } else {
                (out_offset, 0.0)
            }
        };

        let (v_out, limit_partial) =
            limit_transfer(raw_out, 0.0, lower, upper, limit_range, 1.0, false);

        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(0, v_in);
        }
        ctx.set_output_with_partial("out", v_out, raw_partial * limit_partial);

        Ok(())
    }

    fn output_input_partials(&self, ctx: &CmContext, output_port: &str) -> Vec<(String, Value)> {
        if output_port.eq_ignore_ascii_case("out") {
            vec![("in".to_string(), ctx.partial("out"))]
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
        let omega = 2.0 * std::f64::consts::PI * frequency;
        if !omega.is_finite() {
            return Vec::new();
        }
        vec![(
            "in".to_string(),
            Complex64::new(0.0, ctx.param("gain") * omega),
        )]
    }
}

//=============================================================================
// Official Aliases
//=============================================================================

macro_rules! analog_model_alias {
    ($alias:ident, $target:ident, $name:literal) => {
        #[derive(Debug, Default)]
        pub struct $alias;

        impl CodeModel for $alias {
            fn name(&self) -> &str {
                $name
            }

            fn description(&self) -> &str {
                $target.description()
            }

            fn ports(&self) -> &[PortSpec] {
                $target.ports()
            }

            fn parameters(&self) -> &[ParamSpec] {
                $target.parameters()
            }

            fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
                $target.init(ctx)
            }

            fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
                $target.evaluate(ctx)
            }

            fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
                $target.ac_gain(ctx)
            }

            fn output_input_partials(
                &self,
                ctx: &CmContext,
                output_port: &str,
            ) -> Vec<(String, Value)> {
                $target.output_input_partials(ctx, output_port)
            }

            fn output_input_ac_partials(
                &self,
                ctx: &CmContext,
                output_port: &str,
                frequency: Value,
            ) -> Vec<(String, Complex64)> {
                $target.output_input_ac_partials(ctx, output_port, frequency)
            }

            fn output_input_vector_ac_partials(
                &self,
                ctx: &CmContext,
                output_port: &str,
                frequency: Value,
            ) -> Vec<(String, usize, Complex64)> {
                $target.output_input_vector_ac_partials(ctx, output_port, frequency)
            }

            fn output_vector_input_ac_partials(
                &self,
                ctx: &CmContext,
                output_port: &str,
                output_index: usize,
                frequency: Value,
            ) -> Vec<(String, Complex64)> {
                $target.output_vector_input_ac_partials(ctx, output_port, output_index, frequency)
            }

            fn output_vector_input_vector_ac_partials(
                &self,
                ctx: &CmContext,
                output_port: &str,
                output_index: usize,
                frequency: Value,
            ) -> Vec<(String, usize, Complex64)> {
                $target.output_vector_input_vector_ac_partials(
                    ctx,
                    output_port,
                    output_index,
                    frequency,
                )
            }
        }
    };
}

analog_model_alias!(DivideAlias, Divider, "divide");
analog_model_alias!(IntegratorAlias, Integrator, "int");
analog_model_alias!(DifferentiatorAlias, Differentiator, "d_dt");

//=============================================================================
// Sample and Hold
//=============================================================================

/// Sample and hold circuit
///
/// # Parameters
/// - `cntl_th` - Control threshold for sampling (default: 0.5)
/// - `out_ic` - Initial output value (default: 0.0)
///
/// # Ports
/// - `cntl` - Control input (sample when high)
/// - `in` - Analog input to sample
/// - `out` - Held output value
#[derive(Debug, Default)]
pub struct SampleHold;

impl CodeModel for SampleHold {
    fn name(&self) -> &str {
        "s_h"
    }

    fn description(&self) -> &str {
        "Sample and hold circuit"
    }

    fn ports(&self) -> &[PortSpec] {
        use std::sync::OnceLock;
        static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
        PORTS.get_or_init(|| {
            vec![
                PortSpec::input("cntl", PortType::Voltage)
                    .with_description("Control (sample when above threshold)"),
                PortSpec::input("in", PortType::Voltage).with_description("Analog input to sample"),
                PortSpec::output("out", PortType::Voltage).with_description("Held output value"),
            ]
        })
    }

    fn parameters(&self) -> &[ParamSpec] {
        use std::sync::OnceLock;
        static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
        PARAMS.get_or_init(|| {
            vec![
                ParamSpec::real("cntl_th", 0.5).with_description("Control threshold for sampling"),
                ParamSpec::real("out_ic", 0.0).with_description("Initial output value"),
            ]
        })
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        ctx.allocate_states(1);
        let ic = ctx.param("out_ic");
        ctx.set_state(0, ic);
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let cntl_th = ctx.param("cntl_th");

        let v_cntl = ctx.input("cntl");
        let v_in = ctx.input("in");

        // Sample when control is above threshold
        let held_value = if v_cntl >= cntl_th {
            v_in // Sample new value
        } else {
            ctx.state(0) // Hold previous value
        };

        if ctx.evaluation_phase() != EvaluationPhase::RollbackableProbe {
            ctx.set_state(0, held_value);
        }
        ctx.set_output("out", held_value);

        Ok(())
    }
}

//=============================================================================
// Tests
//=============================================================================

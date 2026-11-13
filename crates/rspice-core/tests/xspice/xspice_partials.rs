//! The declared partials of the gain, limit, pwl and summer code models agree
//! with a central finite difference of their own outputs, for scalar and
//! vector inputs alike.

use rspice_core::Value;
use rspice_core::xspice::{
    CmContext, CmError, CodeModel, CodeModelRegistry, ParamSpec, ParamType, PortDirection,
};
use std::collections::BTreeMap;

#[test]
fn scalar_analog_partials_match_finite_difference() {
    let registry = CodeModelRegistry::with_builtins();
    verify_scalar_model(&registry, "gain", |ctx| {
        ctx.set_input_analog("in", 0.375);
        ctx.set_param("gain", 2.75);
        ctx.set_param("in_offset", -0.125);
        ctx.set_param("out_offset", 0.5);
    });
    verify_scalar_model(&registry, "limit", |ctx| {
        ctx.set_input_analog("in", 0.25);
        ctx.set_param("gain", 1.5);
        ctx.set_param("in_offset", 0.0);
        ctx.set_param("out_lower_limit", -10.0);
        ctx.set_param("out_upper_limit", 10.0);
    });
    verify_scalar_model(&registry, "pwl", |ctx| {
        ctx.set_input_analog("in", 0.25);
        ctx.set_real_vector_param("x_array", vec![-1.0, 0.0, 1.0]);
        ctx.set_real_vector_param("y_array", vec![-2.0, 0.0, 4.0]);
        ctx.set_param("input_domain", 0.0);
        ctx.set_param("fraction", 1.0);
    });
}

#[test]
fn vector_analog_partials_match_finite_difference() {
    let registry = CodeModelRegistry::with_builtins();
    let model = registry.get("summer").expect("summer registered");
    let mut ctx = context_with_model_defaults(model.as_ref());
    ctx.set_port_width("in", 3);
    ctx.set_input_analog_vector("in", &[0.25, -0.5, 0.75])
        .expect("seed summer vector input");
    ctx.set_real_vector_param("in_offset", vec![0.0, 0.1, -0.2]);
    ctx.set_real_vector_param("in_gain", vec![1.0, -2.0, 0.5]);
    ctx.set_param("out_gain", 1.25);
    ctx.set_param("out_offset", -0.375);

    let mismatches = verify_model_partials(
        model.as_ref(),
        &ctx,
        PartialVerificationOptions {
            absolute_tolerance: 1.0e-8,
            relative_tolerance: 1.0e-6,
            ..PartialVerificationOptions::default()
        },
    )
    .expect("summer partials verify");
    assert!(mismatches.is_empty(), "{mismatches:#?}");
}

fn verify_scalar_model(
    registry: &CodeModelRegistry,
    name: &str,
    configure: impl FnOnce(&mut CmContext),
) {
    let model = registry.get(name).expect("model registered");
    let mut ctx = context_with_model_defaults(model.as_ref());
    configure(&mut ctx);
    let mismatches = verify_model_partials(
        model.as_ref(),
        &ctx,
        PartialVerificationOptions {
            absolute_tolerance: 1.0e-8,
            relative_tolerance: 1.0e-6,
            ..PartialVerificationOptions::default()
        },
    )
    .unwrap_or_else(|err| panic!("{name} partial verifier failed: {err}"));
    assert!(
        mismatches.is_empty(),
        "{name} partial mismatch(es): {mismatches:#?}"
    );
}

#[derive(Debug, Clone, Copy)]
struct PartialVerificationOptions {
    pub step: Value,
    pub absolute_tolerance: Value,
    pub relative_tolerance: Value,
}

impl Default for PartialVerificationOptions {
    fn default() -> Self {
        Self {
            step: 1.0e-6,
            absolute_tolerance: 1.0e-6,
            relative_tolerance: 1.0e-4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct PartialMismatch {
    pub model: String,
    pub output_port: String,
    pub input_port: String,
    pub input_index: Option<usize>,
    pub analytic: Value,
    pub numeric: Value,
    pub abs_error: Value,
    pub rel_error: Value,
}

fn verify_model_partials(
    model: &dyn CodeModel,
    ctx: &CmContext,
    options: PartialVerificationOptions,
) -> Result<Vec<PartialMismatch>, CmError> {
    let mut base = ctx.clone();
    model.init(&mut base)?;
    model.evaluate(&mut base)?;

    let mut mismatches = Vec::new();
    for output in model.ports().iter().filter(|port| {
        port.direction == PortDirection::Out && port.default_type.is_analog() && !port.is_vector
    }) {
        let analytic_scalars = model
            .output_input_partials(&base, &output.name)
            .into_iter()
            .map(|(name, value)| ((canonical_key(&name), None::<usize>), value))
            .collect::<BTreeMap<_, _>>();
        let analytic_vectors = model
            .output_input_vector_partials(&base, &output.name)
            .into_iter()
            .map(|(name, index, value)| ((canonical_key(&name), Some(index)), value))
            .collect::<BTreeMap<_, _>>();

        for input in model.ports().iter().filter(|port| {
            matches!(port.direction, PortDirection::In | PortDirection::InOut)
                && port.default_type.is_analog()
        }) {
            if input.is_vector {
                let width = ctx.port_width(&input.name).max(1);
                for index in 0..width {
                    let numeric = finite_difference_vector_input(
                        model,
                        ctx,
                        &output.name,
                        &input.name,
                        index,
                        options.step,
                    )?;
                    let analytic = analytic_vectors
                        .get(&(canonical_key(&input.name), Some(index)))
                        .copied()
                        .unwrap_or(0.0);
                    push_partial_mismatch_if_needed(
                        PartialDerivativeSite {
                            model: model.name(),
                            output_port: &output.name,
                            input_port: &input.name,
                            input_index: Some(index),
                        },
                        analytic,
                        numeric,
                        options,
                        &mut mismatches,
                    );
                }
            } else {
                let numeric = finite_difference_scalar_input(
                    model,
                    ctx,
                    &output.name,
                    &input.name,
                    options.step,
                )?;
                let analytic = analytic_scalars
                    .get(&(canonical_key(&input.name), None))
                    .copied()
                    .unwrap_or(0.0);
                push_partial_mismatch_if_needed(
                    PartialDerivativeSite {
                        model: model.name(),
                        output_port: &output.name,
                        input_port: &input.name,
                        input_index: None,
                    },
                    analytic,
                    numeric,
                    options,
                    &mut mismatches,
                );
            }
        }
    }

    Ok(mismatches)
}

fn context_with_model_defaults(model: &dyn CodeModel) -> CmContext {
    let mut ctx = CmContext::new();
    for port in model.ports() {
        if port.is_vector
            && let Some(min_len) = port.vector_min_len
        {
            ctx.set_port_width(&port.name, min_len.max(1));
        }
    }
    for param in model.parameters() {
        apply_param_default(&mut ctx, param);
    }
    ctx
}

fn finite_difference_scalar_input(
    model: &dyn CodeModel,
    ctx: &CmContext,
    output: &str,
    input: &str,
    step: Value,
) -> Result<Value, CmError> {
    let current = ctx.input(input);
    let mut plus = ctx.clone();
    plus.set_input_analog(input, current + step);
    model.init(&mut plus)?;
    model.evaluate(&mut plus)?;

    let mut minus = ctx.clone();
    minus.set_input_analog(input, current - step);
    model.init(&mut minus)?;
    model.evaluate(&mut minus)?;

    Ok((plus.output(output) - minus.output(output)) / (2.0 * step))
}

fn finite_difference_vector_input(
    model: &dyn CodeModel,
    ctx: &CmContext,
    output: &str,
    input: &str,
    index: usize,
    step: Value,
) -> Result<Value, CmError> {
    let mut values = ctx.input_vector(input);
    let width = ctx.port_width(input).max(index + 1);
    values.resize(width, 0.0);

    let mut plus = ctx.clone();
    let mut plus_values = values.clone();
    plus_values[index] += step;
    plus.set_input_analog_vector(input, &plus_values)?;
    model.init(&mut plus)?;
    model.evaluate(&mut plus)?;

    let mut minus = ctx.clone();
    let mut minus_values = values;
    minus_values[index] -= step;
    minus.set_input_analog_vector(input, &minus_values)?;
    model.init(&mut minus)?;
    model.evaluate(&mut minus)?;

    Ok((plus.output(output) - minus.output(output)) / (2.0 * step))
}

/// Which partial derivative a numeric-versus-analytic check is about: the
/// model, the output it differentiates, and the input it differentiates with
/// respect to.
#[derive(Clone, Copy)]
struct PartialDerivativeSite<'a> {
    model: &'a str,
    output_port: &'a str,
    input_port: &'a str,
    input_index: Option<usize>,
}

fn push_partial_mismatch_if_needed(
    site: PartialDerivativeSite<'_>,
    analytic: Value,
    numeric: Value,
    options: PartialVerificationOptions,
    mismatches: &mut Vec<PartialMismatch>,
) {
    let PartialDerivativeSite {
        model,
        output_port,
        input_port,
        input_index,
    } = site;
    let abs_error = (analytic - numeric).abs();
    let scale = analytic.abs().max(numeric.abs()).max(1.0);
    let rel_error = abs_error / scale;
    if abs_error > options.absolute_tolerance && rel_error > options.relative_tolerance {
        mismatches.push(PartialMismatch {
            model: model.to_string(),
            output_port: output_port.to_string(),
            input_port: input_port.to_string(),
            input_index,
            analytic,
            numeric,
            abs_error,
            rel_error,
        });
    }
}

fn apply_param_default(ctx: &mut CmContext, param: &ParamSpec) {
    match param.param_type {
        ParamType::Real | ParamType::Integer | ParamType::Boolean => {
            ctx.set_param(&param.name, param.default);
        }
        ParamType::Complex => {
            if let Some(value) = param.complex_default {
                ctx.set_complex_param(&param.name, value);
            }
        }
        ParamType::String => {
            if let Some(value) = &param.string_default {
                ctx.set_string_param(&param.name, value);
            }
        }
        ParamType::StringVector => {
            if let Some(value) = &param.string_vector_default {
                ctx.set_string_vector_param(&param.name, value.clone());
            }
        }
        ParamType::RealVector => {
            if let Some(value) = &param.real_vector_default {
                ctx.set_real_vector_param(&param.name, value.clone());
            }
        }
        ParamType::IntegerVector => {
            if let Some(value) = &param.integer_vector_default {
                ctx.set_integer_vector_param(&param.name, value.clone());
            }
        }
        ParamType::ComplexVector => {
            if let Some(value) = &param.complex_vector_default {
                ctx.set_complex_vector_param(&param.name, value.clone());
            }
        }
    }
}

fn canonical_key(name: &str) -> String {
    name.trim().to_ascii_lowercase()
}

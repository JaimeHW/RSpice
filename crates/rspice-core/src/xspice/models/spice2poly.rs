//! SPICE2-compatible polynomial controlled source.
//!
//! This ports ngspice's official `spice2poly` XSPICE code model. The
//! coefficient exponent order intentionally follows the SPICE 2G6 `NXTPWR`
//! sequence instead of a rewritten polynomial ordering.

use crate::Value;
use crate::xspice::context::AnalogValue;
use crate::xspice::{
    CmContext, CmError, CmResult, CodeModel, ParamSpec, PortDirection, PortSpec, PortType,
};
use std::sync::{Arc, OnceLock};

const POLY_PLAN_RESOURCE: &str = "xspice.spice2poly.plan";
const POLY_EVAL_RESOURCE: &str = "xspice.spice2poly.eval";

#[derive(Debug, Default)]
pub struct Spice2Poly;

#[derive(Debug, Default)]
pub struct IcmSpice2Poly;

#[derive(Debug, Clone, PartialEq)]
struct PolyEval {
    value: Value,
    partials: Vec<Value>,
}

#[derive(Debug, Clone)]
struct PolyTerm {
    coefficient: Value,
    active_exponents: Vec<(usize, usize)>,
}

#[derive(Debug, Clone)]
struct PolyPlan {
    constant: Value,
    terms: Vec<PolyTerm>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct PolyPlanSignature {
    input_count: usize,
    coef_revision: u64,
}

#[derive(Debug, Clone)]
struct PolyPlanResource {
    signature: PolyPlanSignature,
    plan: CmResult<Arc<PolyPlan>>,
}

#[derive(Debug, Clone, PartialEq)]
struct PolyEvalSignature {
    plan: PolyPlanSignature,
    multiplier: Value,
    inputs: Vec<Value>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PolyEvalBaseSignature {
    plan: PolyPlanSignature,
    multiplier: Value,
}

#[derive(Debug, Clone)]
struct PolyEvalResource {
    signature: PolyEvalSignature,
    result: Arc<PolyEval>,
}

fn ports() -> &'static [PortSpec] {
    static PORTS: OnceLock<Vec<PortSpec>> = OnceLock::new();
    PORTS.get_or_init(|| {
        vec![
            PortSpec {
                name: "in".to_string(),
                direction: PortDirection::In,
                default_type: PortType::DifferentialVoltage,
                allowed_types: vec![
                    PortType::DifferentialVoltage,
                    PortType::DifferentialCurrent,
                    PortType::VoltageName,
                ],
                is_vector: true,
                null_allowed: false,
                vector_min_len: Some(1),
                vector_max_len: None,
                description: "Input vector".to_string(),
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
                description: "Polynomial output".to_string(),
            },
        ]
    })
}

fn parameters() -> &'static [ParamSpec] {
    static PARAMS: OnceLock<Vec<ParamSpec>> = OnceLock::new();
    PARAMS.get_or_init(|| {
        vec![
            ParamSpec::real_vector("coef", Vec::new())
                .required()
                .with_vector_min_len(2)
                .with_description("SPICE2-compatible polynomial coefficient list"),
            ParamSpec::real("m", 1.0).with_description("Output multiplier"),
        ]
    })
}

fn invalid_param(name: &str, message: impl Into<String>) -> CmError {
    CmError::InvalidParameter {
        name: name.to_string(),
        message: message.into(),
    }
}

fn validate_coef(coef: &[Value]) -> CmResult<()> {
    if coef.len() < 2 {
        return Err(invalid_param(
            "coef",
            format!(
                "coefficient vector requires at least 2 values, got {}",
                coef.len()
            ),
        ));
    }
    for (index, value) in coef.iter().enumerate() {
        if !value.is_finite() {
            return Err(invalid_param(
                "coef",
                format!("coefficient {index} must be finite, got {value}"),
            ));
        }
    }
    Ok(())
}

fn checked_inputs(ctx: &CmContext) -> CmResult<&[AnalogValue]> {
    let inputs = ctx.input_analog_vector_values("in").unwrap_or(&[]);
    if inputs.is_empty() {
        return Err(CmError::PortCountMismatch {
            expected: 1,
            actual: 0,
        });
    }
    for (index, input) in inputs.iter().enumerate() {
        if !input.value.is_finite() {
            return Err(CmError::EvaluationError(format!(
                "input {index} must be finite, got {}",
                input.value
            )));
        }
    }
    Ok(inputs)
}

fn checked_coef(ctx: &CmContext) -> CmResult<&[Value]> {
    let coef = ctx
        .real_vector_param("coef")
        .ok_or_else(|| CmError::MissingParameter("coef".to_string()))?;
    validate_coef(coef)?;
    Ok(coef)
}

fn checked_multiplier(ctx: &CmContext) -> CmResult<Value> {
    let multiplier = ctx.param_or("m", 1.0);
    if !multiplier.is_finite() {
        return Err(invalid_param(
            "m",
            format!("multiplier must be finite, got {multiplier}"),
        ));
    }
    Ok(multiplier)
}

fn evterm(x: Value, mut n: usize) -> Value {
    let mut product = 1.0;
    while n > 0 {
        product *= x;
        n -= 1;
    }
    product
}

fn nxtpwr(pwrseq: &mut [usize]) {
    let pdim = pwrseq.len();
    if pdim == 1 {
        pwrseq[0] += 1;
        return;
    }

    let mut k = pdim;
    while k > 0 && pwrseq[k - 1] == 0 {
        k -= 1;
    }
    if k == 0 {
        pwrseq[0] += 1;
        return;
    }

    if k != pdim {
        pwrseq[k - 1] -= 1;
        pwrseq[k] += 1;
        return;
    }

    if pwrseq[..pdim - 1].iter().all(|power| *power == 0) {
        pwrseq[0] = pwrseq[pdim - 1] + 1;
        pwrseq[pdim - 1] = 0;
        return;
    }

    let mut psum = 1usize;
    k = pdim;
    while pwrseq[k - 2] < 1 {
        psum += pwrseq[k - 1];
        pwrseq[k - 1] = 0;
        k -= 1;
    }
    pwrseq[k - 1] += psum;
    pwrseq[k - 2] -= 1;
}

fn poly_plan_signature(ctx: &CmContext, input_count: usize) -> CmResult<PolyPlanSignature> {
    let coef_revision = ctx
        .real_vector_param_revision("coef")
        .ok_or_else(|| CmError::MissingParameter("coef".to_string()))?;
    Ok(PolyPlanSignature {
        input_count,
        coef_revision,
    })
}

fn poly_plan_signature_matches(
    ctx: &CmContext,
    signature: &PolyPlanSignature,
    input_count: usize,
) -> bool {
    signature.input_count == input_count
        && ctx
            .real_vector_param_revision("coef")
            .is_some_and(|revision| revision == signature.coef_revision)
}

fn build_poly_plan(input_count: usize, coef: &[Value]) -> CmResult<Arc<PolyPlan>> {
    validate_coef(coef)?;

    let mut exponents = vec![0usize; input_count];
    let mut terms = Vec::with_capacity(coef.iter().skip(1).filter(|&&c| c != 0.0).count());
    for coefficient in coef.iter().skip(1).copied() {
        nxtpwr(&mut exponents);
        if coefficient == 0.0 {
            continue;
        }
        let active_exponents = exponents
            .iter()
            .copied()
            .enumerate()
            .filter(|(_, exponent)| *exponent != 0)
            .collect();
        terms.push(PolyTerm {
            coefficient,
            active_exponents,
        });
    }

    Ok(Arc::new(PolyPlan {
        constant: coef[0],
        terms,
    }))
}

fn cache_poly_plan(ctx: &mut CmContext, input_count: usize) -> CmResult<Arc<PolyPlan>> {
    if let Some(resource) = ctx.resource::<PolyPlanResource>(POLY_PLAN_RESOURCE)
        && poly_plan_signature_matches(ctx, &resource.signature, input_count)
    {
        return resource.plan.clone();
    }

    let signature = poly_plan_signature(ctx, input_count)?;
    let plan = build_poly_plan(input_count, checked_coef(ctx)?);
    ctx.set_resource(
        POLY_PLAN_RESOURCE,
        Arc::new(PolyPlanResource {
            signature,
            plan: plan.clone(),
        }),
    );
    plan
}

fn poly_plan(ctx: &CmContext, input_count: usize) -> CmResult<Arc<PolyPlan>> {
    if let Some(resource) = ctx.resource::<PolyPlanResource>(POLY_PLAN_RESOURCE)
        && poly_plan_signature_matches(ctx, &resource.signature, input_count)
    {
        return resource.plan.clone();
    }

    poly_plan_signature(ctx, input_count)?;
    build_poly_plan(input_count, checked_coef(ctx)?)
}

fn input_values(inputs: &[AnalogValue]) -> Vec<Value> {
    inputs.iter().map(|input| input.value).collect()
}

fn poly_eval_base_signature(
    ctx: &CmContext,
    input_count: usize,
) -> CmResult<PolyEvalBaseSignature> {
    Ok(PolyEvalBaseSignature {
        plan: poly_plan_signature(ctx, input_count)?,
        multiplier: checked_multiplier(ctx)?,
    })
}

fn poly_eval_signature_from_inputs(
    base: PolyEvalBaseSignature,
    inputs: &[AnalogValue],
) -> PolyEvalSignature {
    PolyEvalSignature {
        plan: base.plan,
        multiplier: base.multiplier,
        inputs: input_values(inputs),
    }
}

fn poly_eval_signature(ctx: &CmContext) -> CmResult<PolyEvalSignature> {
    let inputs = checked_inputs(ctx)?;
    Ok(poly_eval_signature_from_inputs(
        poly_eval_base_signature(ctx, inputs.len())?,
        inputs,
    ))
}

fn poly_eval_inputs_match(cached: &[Value], inputs: &[AnalogValue]) -> bool {
    cached.len() == inputs.len()
        && cached
            .iter()
            .zip(inputs)
            .all(|(cached, input)| *cached == input.value)
}

fn poly_eval_resource_matches(
    resource: &PolyEvalResource,
    base: PolyEvalBaseSignature,
    inputs: &[AnalogValue],
) -> bool {
    resource.signature.plan == base.plan
        && resource.signature.multiplier == base.multiplier
        && poly_eval_inputs_match(&resource.signature.inputs, inputs)
}

fn evaluate_poly(inputs: &[Value], plan: &PolyPlan, multiplier: Value) -> PolyEval {
    let mut value = plan.constant;
    let mut partials = vec![0.0; inputs.len()];

    for term in &plan.terms {
        let mut product = 1.0;
        for &(input_index, exponent) in &term.active_exponents {
            product *= evterm(inputs[input_index], exponent);
        }
        value += term.coefficient * product;

        for &(input_index, exponent) in &term.active_exponents {
            let mut partial_product = exponent as Value;
            for &(term_index, term_exponent) in &term.active_exponents {
                partial_product *= if term_index == input_index {
                    evterm(inputs[term_index], term_exponent - 1)
                } else {
                    evterm(inputs[term_index], term_exponent)
                };
            }
            partials[input_index] += term.coefficient * partial_product;
        }
    }

    value *= multiplier;
    for partial in &mut partials {
        *partial *= multiplier;
    }

    PolyEval { value, partials }
}

fn evaluate_context(ctx: &CmContext) -> CmResult<Arc<PolyEval>> {
    let inputs = checked_inputs(ctx)?;
    let base = poly_eval_base_signature(ctx, inputs.len())?;
    if let Some(resource) = ctx.resource::<PolyEvalResource>(POLY_EVAL_RESOURCE)
        && poly_eval_resource_matches(&resource, base, inputs)
    {
        return Ok(Arc::clone(&resource.result));
    }

    let signature = poly_eval_signature_from_inputs(base, inputs);
    let plan = poly_plan(ctx, signature.inputs.len())?;
    Ok(Arc::new(evaluate_poly(
        &signature.inputs,
        &plan,
        signature.multiplier,
    )))
}

fn evaluate_context_cached(ctx: &mut CmContext) -> CmResult<Arc<PolyEval>> {
    let inputs = checked_inputs(ctx)?;
    let base = poly_eval_base_signature(ctx, inputs.len())?;
    if let Some(resource) = ctx.resource::<PolyEvalResource>(POLY_EVAL_RESOURCE)
        && poly_eval_resource_matches(&resource, base, inputs)
    {
        return Ok(Arc::clone(&resource.result));
    }

    let signature = poly_eval_signature_from_inputs(base, inputs);
    let plan = cache_poly_plan(ctx, signature.inputs.len())?;
    let result = Arc::new(evaluate_poly(
        &signature.inputs,
        &plan,
        signature.multiplier,
    ));
    ctx.set_resource(
        POLY_EVAL_RESOURCE,
        Arc::new(PolyEvalResource {
            signature,
            result: Arc::clone(&result),
        }),
    );
    Ok(result)
}

impl CodeModel for Spice2Poly {
    fn name(&self) -> &str {
        "spice2poly"
    }

    fn description(&self) -> &str {
        "SPICE 2G6-compatible polynomial controlled source"
    }

    fn ports(&self) -> &[PortSpec] {
        ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        checked_coef(ctx)?;
        checked_multiplier(ctx)?;
        Ok(())
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        let result = evaluate_context_cached(ctx)?;
        ctx.set_output_with_partial("out", result.value, 0.0);
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
        match evaluate_context(ctx) {
            Ok(result) => result
                .partials
                .iter()
                .copied()
                .enumerate()
                .filter_map(|(index, partial)| {
                    (partial.is_finite() && partial != 0.0).then_some((
                        "in".to_string(),
                        index,
                        partial,
                    ))
                })
                .collect(),
            Err(_) => Vec::new(),
        }
    }
}

impl CodeModel for IcmSpice2Poly {
    fn name(&self) -> &str {
        "icm_spice2poly"
    }

    fn description(&self) -> &str {
        Spice2Poly.description()
    }

    fn ports(&self) -> &[PortSpec] {
        Spice2Poly.ports()
    }

    fn parameters(&self) -> &[ParamSpec] {
        Spice2Poly.parameters()
    }

    fn init(&self, ctx: &mut CmContext) -> CmResult<()> {
        Spice2Poly.init(ctx)
    }

    fn evaluate(&self, ctx: &mut CmContext) -> CmResult<()> {
        Spice2Poly.evaluate(ctx)
    }

    fn ac_gain(&self, ctx: &CmContext) -> Vec<Value> {
        Spice2Poly.ac_gain(ctx)
    }

    fn output_input_vector_partials(
        &self,
        ctx: &CmContext,
        output_port: &str,
    ) -> Vec<(String, usize, Value)> {
        Spice2Poly.output_input_vector_partials(ctx, output_port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn set_inputs(ctx: &mut CmContext, values: &[Value]) {
        ctx.set_input_analog_vector_from_fn("in", values.len(), |index| {
            AnalogValue::new(values[index])
        });
    }

    #[test]
    fn nxtpwr_matches_spice2_order_for_two_inputs() {
        let mut exponents = vec![0usize; 2];
        let mut sequence = Vec::new();
        for _ in 0..9 {
            nxtpwr(&mut exponents);
            sequence.push(exponents.clone());
        }

        assert_eq!(
            sequence,
            vec![
                vec![1, 0],
                vec![0, 1],
                vec![2, 0],
                vec![1, 1],
                vec![0, 2],
                vec![3, 0],
                vec![2, 1],
                vec![1, 2],
                vec![0, 3],
            ]
        );
    }

    #[test]
    fn nxtpwr_matches_spice2_order_for_three_inputs() {
        let mut exponents = vec![0usize; 3];
        let mut sequence = Vec::new();
        for _ in 0..10 {
            nxtpwr(&mut exponents);
            sequence.push(exponents.clone());
        }

        assert_eq!(
            sequence,
            vec![
                vec![1, 0, 0],
                vec![0, 1, 0],
                vec![0, 0, 1],
                vec![2, 0, 0],
                vec![1, 1, 0],
                vec![1, 0, 1],
                vec![0, 2, 0],
                vec![0, 1, 1],
                vec![0, 0, 2],
                vec![3, 0, 0],
            ]
        );
    }

    #[test]
    fn polynomial_value_and_partials_use_multiplier() {
        let inputs = vec![2.0, 3.0];
        let plan = build_poly_plan(inputs.len(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
            .expect("valid polynomial plan");
        let result = evaluate_poly(&inputs, &plan, 2.0);

        assert!((result.value - 228.0).abs() < 1.0e-12);
        assert!((result.partials[0] - 66.0).abs() < 1.0e-12);
        assert!((result.partials[1] - 98.0).abs() < 1.0e-12);
    }

    #[test]
    fn polynomial_plan_skips_zero_terms_without_changing_exponent_order() {
        let inputs = vec![2.0, 3.0];
        let plan = build_poly_plan(
            inputs.len(),
            &[1.0, 0.0, 3.0, 0.0, 5.0, 0.0, 7.0, 0.0, 0.0, 10.0],
        )
        .expect("valid sparse polynomial plan");

        assert_eq!(
            plan.terms
                .iter()
                .map(|term| (term.coefficient, term.active_exponents.as_slice()))
                .collect::<Vec<_>>(),
            vec![
                (3.0, &[(1, 1)][..]),
                (5.0, &[(0, 1), (1, 1)][..]),
                (7.0, &[(0, 3)][..]),
                (10.0, &[(1, 3)][..]),
            ]
        );

        let result = evaluate_poly(&inputs, &plan, 1.0);
        assert!((result.value - 366.0).abs() < 1.0e-12);
        assert!((result.partials[0] - 99.0).abs() < 1.0e-12);
        assert!((result.partials[1] - 283.0).abs() < 1.0e-12);
    }

    #[test]
    fn polynomial_partials_handle_zero_sparse_inputs() {
        let inputs = vec![0.0, 3.0];
        let plan = build_poly_plan(inputs.len(), &[0.0, 0.0, 0.0, 0.0, 1.0])
            .expect("valid sparse polynomial plan");
        let result = evaluate_poly(&inputs, &plan, 1.0);

        assert_eq!(plan.terms[0].active_exponents, vec![(0, 1), (1, 1)]);
        assert!((result.value - 0.0).abs() < 1.0e-12);
        assert!((result.partials[0] - 3.0).abs() < 1.0e-12);
        assert!((result.partials[1] - 0.0).abs() < 1.0e-12);
    }

    #[test]
    fn plan_cache_reloads_when_coef_or_width_changes() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("coef", vec![1.0, 2.0, 3.0]);

        let first = cache_poly_plan(&mut ctx, 2).expect("valid polynomial plan");
        let second = cache_poly_plan(&mut ctx, 2).expect("cached polynomial plan");
        assert!(Arc::ptr_eq(&first, &second));

        ctx.set_real_vector_param("unrelated", vec![1.0, 2.0]);
        let after_unrelated =
            cache_poly_plan(&mut ctx, 2).expect("unrelated vector preserves polynomial plan");
        assert!(Arc::ptr_eq(&first, &after_unrelated));

        let wider = cache_poly_plan(&mut ctx, 3).expect("reloaded polynomial plan");
        assert!(!Arc::ptr_eq(&first, &wider));

        ctx.set_real_vector_param("coef", vec![1.0, 2.0, 4.0]);
        let updated = cache_poly_plan(&mut ctx, 3).expect("updated polynomial plan");
        assert!(!Arc::ptr_eq(&wider, &updated));
    }

    #[test]
    fn poly_eval_cache_reuses_current_result_until_inputs_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("coef", vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0]);
        ctx.set_param("m", 2.0);
        set_inputs(&mut ctx, &[2.0, 3.0]);

        let initial = evaluate_context_cached(&mut ctx).expect("evaluation caches");
        assert_eq!(
            initial.as_ref(),
            &PolyEval {
                value: 228.0,
                partials: vec![66.0, 98.0],
            }
        );

        let cached_initial = evaluate_context(&ctx).expect("read-only initial cache");
        assert!(
            Arc::ptr_eq(&initial, &cached_initial),
            "read-only spice2poly eval should reuse the cached Arc"
        );
        let mutable_cached_initial =
            evaluate_context_cached(&mut ctx).expect("mutable initial cache");
        assert!(
            Arc::ptr_eq(&initial, &mutable_cached_initial),
            "mutable spice2poly eval should reuse the cached Arc"
        );

        let signature = poly_eval_signature(&ctx).expect("current eval signature");
        let sentinel = Arc::new(PolyEval {
            value: 123.0,
            partials: vec![7.0, 11.0],
        });
        ctx.set_resource(
            POLY_EVAL_RESOURCE,
            Arc::new(PolyEvalResource {
                signature,
                result: Arc::clone(&sentinel),
            }),
        );

        let cached_sentinel = evaluate_context(&ctx).expect("read-only path reuses cache");
        assert!(
            Arc::ptr_eq(&cached_sentinel, &sentinel),
            "matching read-only spice2poly eval signatures should reuse the cached Arc"
        );
        let mutable_cached_sentinel =
            evaluate_context_cached(&mut ctx).expect("mutable path reuses cache");
        assert!(
            Arc::ptr_eq(&mutable_cached_sentinel, &sentinel),
            "matching mutable spice2poly eval signatures should reuse the cached Arc"
        );
        assert_eq!(
            Spice2Poly.output_input_vector_partials(&ctx, "out"),
            vec![("in".to_string(), 0, 7.0), ("in".to_string(), 1, 11.0)]
        );

        set_inputs(&mut ctx, &[2.0, 4.0]);
        let updated = evaluate_context(&ctx).expect("changed inputs invalidate cache");
        assert_ne!(updated.as_ref(), sentinel.as_ref());
        assert_eq!(
            updated.as_ref(),
            evaluate_context_cached(&mut ctx)
                .expect("direct updated eval")
                .as_ref()
        );
    }

    #[test]
    fn poly_eval_resource_match_compares_current_input_slice() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("coef", vec![1.0, 2.0, 3.0, 4.0]);
        ctx.set_param("m", 2.0);
        set_inputs(&mut ctx, &[2.0, 3.0]);

        let signature = poly_eval_signature(&ctx).expect("current eval signature");
        let inputs = checked_inputs(&ctx).expect("current inputs");
        let base = poly_eval_base_signature(&ctx, inputs.len()).expect("current base signature");
        let resource = PolyEvalResource {
            signature,
            result: Arc::new(PolyEval {
                value: 123.0,
                partials: vec![7.0, 11.0],
            }),
        };

        assert!(
            poly_eval_resource_matches(&resource, base, inputs),
            "matching spice2poly inputs should hit the eval cache without rebuilding a signature"
        );

        set_inputs(&mut ctx, &[2.0, 4.0]);
        let changed_inputs = checked_inputs(&ctx).expect("changed inputs");
        assert!(
            !poly_eval_resource_matches(&resource, base, changed_inputs),
            "changed spice2poly inputs must invalidate the eval cache"
        );
    }

    #[test]
    fn poly_eval_cache_invalidates_when_multiplier_or_coefficients_change() {
        let mut ctx = CmContext::new();
        ctx.set_real_vector_param("coef", vec![1.0, 2.0, 3.0]);
        ctx.set_param("m", 1.0);
        set_inputs(&mut ctx, &[2.0, 3.0]);

        let signature = poly_eval_signature(&ctx).expect("current eval signature");
        let sentinel = Arc::new(PolyEval {
            value: 99.0,
            partials: vec![5.0, 6.0],
        });
        ctx.set_resource(
            POLY_EVAL_RESOURCE,
            Arc::new(PolyEvalResource {
                signature,
                result: Arc::clone(&sentinel),
            }),
        );

        let cached_sentinel = evaluate_context(&ctx).expect("matching eval reuses cache");
        assert!(
            Arc::ptr_eq(&cached_sentinel, &sentinel),
            "matching spice2poly eval should reuse the cached Arc"
        );

        ctx.set_param("m", 3.0);
        let changed_multiplier =
            evaluate_context(&ctx).expect("changed multiplier invalidates cache");
        assert_ne!(changed_multiplier.as_ref(), sentinel.as_ref());

        let signature = poly_eval_signature(&ctx).expect("multiplier signature");
        ctx.set_resource(
            POLY_EVAL_RESOURCE,
            Arc::new(PolyEvalResource {
                signature,
                result: Arc::clone(&sentinel),
            }),
        );
        ctx.set_real_vector_param("coef", vec![1.0, 2.0, 4.0]);
        let changed_coef = evaluate_context(&ctx).expect("changed coefficients invalidate cache");
        assert_ne!(changed_coef.as_ref(), sentinel.as_ref());
    }
}

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

#[derive(Debug, Default)]
pub struct Spice2Poly;

#[derive(Debug, Default)]
pub struct IcmSpice2Poly;

#[derive(Debug, Clone)]
struct PolyEval {
    value: Value,
    partials: Vec<Value>,
}

#[derive(Debug, Clone)]
struct PolyTerm {
    coefficient: Value,
    exponents: Vec<usize>,
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
            exponents: exponents.clone(),
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

fn evaluate_poly(inputs: &[AnalogValue], plan: &PolyPlan, multiplier: Value) -> PolyEval {
    let mut value = plan.constant;
    let mut partials = vec![0.0; inputs.len()];

    for term in &plan.terms {
        let mut product = 1.0;
        for &(input_index, exponent) in &term.active_exponents {
            product *= evterm(inputs[input_index].value, exponent);
        }
        value += term.coefficient * product;

        for &(input_index, exponent) in &term.active_exponents {
            let mut partial_product = exponent as Value;
            for &(term_index, term_exponent) in &term.active_exponents {
                partial_product *= if term_index == input_index {
                    evterm(inputs[term_index].value, term_exponent - 1)
                } else {
                    evterm(inputs[term_index].value, term_exponent)
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

fn evaluate_context(ctx: &CmContext) -> CmResult<PolyEval> {
    let inputs = checked_inputs(ctx)?;
    let plan = poly_plan(ctx, inputs.len())?;
    let multiplier = checked_multiplier(ctx)?;
    Ok(evaluate_poly(inputs, &plan, multiplier))
}

fn evaluate_context_cached(ctx: &mut CmContext) -> CmResult<PolyEval> {
    let input_count = checked_inputs(ctx)?.len();
    let plan = cache_poly_plan(ctx, input_count)?;
    let inputs = checked_inputs(ctx)?;
    let multiplier = checked_multiplier(ctx)?;
    Ok(evaluate_poly(inputs, &plan, multiplier))
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
                .into_iter()
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
        let inputs = vec![AnalogValue::new(2.0), AnalogValue::new(3.0)];
        let plan = build_poly_plan(inputs.len(), &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0])
            .expect("valid polynomial plan");
        let result = evaluate_poly(&inputs, &plan, 2.0);

        assert!((result.value - 228.0).abs() < 1.0e-12);
        assert!((result.partials[0] - 66.0).abs() < 1.0e-12);
        assert!((result.partials[1] - 98.0).abs() < 1.0e-12);
    }

    #[test]
    fn polynomial_plan_skips_zero_terms_without_changing_exponent_order() {
        let inputs = vec![AnalogValue::new(2.0), AnalogValue::new(3.0)];
        let plan = build_poly_plan(
            inputs.len(),
            &[1.0, 0.0, 3.0, 0.0, 5.0, 0.0, 7.0, 0.0, 0.0, 10.0],
        )
        .expect("valid sparse polynomial plan");

        assert_eq!(
            plan.terms
                .iter()
                .map(|term| (term.coefficient, term.exponents.as_slice()))
                .collect::<Vec<_>>(),
            vec![
                (3.0, &[0, 1][..]),
                (5.0, &[1, 1][..]),
                (7.0, &[3, 0][..]),
                (10.0, &[0, 3][..]),
            ]
        );
        assert_eq!(
            plan.terms
                .iter()
                .map(|term| term.active_exponents.as_slice())
                .collect::<Vec<_>>(),
            vec![
                &[(1, 1)][..],
                &[(0, 1), (1, 1)][..],
                &[(0, 3)][..],
                &[(1, 3)][..],
            ]
        );

        let result = evaluate_poly(&inputs, &plan, 1.0);
        assert!((result.value - 366.0).abs() < 1.0e-12);
        assert!((result.partials[0] - 99.0).abs() < 1.0e-12);
        assert!((result.partials[1] - 283.0).abs() < 1.0e-12);
    }

    #[test]
    fn polynomial_partials_handle_zero_sparse_inputs() {
        let inputs = vec![AnalogValue::new(0.0), AnalogValue::new(3.0)];
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
}

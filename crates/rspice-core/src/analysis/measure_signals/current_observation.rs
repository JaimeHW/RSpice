//! Resolve the singular part of a retained current using the output grammar.
//!
//! `None` retains the historical sampled-waveform contract. Once a result
//! carries an impulse section, every requested current must have explicit,
//! complete coverage. Empty complete histories and missing histories differ.

use super::*;
use crate::netlist::expr::{BinOpKind, UnaryOpKind};
use crate::{CurrentImpulseOwner, CurrentImpulseTrace};

#[derive(Debug, thiserror::Error)]
pub(crate) enum CurrentObservationError {
    #[error("current observation evaluation aborted")]
    Aborted,
    #[error("{detail}")]
    Invalid { detail: String },
}

impl From<CurrentObservationError> for crate::analysis::fourier::FourierError {
    fn from(error: CurrentObservationError) -> Self {
        match error {
            CurrentObservationError::Aborted => Self::Aborted,
            CurrentObservationError::Invalid { detail } => Self::CurrentObservation { detail },
        }
    }
}

#[derive(Clone, Copy)]
pub(crate) struct CurrentImpulseContribution<'a> {
    pub(crate) trace: &'a CurrentImpulseTrace,
    pub(crate) weight: Value,
}

#[derive(Default)]
pub(crate) struct ResolvedCurrentObservation<'a> {
    pub(crate) terms: Vec<CurrentImpulseContribution<'a>>,
    pub(crate) observes_current: bool,
}

fn failure(detail: impl Into<String>) -> CurrentObservationError {
    CurrentObservationError::Invalid {
        detail: detail.into(),
    }
}

#[derive(Default)]
struct Form<'a> {
    // Some only for a time-independent, real, finite expression. A sampled
    // voltage or an impulse-free current is still time dependent.
    constant: Option<Value>,
    terms: Vec<CurrentImpulseContribution<'a>>,
}

impl Form<'_> {
    fn scale(&mut self, weight: Value) -> Result<(), CurrentObservationError> {
        for term in &mut self.terms {
            let scaled = term.weight * weight;
            if !scaled.is_finite() || (scaled == 0.0 && term.weight != 0.0 && weight != 0.0) {
                return Err(failure(
                    "current impulse expression coefficient is not representable",
                ));
            }
            term.weight = scaled;
        }
        self.terms.retain(|term| term.weight != 0.0);
        Ok(())
    }
}

struct Resolver<'a, 'b> {
    traces: &'a [CurrentImpulseTrace],
    // None denotes an ambiguous alias, independent of insertion order.
    aliases: HashMap<String, Option<usize>>,
    retained_currents: HashSet<String>,
    params: &'b crate::netlist::ParamContext,
    abort: &'b dyn AbortSignal,
    extent: (Value, Value),
    window: (Value, Value),
    observes_current: std::cell::Cell<bool>,
}

impl<'a> Resolver<'a, '_> {
    fn probe(&self, authored: &str) -> Result<Form<'a>, CurrentObservationError> {
        let canonical = canonical_measure_signal_name(authored);
        let (lookup, projection) = match split_equation_output_operator(&canonical) {
            Some((prefix, args))
                if !self.aliases.contains_key(&canonical)
                    && is_current_projection_accessor(&prefix.to_ascii_uppercase()) =>
            {
                let [device] = args.as_slice() else {
                    return Err(failure(format!("invalid current probe '{authored}'")));
                };
                (
                    canonical_measure_signal_name(&format!("I({device})")),
                    Some(prefix.to_ascii_uppercase()),
                )
            }
            _ => (canonical.clone(), None),
        };
        let trace = match self.aliases.get(&lookup) {
            Some(Some(index)) => self.traces.get(*index),
            Some(None) => {
                return Err(failure(format!(
                    "current impulse alias '{authored}' is ambiguous"
                )));
            }
            None => None,
        };
        let Some(trace) = trace else {
            let current = projection.is_some()
                || split_equation_output_operator(&canonical).is_some_and(|(prefix, _)| {
                    is_current_output_accessor(&prefix.to_ascii_uppercase())
                })
                || match crate::netlist::parse_save_probe(authored) {
                    Some(SaveSignal::DeviceParam { param, .. }) => current_parameter(&param),
                    _ => self.retained_currents.contains(&canonical),
                };
            if current {
                return Err(failure(format!(
                    "current '{authored}' has no complete impulse history"
                )));
            }
            if split_equation_output_operator(&canonical).is_some_and(|(prefix, _)| {
                matches!(prefix.to_ascii_uppercase().as_str(), "P" | "W")
            }) {
                return Err(failure(format!(
                    "power probe '{authored}' needs a defined voltage/current impulse product"
                )));
            }
            return Ok(Form::default());
        };
        self.observes_current.set(true);
        if !trace.complete {
            return Err(failure(format!(
                "current '{authored}' has an incomplete impulse history"
            )));
        }
        trace
            .validate(self.extent.0, self.extent.1)
            .map_err(failure)?;
        let has_impulses = trace
            .points
            .iter()
            .any(|point| point.time > self.window.0 && point.time <= self.window.1);
        if projection
            .as_deref()
            .is_some_and(|prefix| !matches!(prefix, "I" | "IR"))
            && has_impulses
        {
            return Err(failure(format!(
                "current projection '{authored}' is not defined for charge impulses"
            )));
        }
        let mut form = Form::default();
        if has_impulses {
            form.terms
                .try_reserve_exact(1)
                .map_err(|_| failure("cannot allocate current impulse terms"))?;
            form.terms
                .push(CurrentImpulseContribution { trace, weight: 1.0 });
        }
        Ok(form)
    }

    fn expression(&self, expression: &NetExpr) -> Result<Form<'a>, CurrentObservationError> {
        if self.abort.is_aborted() {
            return Err(CurrentObservationError::Aborted);
        }
        let mut form = match expression {
            NetExpr::Param(name) => {
                if self.params.get_complex(name).is_some()
                    && !matches!(
                        name.to_ascii_uppercase().as_str(),
                        "TIME" | "FREQ" | "FREQUENCY" | "HERTZ"
                    )
                {
                    Form {
                        constant: self.constant(expression),
                        ..Form::default()
                    }
                } else {
                    self.probe(name)?
                }
            }
            NetExpr::Number(_) | NetExpr::ComplexNumber(_) => Form {
                constant: self.constant(expression),
                ..Form::default()
            },
            NetExpr::StringLiteral(_) => Form::default(),
            NetExpr::FnCall { name, args }
                if is_equation_probe_accessor(name)
                    || is_equation_noise_accessor(name)
                    || is_equation_generic_output_accessor(name) =>
            {
                let arguments = args
                    .iter()
                    .map(|arg| {
                        equation_probe_argument(Some(arg)).ok_or_else(|| {
                            failure(format!("invalid impulse probe argument in {name}()"))
                        })
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.probe(&format!("{name}({})", arguments.join(",")))?
            }
            NetExpr::UnaryOp { op, operand } => {
                let mut child = self.expression(operand)?;
                if !child.terms.is_empty() {
                    match op {
                        UnaryOpKind::Pos => {}
                        UnaryOpKind::Neg => child.scale(-1.0)?,
                        UnaryOpKind::Not => {
                            return Err(failure(
                                "logical operations on current impulses are undefined",
                            ));
                        }
                    }
                }
                child.constant = child.constant.and_then(|_| self.constant(expression));
                child
            }
            NetExpr::BinOp { op, left, right } => {
                let mut left = self.expression(left)?;
                let mut right = self.expression(right)?;
                let constant = if left.constant.is_some() && right.constant.is_some() {
                    self.constant(expression)
                } else {
                    None
                };
                if !left.terms.is_empty() || !right.terms.is_empty() {
                    match op {
                        BinOpKind::Add | BinOpKind::Sub => {
                            if matches!(op, BinOpKind::Sub) {
                                right.scale(-1.0)?;
                            }
                            left.terms
                                .try_reserve(right.terms.len())
                                .map_err(|_| failure("cannot allocate current impulse terms"))?;
                            left.terms.extend(right.terms);
                        }
                        BinOpKind::Mul if left.terms.is_empty() && left.constant.is_some() => {
                            right.scale(
                                left.constant
                                    .ok_or_else(|| failure("missing constant multiplier"))?,
                            )?;
                            left = right;
                        }
                        BinOpKind::Mul if right.terms.is_empty() && right.constant.is_some() => {
                            left.scale(
                                right
                                    .constant
                                    .ok_or_else(|| failure("missing constant multiplier"))?,
                            )?;
                        }
                        BinOpKind::Div
                            if right.terms.is_empty()
                                && right.constant.is_some_and(|v| v != 0.0) =>
                        {
                            let divisor = right
                                .constant
                                .ok_or_else(|| failure("missing constant divisor"))?;
                            for term in &mut left.terms {
                                let scaled = term.weight / divisor;
                                if !scaled.is_finite() || (scaled == 0.0 && term.weight != 0.0) {
                                    return Err(failure(
                                        "current impulse expression coefficient is not representable",
                                    ));
                                }
                                term.weight = scaled;
                            }
                        }
                        _ => {
                            return Err(failure(
                                "current impulses require an affine expression with time-independent real coefficients; nonlinear or time-varying impulse operations are unsupported",
                            ));
                        }
                    }
                }
                left.constant = constant;
                left
            }
            NetExpr::FnCall { name, args } => {
                // Statistical calls must not be sampled again while deriving
                // a coefficient, nor certified time independent merely because
                // their explicit arguments are constants.
                let mut all_constant = !matches!(
                    name.to_ascii_uppercase().as_str(),
                    "RAND" | "RANDOM" | "GAUSS" | "AGAUSS" | "UNIF" | "AUNIF"
                );
                for arg in args {
                    let child = self.expression(arg)?;
                    if !child.terms.is_empty() {
                        return Err(failure(
                            "functions of current impulses are undefined; use an affine current expression",
                        ));
                    }
                    all_constant &= child.constant.is_some();
                }
                Form {
                    constant: all_constant.then(|| self.constant(expression)).flatten(),
                    ..Form::default()
                }
            }
        };
        // Coalesce repeated owners before the transform. Exact cancellation
        // must not leave two large terms whose rounding hides another owner.
        form.terms
            .sort_unstable_by_key(|term| std::ptr::from_ref(term.trace));
        let mut merged: Vec<CurrentImpulseContribution<'a>> = Vec::new();
        merged
            .try_reserve_exact(form.terms.len())
            .map_err(|_| failure("cannot allocate current impulse terms"))?;
        for term in form.terms {
            if let Some(last) = merged.last_mut()
                && std::ptr::eq(last.trace, term.trace)
            {
                last.weight += term.weight;
                if !last.weight.is_finite() {
                    return Err(failure(
                        "current impulse expression coefficient is not representable",
                    ));
                }
            } else {
                merged.push(term);
            }
        }
        merged.retain(|term| term.weight != 0.0);
        form.terms = merged;
        Ok(form)
    }

    fn constant(&self, expression: &NetExpr) -> Option<Value> {
        // Use the same parameter/dialect evaluator, retaining complex and
        // nonfinite failures instead of projecting them into coefficients.
        crate::netlist::expr::evaluate_complex_raw(expression, self.params)
            .ok()
            .filter(|value| is_real(*value) && value.re.is_finite())
            .map(|value| value.re)
    }
}

pub(super) fn current_parameter(parameter: &str) -> bool {
    let parameter = parameter.to_ascii_uppercase();
    parameter == "I" || is_device_lead_current_accessor(&parameter)
}

pub(crate) fn resolve<'a>(
    netlist: Option<&Netlist>,
    result: &'a TransientResult,
    spec: &str,
    window: (Value, Value),
    abort: &dyn AbortSignal,
) -> Result<Vec<CurrentImpulseContribution<'a>>, CurrentObservationError> {
    resolve_with_coverage(netlist, result, spec, window, abort).map(|resolved| resolved.terms)
}

pub(crate) fn resolve_with_coverage<'a>(
    netlist: Option<&Netlist>,
    result: &'a TransientResult,
    spec: &str,
    window: (Value, Value),
    abort: &dyn AbortSignal,
) -> Result<ResolvedCurrentObservation<'a>, CurrentObservationError> {
    if abort.is_aborted() {
        return Err(CurrentObservationError::Aborted);
    }
    let Some(traces) = result.current_impulses.as_deref() else {
        return Ok(ResolvedCurrentObservation::default());
    };
    let mut aliases: HashMap<String, Option<usize>> = HashMap::new();
    aliases
        .try_reserve(
            traces
                .len()
                .checked_mul(4)
                .ok_or_else(|| failure("current impulse alias capacity overflow"))?,
        )
        .map_err(|_| failure("cannot allocate current impulse aliases"))?;
    for (index, trace) in traces.iter().enumerate() {
        if abort.is_aborted() {
            return Err(CurrentObservationError::Aborted);
        }
        let names = match &trace.owner {
            CurrentImpulseOwner::Branch { branch_name } => vec![format!("I({branch_name})")],
            CurrentImpulseOwner::DeviceLead {
                device_name,
                parameter,
            } => vec![
                format!("@{device_name}[{parameter}]"),
                format!("{device_name}:{parameter}"),
                format!("{parameter}({device_name})"),
                format!("N({device_name}:{parameter})"),
            ],
        };
        for name in names {
            aliases
                .entry(canonical_measure_signal_name(&name))
                .and_modify(|entry| {
                    if *entry != Some(index) {
                        *entry = None;
                    }
                })
                .or_insert(Some(index));
        }
    }
    let empty_params = crate::netlist::ParamContext::new();
    let mut retained_currents = HashSet::new();
    retained_currents
        .try_reserve(
            result
                .device_op_traces
                .len()
                .checked_mul(4)
                .ok_or_else(|| failure("current trace alias capacity overflow"))?,
        )
        .map_err(|_| failure("cannot allocate current trace aliases"))?;
    for trace in &result.device_op_traces {
        if abort.is_aborted() {
            return Err(CurrentObservationError::Aborted);
        }
        if current_parameter(&trace.parameter) {
            let device = &trace.device_name;
            let param = &trace.parameter;
            for alias in [
                format!("@{device}[{param}]"),
                format!("{device}:{param}"),
                format!("{param}({device})"),
                format!("N({device}:{param})"),
            ] {
                retained_currents.insert(canonical_measure_signal_name(&alias));
            }
        }
    }
    let params = netlist.map_or(&empty_params, |netlist| &netlist.params);
    let resolver = Resolver {
        traces,
        aliases,
        retained_currents,
        params,
        abort,
        extent: (
            result.time.first().copied().ok_or(failure(
                "current observations require a nonempty time extent",
            ))?,
            result.time.last().copied().ok_or(failure(
                "current observations require a nonempty time extent",
            ))?,
        ),
        window,
        observes_current: std::cell::Cell::new(false),
    };
    let trimmed = spec.trim();
    let terms = if let Some(body) = trimmed
        .strip_prefix('{')
        .and_then(|body| body.strip_suffix('}'))
    {
        let protected = params
            .all_params()
            .into_iter()
            .map(|(name, _)| name.to_ascii_uppercase())
            .collect();
        let expanded = crate::netlist::expr::expand_output_user_functions_with_abort(
            body, params, &protected, abort,
        )
        .map_err(|error| match error {
            crate::netlist::expr::BehavioralPreparationError::Aborted => {
                CurrentObservationError::Aborted
            }
            crate::netlist::expr::BehavioralPreparationError::Semantic(detail) => failure(detail),
        })?;
        let parsed = crate::netlist::expr::parse_expression_with_abort(&expanded, abort).map_err(
            |error| match error {
                crate::netlist::expr::ParseExpressionWithAbortError::Aborted => {
                    CurrentObservationError::Aborted
                }
                crate::netlist::expr::ParseExpressionWithAbortError::Parse(error) => {
                    failure(error.to_string())
                }
            },
        )?;
        parsed
            .ensure_stack_safe_depth()
            .map_err(|error| failure(error.to_string()))?;
        resolver.expression(&parsed)?.terms
    } else {
        resolver.probe(trimmed)?.terms
    };
    if abort.is_aborted() {
        return Err(CurrentObservationError::Aborted);
    }
    Ok(ResolvedCurrentObservation {
        terms,
        observes_current: resolver.observes_current.get(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::CurrentImpulsePoint;
    use crate::analysis::{FourierAnalysis, FourierConfig, FourierError};

    fn fixture() -> TransientResult {
        let time = (0..=256).map(|n| n as Value / 128.0).collect::<Vec<_>>();
        TransientResult {
            step_sizes: vec![1.0 / 128.0; time.len()],
            voltages: vec![vec![3.0; time.len()]],
            branch_currents: vec![vec![0.0; time.len()]],
            device_op_traces: vec![TransientDeviceOpTrace {
                device_name: "X1.Q1".into(),
                parameter: "ic".into(),
                values: vec![0.0; time.len()],
            }],
            time,
            num_nodes: 1,
            node_names: vec!["out".into()],
            branch_names: vec!["X1.V1".into()],
            digital_traces: vec![],
            digital_buses: vec![],
            real_traces: vec![],
            store_traces: vec![],
            fft_results: vec![],
            current_impulses: Some(vec![
                CurrentImpulseTrace {
                    owner: CurrentImpulseOwner::Branch {
                        branch_name: "X1.V1".into(),
                    },
                    complete: true,
                    points: vec![CurrentImpulsePoint {
                        time: 1.25,
                        charge_coulombs: 0.002,
                    }],
                },
                CurrentImpulseTrace {
                    owner: CurrentImpulseOwner::DeviceLead {
                        device_name: "X1.Q1".into(),
                        parameter: "ic".into(),
                    },
                    complete: true,
                    points: vec![CurrentImpulsePoint {
                        time: 1.25,
                        charge_coulombs: -0.002,
                    }],
                },
            ]),
        }
    }

    fn spectrum(
        result: &TransientResult,
        spec: &str,
        netlist: Option<&Netlist>,
    ) -> Result<crate::analysis::FourierResult, FourierError> {
        let finite = evaluate_transient_probe_with_abort(netlist, result, spec, &NoAbort)
            .map_err(|error| FourierError::from(failure(error.to_string())))?;
        FourierAnalysis::new(FourierConfig::new(1.0).with_harmonics(4))
            .analyze_transient_output_with_abort(netlist, result, spec, &finite, &NoAbort)
    }

    fn close(actual: Value, expected: Value) {
        assert!(
            (actual - expected).abs() <= 2e-14 * expected.abs().max(1e-3),
            "{actual} != {expected}"
        );
    }

    #[test]
    fn fourier_current_impulse_has_analytic_charge_phase_and_all_aliases() {
        let result = fixture();
        // A 1 mF capacitor's +2 V jump transfers +2 mC. The source's
        // orientation-reversed terminal has the opposite signed charge.
        for spec in ["I(X1.V1)", "i( x1:v1 )", "IR(X1.V1)"] {
            let output = spectrum(&result, spec, None).unwrap();
            close(output.dc_component, 0.002);
            for n in 1..=4 {
                close(output.harmonics[n].magnitude, 0.004);
                let angle = output.harmonics[n].phase.to_radians();
                close(
                    output.harmonics[n].magnitude * angle.cos(),
                    0.004 * (-(n as Value) * std::f64::consts::FRAC_PI_2).cos(),
                );
                close(
                    output.harmonics[n].magnitude * angle.sin(),
                    0.004 * (-(n as Value) * std::f64::consts::FRAC_PI_2).sin(),
                );
            }
            close(output.thd.unwrap(), 3.0_f64.sqrt() * 100.0);
        }
        for spec in [
            "IC(X1.Q1)",
            "ic(x1:q1)",
            "@X1.Q1[IC]",
            "x1:q1:ic",
            "N(X1.Q1:ic)",
        ] {
            let output = spectrum(&result, spec, None).unwrap();
            close(output.dc_component, -0.002);
            close(output.harmonics[1].magnitude, 0.004);
            close(output.harmonics[1].phase, 90.0);
        }
    }

    #[test]
    fn fourier_current_impulse_window_counts_periodic_endpoint_once() {
        let mut result = fixture();
        result.current_impulses.as_mut().unwrap()[0].points = vec![
            CurrentImpulsePoint {
                time: 0.0,
                charge_coulombs: 10.0,
            },
            CurrentImpulsePoint {
                time: 1.0,
                charge_coulombs: 0.003,
            },
            CurrentImpulsePoint {
                time: 2.0,
                charge_coulombs: 0.002,
            },
        ];
        let output = spectrum(&result, "I(X1.V1)", None).unwrap();
        close(output.dc_component, 0.002);
        close(output.harmonics[1].magnitude, 0.004);
        close(output.harmonics[1].phase, 0.0);
        // The preceding period owns the seam, which the next period excludes.
        result.time.truncate(129);
        result.branch_currents[0].truncate(129);
        result.current_impulses.as_mut().unwrap()[0].points.pop();
        let output = spectrum(&result, "I(X1.V1)", None).unwrap();
        close(output.dc_component, 0.003);
    }

    #[test]
    fn fourier_current_impulse_combines_finite_and_singular_coefficients() {
        let mut result = fixture();
        result.branch_currents[0].fill(1.0);
        let output = spectrum(&result, "I(X1.V1)", None).unwrap();
        close(output.dc_component, 1.002);
        close(output.harmonics[1].magnitude, 0.004);
        close(output.harmonics[1].phase, -90.0);
    }

    #[test]
    fn fourier_current_impulse_affine_functions_parameters_and_owner_cancellation() {
        let result = fixture();
        let netlist = Netlist::parse(
            "affine\n.param gain=3\n.func twice(x) {2*x}\nV1 out 0 0\nR1 out 0 1k\n.end\n",
        )
        .unwrap();
        for spec in [
            "{gain*I(X1.V1)+IC(X1.Q1)}",
            "{twice(I(X1.V1))}",
            "{I(X1.V1)/0.5}",
        ] {
            let output = spectrum(&result, spec, Some(&netlist)).unwrap();
            close(output.dc_component, 0.004);
            close(output.harmonics[1].magnitude, 0.008);
        }
        let zero = spectrum(&result, "{I(X1.V1)+IC(X1.Q1)}", Some(&netlist)).unwrap();
        assert_eq!(zero.dc_component, 0.0);
        assert!(zero.harmonics.iter().all(|h| h.magnitude == 0.0));
        let finite = spectrum(&result, "{I(X1.V1)-I(X1.V1)+V(out)}", Some(&netlist)).unwrap();
        close(finite.dc_component, 3.0);
    }

    #[test]
    fn fourier_current_impulse_refuses_undefined_algebra_and_incomplete_coverage() {
        let mut result = fixture();
        let netlist = Netlist::parse("nonlinear\nV1 out 0 0\nR1 out 0 1k\n.end\n").unwrap();
        for spec in [
            "{ABS(I(X1.V1))}",
            "{I(X1.V1)*I(X1.V1)}",
            "{V(out)*I(X1.V1)}",
            "{TIME*I(X1.V1)}",
            "IM(X1.V1)",
        ] {
            assert!(
                matches!(
                    spectrum(&result, spec, Some(&netlist)),
                    Err(FourierError::CurrentObservation { .. })
                ),
                "{spec}"
            );
        }
        result.current_impulses.as_mut().unwrap()[0].complete = false;
        assert!(
            spectrum(&result, "I(X1.V1)", None)
                .unwrap_err()
                .to_string()
                .contains("incomplete")
        );
        result.current_impulses = Some(vec![]);
        assert!(
            spectrum(&result, "I(X1.V1)", None)
                .unwrap_err()
                .to_string()
                .contains("no complete")
        );
        assert!(spectrum(&result, "V(out)", None).is_ok());
        result.current_impulses = None;
        assert_eq!(
            spectrum(&result, "I(X1.V1)", None).unwrap().dc_component,
            0.0
        );
    }

    #[test]
    fn fourier_current_impulse_complete_zero_allows_regular_nonlinear_current() {
        let mut result = fixture();
        result.current_impulses.as_mut().unwrap()[0].points.clear();
        result.branch_currents[0].fill(-2.0);
        let netlist = Netlist::parse("regular\nV1 out 0 0\n.end\n").unwrap();
        close(
            spectrum(&result, "{ABS(I(X1.V1))}", Some(&netlist))
                .unwrap()
                .dc_component,
            2.0,
        );
        result.branch_currents[0].clear();
        assert!(spectrum(&result, "I(X1.V1)", None).is_err());
    }

    #[test]
    fn fourier_current_impulse_outside_window_does_not_block_regular_algebra() {
        let mut result = fixture();
        result.current_impulses.as_mut().unwrap()[0].points[0].time = 0.25;
        result.branch_currents[0].fill(-2.0);
        let netlist = Netlist::parse("regular final period\nV1 out 0 0\n.end\n").unwrap();
        close(
            spectrum(&result, "{ABS(I(X1.V1))}", Some(&netlist))
                .unwrap()
                .dc_component,
            2.0,
        );
        close(
            spectrum(&result, "IM(X1.V1)", None).unwrap().dc_component,
            2.0,
        );
    }

    #[test]
    fn fourier_current_impulse_raw_names_bind_inventory_without_guessing_node_suffixes() {
        let mut result = fixture();
        result.node_names[0] = "x1.ic".into();
        result.current_impulses = Some(vec![]);
        close(spectrum(&result, "x1:ic", None).unwrap().dc_component, 3.0);
        assert!(
            spectrum(&result, "x1:q1:ic", None)
                .unwrap_err()
                .to_string()
                .contains("no complete impulse history")
        );
    }

    #[test]
    fn fourier_current_impulse_resume_seam_matches_uninterrupted_window() {
        let mut full = fixture();
        full.current_impulses.as_mut().unwrap()[0].points.insert(
            0,
            CurrentImpulsePoint {
                time: 1.0,
                charge_coulombs: 0.003,
            },
        );
        let expected = spectrum(&full, "I(X1.V1)", None).unwrap();
        let mut resumed = full.clone();
        resumed.time.drain(..128);
        resumed.branch_currents[0].drain(..128);
        // The physical event accepted at the checkpoint is not replayed.
        resumed.current_impulses.as_mut().unwrap()[0]
            .points
            .remove(0);
        assert_eq!(spectrum(&resumed, "I(X1.V1)", None).unwrap(), expected);
        close(expected.dc_component, 0.002);
    }

    #[test]
    fn fourier_current_impulse_rejects_alias_collision_bad_points_and_honors_abort() {
        let mut result = fixture();
        let mut duplicate = result.current_impulses.as_ref().unwrap()[0].clone();
        duplicate.owner = CurrentImpulseOwner::Branch {
            branch_name: "x1:v1".into(),
        };
        result.current_impulses.as_mut().unwrap().push(duplicate);
        assert!(
            spectrum(&result, "I(X1.V1)", None)
                .unwrap_err()
                .to_string()
                .contains("ambiguous")
        );
        result.current_impulses.as_mut().unwrap().pop();
        result.current_impulses.as_mut().unwrap()[0].points[0].charge_coulombs = Value::NAN;
        assert!(spectrum(&result, "I(X1.V1)", None).is_err());
        struct Cancel;
        impl AbortSignal for Cancel {
            fn is_aborted(&self) -> bool {
                true
            }
        }
        assert!(matches!(
            resolve(None, &result, "I(X1.V1)", (1.0, 2.0), &Cancel),
            Err(CurrentObservationError::Aborted)
        ));
    }

    #[test]
    fn fourier_current_impulse_preserves_subnormal_charge_and_extreme_time_scale() {
        let mut result = fixture();
        for time in &mut result.time {
            *time *= 1e-300;
        }
        result.current_impulses.as_mut().unwrap()[0].points[0] = CurrentImpulsePoint {
            time: 1.25e-300,
            charge_coulombs: Value::from_bits(1),
        };
        let output = FourierAnalysis::new(FourierConfig::new(1e300).with_harmonics(1))
            .analyze_transient_output_with_abort(
                None,
                &result,
                "I(X1.V1)",
                &result.branch_currents[0],
                &NoAbort,
            )
            .unwrap();
        let expected = Value::from_bits(1) / 1e-300;
        assert!((output.dc_component / expected - 1.0).abs() < 2e-14);
        assert!((output.harmonics[1].magnitude / (2.0 * expected) - 1.0).abs() < 2e-14);
    }

    #[test]
    fn fourier_current_impulse_authored_cards_use_shared_physical_transform() {
        let result = fixture();
        let netlist = Netlist::parse("current Fourier\nV1 out 0 0\nR1 out 0 1k\n.tran 1m 2\n.four 1 4 I(X1.V1) {2*I(X1.V1)}\n.end\n").unwrap();
        let spectra = crate::engine::evaluate_transient_fourier_results(
            &netlist,
            &result,
            ResourceLimits::default(),
            &NoAbort,
        )
        .unwrap();
        assert_eq!(spectra.len(), 2);
        close(spectra[0].spectrum.dc_component, 0.002);
        close(spectra[1].spectrum.dc_component, 0.004);
    }
}

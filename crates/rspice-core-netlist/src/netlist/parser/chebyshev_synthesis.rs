//! Exact PSpice Type-I CHEBYSHEV controlled-source synthesis.
//!
//! PSpice specifies these filters by pass-band/stop-band edge frequencies,
//! maximum pass-band ripple, and minimum stop-band attenuation.  The design
//! below computes the minimum-order analog Type-I Chebyshev prototype, applies
//! the standard low-pass, high-pass, band-pass, or band-reject frequency
//! transformation algebraically, and lowers the resulting real rational
//! transfer function through the same state-space realization used by
//! authored LAPLACE sources.  No response sampling or coefficient fitting is
//! involved.

use num_complex::Complex64;

use super::{Element, ParseError, PspiceChebyshevKind, synthesize_rational_transfer};
use crate::Value;

const MAX_REALIZED_ORDER: usize = 32;

impl PspiceChebyshevKind {
    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::LowPass => "LP",
            Self::HighPass => "HP",
            Self::BandPass => "BP",
            Self::BandReject => "BR",
        }
    }

    pub(super) const fn frequency_count(self) -> usize {
        match self {
            Self::LowPass | Self::HighPass => 2,
            Self::BandPass | Self::BandReject => 4,
        }
    }
}

/// Fully evaluated PSpice CHEBYSHEV source specification.
#[derive(Debug, Clone, PartialEq)]
pub(in crate::netlist) struct ChebyshevSpec {
    pub(in crate::netlist) kind: PspiceChebyshevKind,
    pub(in crate::netlist) frequencies_hz: Vec<Value>,
    pub(in crate::netlist) ripple_db: Value,
    pub(in crate::netlist) stop_db: Value,
}

type Poly = Vec<Value>;

fn poly_trim(mut polynomial: Poly) -> Poly {
    while polynomial.len() > 1 && polynomial.last() == Some(&0.0) {
        polynomial.pop();
    }
    polynomial
}

fn poly_mul(left: &[Value], right: &[Value]) -> Poly {
    let mut product = vec![0.0; left.len() + right.len() - 1];
    for (left_power, left_coefficient) in left.iter().enumerate() {
        for (right_power, right_coefficient) in right.iter().enumerate() {
            product[left_power + right_power] += left_coefficient * right_coefficient;
        }
    }
    poly_trim(product)
}

fn poly_pow(base: &[Value], exponent: usize) -> Poly {
    let mut result = vec![1.0];
    for _ in 0..exponent {
        result = poly_mul(&result, base);
    }
    result
}

fn poly_add_scaled(target: &mut Poly, term: &[Value], scale: Value) {
    if target.len() < term.len() {
        target.resize(term.len(), 0.0);
    }
    for (coefficient, term_coefficient) in target.iter_mut().zip(term) {
        *coefficient += scale * term_coefficient;
    }
}

fn monomial(coefficient: Value, power: usize) -> Poly {
    let mut result = vec![0.0; power + 1];
    result[power] = coefficient;
    result
}

fn minimum_order(
    ripple_db: Value,
    stop_db: Value,
    normalized_stop: Value,
) -> Result<(usize, Value), String> {
    if !(ripple_db.is_finite() && ripple_db > 0.0) {
        return Err("pass-band ripple must be finite and greater than 0 dB".to_string());
    }
    if !(stop_db.is_finite() && stop_db > ripple_db) {
        return Err(
            "stop-band attenuation must be finite and greater than pass-band ripple".to_string(),
        );
    }
    if !(normalized_stop.is_finite() && normalized_stop > 1.0) {
        return Err("stop-band edges must map strictly beyond the pass band".to_string());
    }

    let epsilon = (10.0f64.powf(ripple_db / 10.0) - 1.0).sqrt();
    let stop_factor = (10.0f64.powf(stop_db / 10.0) - 1.0).sqrt();
    let raw_order = (stop_factor / epsilon).acosh() / normalized_stop.acosh();
    if !(raw_order.is_finite() && raw_order > 0.0) {
        return Err("filter order is not finite for the supplied requirements".to_string());
    }
    let order = raw_order.ceil() as usize;
    Ok((order.max(1), epsilon))
}

/// Return the normalized Type-I prototype gain, expanded denominator, and
/// real first/second-order factors in ascending powers of normalized complex
/// frequency.  Keeping the factors lets the realized source use a
/// numerically conditioned cascade without changing the exact transfer.
fn normalized_prototype(order: usize, epsilon: Value) -> Result<(Value, Poly, Vec<Poly>), String> {
    let mu = (1.0 / epsilon).asinh() / order as Value;
    let mut factors = Vec::with_capacity(order.div_ceil(2));

    for pole_index in 1..=order {
        let theta = std::f64::consts::PI * (2 * pole_index - 1) as Value / (2 * order) as Value;
        let pole = Complex64::new(-mu.sinh() * theta.sin(), mu.cosh() * theta.cos());
        if !order.is_multiple_of(2) && pole_index == order.div_ceil(2) {
            factors.push(vec![-pole.re, 1.0]);
        } else if pole.im > 0.0 {
            factors.push(vec![pole.norm_sqr(), -2.0 * pole.re, 1.0]);
        }
    }

    if factors.iter().map(|factor| factor.len() - 1).sum::<usize>() != order {
        return Err("prototype pole pairing did not preserve filter order".to_string());
    }
    let denominator = factors
        .iter()
        .fold(vec![1.0], |expanded, factor| poly_mul(&expanded, factor));
    let dc_denominator = denominator[0];
    let gain = if order.is_multiple_of(2) {
        dc_denominator / (1.0 + epsilon * epsilon).sqrt()
    } else {
        dc_denominator
    };
    Ok((gain, denominator, factors))
}

fn finite_positive_frequencies(frequencies: &[Value]) -> Result<(), String> {
    if frequencies
        .iter()
        .any(|frequency| !(frequency.is_finite() && *frequency > 0.0))
    {
        return Err("all cutoff frequencies must be finite and greater than zero".to_string());
    }
    Ok(())
}

fn low_or_high_pass_sections(
    kind: PspiceChebyshevKind,
    frequencies_hz: &[Value],
    ripple_db: Value,
    stop_db: Value,
) -> Result<Vec<(Poly, Poly)>, String> {
    finite_positive_frequencies(frequencies_hz)?;
    let (pass_hz, normalized_stop) = match kind {
        PspiceChebyshevKind::LowPass => {
            let [pass_hz, stop_hz] = frequencies_hz else {
                return Err("LP requires exactly two cutoff frequencies".to_string());
            };
            if stop_hz <= pass_hz {
                return Err("LP requires pass-band frequency < stop-band frequency".to_string());
            }
            (*pass_hz, stop_hz / pass_hz)
        }
        PspiceChebyshevKind::HighPass => {
            let [pass_hz, stop_hz] = frequencies_hz else {
                return Err("HP requires exactly two cutoff frequencies".to_string());
            };
            if stop_hz >= pass_hz {
                return Err("HP requires pass-band frequency > stop-band frequency".to_string());
            }
            (*pass_hz, pass_hz / stop_hz)
        }
        _ => unreachable!("low_or_high_pass only receives LP or HP"),
    };
    let (order, epsilon) = minimum_order(ripple_db, stop_db, normalized_stop)?;
    if order > MAX_REALIZED_ORDER {
        return Err(format!(
            "{} requirements need order {order}, exceeding the bounded order {MAX_REALIZED_ORDER}",
            kind.label()
        ));
    }
    let (gain, prototype_denominator, prototype_factors) = normalized_prototype(order, epsilon)?;
    let pass_radians = 2.0 * std::f64::consts::PI * pass_hz;
    let overall_gain = gain / prototype_denominator[0];

    let mut sections = Vec::with_capacity(prototype_factors.len());
    for factor in prototype_factors {
        let section_order = factor.len() - 1;
        let (numerator, denominator) = match kind {
            PspiceChebyshevKind::LowPass => {
                let denominator = factor
                    .iter()
                    .enumerate()
                    .map(|(power, coefficient)| {
                        coefficient * pass_radians.powi((section_order - power) as i32)
                    })
                    .collect::<Vec<_>>();
                (vec![denominator[0]], denominator)
            }
            PspiceChebyshevKind::HighPass => {
                let mut denominator = vec![0.0; section_order + 1];
                for (power, coefficient) in factor.iter().enumerate() {
                    denominator[section_order - power] =
                        coefficient * pass_radians.powi(power as i32);
                }
                (monomial(factor[0], section_order), denominator)
            }
            _ => unreachable!("low_or_high_pass only receives LP or HP"),
        };
        sections.push((numerator, denominator));
    }
    for coefficient in &mut sections[0].0 {
        *coefficient *= overall_gain;
    }
    Ok(sections)
}

fn band_filter_sections(
    kind: PspiceChebyshevKind,
    frequencies_hz: &[Value],
    ripple_db: Value,
    stop_db: Value,
) -> Result<Vec<(Poly, Poly)>, String> {
    finite_positive_frequencies(frequencies_hz)?;
    let [first, second, third, fourth] = frequencies_hz else {
        return Err(format!(
            "{} requires exactly four cutoff frequencies",
            kind.label()
        ));
    };

    let (pass_low_hz, pass_high_hz, stop_low_hz, stop_high_hz) = match kind {
        PspiceChebyshevKind::BandPass => {
            if !(first < second && second < third && third < fourth) {
                return Err("BP requires stop-low < pass-low < pass-high < stop-high".to_string());
            }
            (*second, *third, *first, *fourth)
        }
        PspiceChebyshevKind::BandReject => {
            if !(first < second && second < fourth && fourth < third) {
                return Err(
                    "BR requires pass-low < stop-low < stop-high < pass-high; its source order is pass-low, stop-low, pass-high, stop-high"
                        .to_string(),
                );
            }
            (*first, *third, *second, *fourth)
        }
        _ => unreachable!("band_filter only receives BP or BR"),
    };

    let pass_low = 2.0 * std::f64::consts::PI * pass_low_hz;
    let pass_high = 2.0 * std::f64::consts::PI * pass_high_hz;
    let stop_low = 2.0 * std::f64::consts::PI * stop_low_hz;
    let stop_high = 2.0 * std::f64::consts::PI * stop_high_hz;
    let center_squared = pass_low * pass_high;
    let bandwidth = pass_high - pass_low;

    let normalized_stop = match kind {
        PspiceChebyshevKind::BandPass => [stop_low, stop_high]
            .into_iter()
            .map(|frequency| {
                ((frequency * frequency - center_squared) / (bandwidth * frequency)).abs()
            })
            .fold(Value::INFINITY, Value::min),
        PspiceChebyshevKind::BandReject => [stop_low, stop_high]
            .into_iter()
            .map(|frequency| {
                ((bandwidth * frequency) / (center_squared - frequency * frequency)).abs()
            })
            .fold(Value::INFINITY, Value::min),
        _ => unreachable!("band_filter only receives BP or BR"),
    };

    let (prototype_order, epsilon) = minimum_order(ripple_db, stop_db, normalized_stop)?;
    let realized_order = 2 * prototype_order;
    if realized_order > MAX_REALIZED_ORDER {
        return Err(format!(
            "{} requirements need realized order {realized_order}, exceeding the bounded order {MAX_REALIZED_ORDER}",
            kind.label()
        ));
    }
    let (gain, prototype_denominator, prototype_factors) =
        normalized_prototype(prototype_order, epsilon)?;
    let quadratic = vec![center_squared, 0.0, 1.0];
    let linear = vec![0.0, bandwidth];
    let overall_gain = gain / prototype_denominator[0];

    let mut sections = Vec::with_capacity(prototype_factors.len());
    for factor in prototype_factors {
        let section_order = factor.len() - 1;
        let mut denominator = vec![0.0];
        for (power, coefficient) in factor.iter().enumerate() {
            let term = match kind {
                PspiceChebyshevKind::BandPass => poly_mul(
                    &poly_pow(&quadratic, power),
                    &poly_pow(&linear, section_order - power),
                ),
                PspiceChebyshevKind::BandReject => poly_mul(
                    &poly_pow(&linear, power),
                    &poly_pow(&quadratic, section_order - power),
                ),
                _ => unreachable!("band_filter only receives BP or BR"),
            };
            poly_add_scaled(&mut denominator, &term, *coefficient);
        }
        let numerator_base = match kind {
            PspiceChebyshevKind::BandPass => poly_pow(&linear, section_order),
            PspiceChebyshevKind::BandReject => poly_pow(&quadratic, section_order),
            _ => unreachable!("band_filter only receives BP or BR"),
        };
        let numerator: Poly = numerator_base
            .into_iter()
            .map(|coefficient| factor[0] * coefficient)
            .collect();
        sections.push((numerator, poly_trim(denominator)));
    }
    for coefficient in &mut sections[0].0 {
        *coefficient *= overall_gain;
    }
    Ok(sections)
}

fn design_sections(spec: &ChebyshevSpec) -> Result<Vec<(Poly, Poly)>, String> {
    if spec.frequencies_hz.len() != spec.kind.frequency_count() {
        return Err(format!(
            "{} requires exactly {} cutoff frequencies",
            spec.kind.label(),
            spec.kind.frequency_count()
        ));
    }
    match spec.kind {
        PspiceChebyshevKind::LowPass | PspiceChebyshevKind::HighPass => low_or_high_pass_sections(
            spec.kind,
            &spec.frequencies_hz,
            spec.ripple_db,
            spec.stop_db,
        ),
        PspiceChebyshevKind::BandPass | PspiceChebyshevKind::BandReject => band_filter_sections(
            spec.kind,
            &spec.frequencies_hz,
            spec.ripple_db,
            spec.stop_db,
        ),
    }
}

fn design_transfer(spec: &ChebyshevSpec) -> Result<(Poly, Poly), String> {
    let sections = design_sections(spec)?;
    Ok(sections.into_iter().fold(
        (vec![1.0], vec![1.0]),
        |(numerator, denominator), (section_numerator, section_denominator)| {
            (
                poly_mul(&numerator, &section_numerator),
                poly_mul(&denominator, &section_denominator),
            )
        },
    ))
}

/// Compile one PSpice CHEBYSHEV E/G source into exact dynamic helper elements.
pub(in crate::netlist) fn synthesize_chebyshev(
    name: &str,
    node_pos: &str,
    node_neg: &str,
    input_expr: &str,
    spec: &ChebyshevSpec,
    voltage_output: bool,
    line_num: usize,
) -> Result<Vec<Element>, ParseError> {
    // A high-pass cascade contains direct feedthrough in every section.  The
    // engine's behavioral-source Newton path is better conditioned when that
    // proper transfer is lowered as one controller form; the other families
    // use the low-order cascade to avoid high-order band polynomial scaling.
    let sections = (if spec.kind == PspiceChebyshevKind::HighPass {
        design_transfer(spec).map(|transfer| vec![transfer])
    } else {
        design_sections(spec)
    })
    .map_err(|message| ParseError::Syntax {
        line: line_num,
        message: format!("CHEBYSHEV source '{name}': {message}"),
    })?;
    let section_count = sections.len();
    let mut elements = Vec::new();
    let mut section_input = input_expr.to_string();
    for (index, (numerator, denominator)) in sections.into_iter().enumerate() {
        let representable = !numerator.is_empty()
            && !denominator.is_empty()
            && numerator.iter().any(|coefficient| *coefficient != 0.0)
            && denominator
                .first()
                .is_some_and(|coefficient| *coefficient != 0.0)
            && denominator
                .last()
                .is_some_and(|coefficient| *coefficient != 0.0)
            && numerator
                .iter()
                .chain(&denominator)
                .all(|coefficient| coefficient.is_finite());
        if !representable {
            return Err(ParseError::Syntax {
                line: line_num,
                message: format!(
                    "CHEBYSHEV source '{name}': cutoff range cannot be represented without degenerate or non-finite transfer coefficients"
                ),
            });
        }
        let last = index + 1 == section_count;
        let section_name = if last {
            name.to_string()
        } else {
            format!("{name}.__F{}", index + 1)
        };
        let intermediate_node = format!("{name}.__Y{}", index + 1);
        let section_node_pos = if last { node_pos } else { &intermediate_node };
        let section_node_neg = if last { node_neg } else { "0" };
        let mut section_elements = synthesize_rational_transfer(
            &section_name,
            section_node_pos,
            section_node_neg,
            &section_input,
            numerator,
            denominator,
            if last { voltage_output } else { true },
            crate::netlist::SynthesizedTransferForm::Chebyshev,
            line_num,
        )?;
        if !last {
            let Some(section_output) = section_elements
                .iter_mut()
                .find(|element| element.name.eq_ignore_ascii_case(&section_name))
            else {
                return Err(ParseError::Syntax {
                    line: line_num,
                    message: format!(
                        "CHEBYSHEV source '{name}': dynamic section did not produce its required output element"
                    ),
                });
            };
            section_output.provenance =
                crate::netlist::ElementProvenance::GeneratedDynamicInternalNode {
                    owner: section_name.clone(),
                    form: crate::netlist::SynthesizedTransferForm::Chebyshev,
                    node: intermediate_node.clone(),
                };
            section_input = format!("V({intermediate_node})");
        }
        elements.extend(section_elements);
    }
    Ok(elements)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn magnitude(numerator: &[Value], denominator: &[Value], frequency_hz: Value) -> Value {
        let s = Complex64::new(0.0, 2.0 * std::f64::consts::PI * frequency_hz);
        let evaluate = |coefficients: &[Value]| {
            coefficients
                .iter()
                .rev()
                .fold(Complex64::new(0.0, 0.0), |value, coefficient| {
                    value * s + coefficient
                })
        };
        (evaluate(numerator) / evaluate(denominator)).norm()
    }

    fn spec(kind: PspiceChebyshevKind, frequencies_hz: &[Value]) -> ChebyshevSpec {
        ChebyshevSpec {
            kind,
            frequencies_hz: frequencies_hz.to_vec(),
            ripple_db: 0.1,
            stop_db: 50.0,
        }
    }

    fn assert_edges(spec: &ChebyshevSpec, pass_edges: &[Value], stop_edges: &[Value]) {
        let (numerator, denominator) = design_transfer(spec).expect("filter designs");
        let pass_floor = 10.0f64.powf(-spec.ripple_db / 20.0);
        let stop_ceiling = 10.0f64.powf(-spec.stop_db / 20.0);
        for edge in pass_edges {
            let response = magnitude(&numerator, &denominator, *edge);
            assert!(
                response >= pass_floor * (1.0 - 1.0e-8) && response <= 1.0 + 1.0e-8,
                "pass edge {edge} Hz response {response} is outside [{pass_floor}, 1]"
            );
        }
        for edge in stop_edges {
            let response = magnitude(&numerator, &denominator, *edge);
            assert!(
                response <= stop_ceiling * (1.0 + 1.0e-7),
                "stop edge {edge} Hz response {response} exceeds {stop_ceiling}"
            );
        }
    }

    #[test]
    fn official_low_and_high_pass_examples_meet_both_edges() {
        assert_edges(
            &spec(PspiceChebyshevKind::LowPass, &[800.0, 1_200.0]),
            &[800.0],
            &[1_200.0],
        );
        assert_edges(
            &spec(PspiceChebyshevKind::HighPass, &[1_200.0, 800.0]),
            &[1_200.0],
            &[800.0],
        );
    }

    #[test]
    fn official_band_pass_and_reject_examples_meet_all_edges() {
        assert_edges(
            &spec(
                PspiceChebyshevKind::BandPass,
                &[800.0, 1_200.0, 2_000.0, 3_000.0],
            ),
            &[1_200.0, 2_000.0],
            &[800.0, 3_000.0],
        );
        assert_edges(
            &spec(
                PspiceChebyshevKind::BandReject,
                &[800.0, 1_200.0, 3_000.0, 2_000.0],
            ),
            &[800.0, 3_000.0],
            &[1_200.0, 2_000.0],
        );
    }

    #[test]
    fn invalid_edges_and_unbounded_orders_fail_closed() {
        let invalid = spec(PspiceChebyshevKind::LowPass, &[1_200.0, 800.0]);
        assert!(
            design_transfer(&invalid)
                .unwrap_err()
                .contains("LP requires")
        );

        let excessive = ChebyshevSpec {
            kind: PspiceChebyshevKind::LowPass,
            frequencies_hz: vec![1_000.0, 1_000.001],
            ripple_db: 0.001,
            stop_db: 200.0,
        };
        assert!(
            design_transfer(&excessive)
                .unwrap_err()
                .contains("bounded order")
        );
    }
}

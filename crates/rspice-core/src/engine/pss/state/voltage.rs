//! Sparse voltage relations for differential and cascaded ideal amplifiers.

use super::*;
use std::collections::BTreeMap;

use num_bigint::{BigInt, Sign};

mod descriptor;
pub(super) use descriptor::PssDescriptor;

const FORM_TERM_WORDS: usize = std::mem::size_of::<(ForestValue, Value)>().div_ceil(8);

type VoltageRow = crate::engine::exact_constraints::ExactRow<ForestValue>;
pub(super) type PssVoltageConstraintBuilder =
    crate::engine::exact_constraints::ExactElimination<ForestValue>;

#[derive(Debug, Clone)]
pub(super) struct PssVoltageConstraints {
    node_forms: std::sync::Arc<Vec<Vec<(ForestValue, Value)>>>,
    max_values: usize,
    retained_words: usize,
}

#[derive(Debug, Clone)]
pub(super) struct InitialChargeRates {
    pub branches: Vec<usize>,
    node_forms: std::sync::Arc<Vec<Vec<(ForestValue, Value)>>>,
    pub positions: Vec<(usize, usize)>,
    max_values: usize,
    retained_words: usize,
}

fn precision_error() -> SimulationError {
    SimulationError::Circuit(
        "PSS voltage-constraint projection exceeds finite precision".to_owned(),
    )
}

/// Every finite binary64 number is an integer multiple of 2^-1074. Common
/// factors are removed from each row before it enters elimination.
fn integer_coefficient(value: Value) -> Result<BigInt, SimulationError> {
    crate::engine::exact_constraints::integer_coefficient(value).ok_or_else(precision_error)
}

/// Correctly round a rational coefficient, including subnormal ties. A
/// nonzero coefficient that disappears cannot silently become a missing edge.
fn coefficient_ratio(numerator: &BigInt, denominator: &BigInt) -> Result<Value, SimulationError> {
    crate::engine::exact_constraints::coefficient_ratio(numerator, denominator)
        .ok_or_else(precision_error)
}

impl VoltageRow {
    fn form(self) -> Result<Vec<(ForestValue, Value)>, SimulationError> {
        let mut form = Vec::with_capacity(self.values.len());
        for (key, value) in self.values {
            form.push((key, -coefficient_ratio(&value, &self.query)?));
        }
        Ok(form)
    }
}

impl PssVoltageConstraintBuilder {
    /// Compiled mappings remain alive during subsequent elimination queries.
    /// Account for their capacity before converting the next integer row.
    fn compile_row(
        &mut self,
        row: VoltageRow,
    ) -> Result<Vec<(ForestValue, Value)>, SimulationError> {
        let words = row.values.len().saturating_mul(FORM_TERM_WORDS);
        self.check_cost(words.saturating_add(row.words().saturating_mul(3)))?;
        let form = row.form()?;
        self.reserve_retained_words(words)?;
        Ok(form)
    }

    pub(super) fn add(
        &mut self,
        nodes: impl IntoIterator<Item = (usize, Value)>,
        value: ForestValue,
        abort: &dyn AbortSignal,
    ) -> Result<bool, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let mut row = VoltageRow::default();
        for (node, coefficient) in nodes {
            if node != 0 {
                Self::add_integer(&mut row.nodes, node, integer_coefficient(coefficient)?);
            }
        }
        if value != ForestValue::Zero {
            row.values.insert(value, integer_coefficient(1.0)?);
        }
        Ok(self.admit(row, 1, abort)?.is_none())
    }

    pub(super) fn port(
        &self,
        pos: usize,
        neg: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(ForestValue, Value)>, SimulationError> {
        let row = self.port_row(pos, neg, abort)?;
        if !row.nodes.is_empty() {
            return Err(SimulationError::Circuit(
                "PSS charge-port voltage is outside its independent state basis".to_owned(),
            ));
        }
        row.form()
    }

    fn port_row(
        &self,
        pos: usize,
        neg: usize,
        abort: &dyn AbortSignal,
    ) -> Result<VoltageRow, SimulationError> {
        let mut row = VoltageRow {
            query: BigInt::from(1_u32),
            ..VoltageRow::default()
        };
        for (node, weight) in [(pos, 1), (neg, -1)] {
            if node != 0 {
                Self::add_integer(&mut row.nodes, node, BigInt::from(weight));
            }
        }
        self.reduce(&mut row, abort)?;
        Ok(row)
    }
}

impl PssVoltageConstraintBuilder {
    pub(super) fn finish(
        mut self,
        circuit: &CircuitData,
        abort: &dyn AbortSignal,
    ) -> Result<PssVoltageConstraints, SimulationError> {
        // Every charge-port voltage must be fixed, even if the electrical
        // component's common-mode voltage is still an algebraic unknown.
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit.capacitors.capacitances[index] != 0.0 {
                self.port(stamp.pp.row, stamp.nn.row, abort)?;
            }
        }
        for diode in &circuit.diodes.devices {
            if diode.has_charge_storage() {
                self.port(diode.node_anode, diode.node_cathode, abort)?;
            }
        }
        for bjt in &circuit.bjts.devices {
            for (pos, neg) in bjt.charge_storage_nodes().into_iter().flatten() {
                self.port(pos, neg, abort)?;
            }
        }
        for jfet in &circuit.jfets {
            for (pos, neg) in jfet.classic_charge_storage_nodes().into_iter().flatten() {
                self.port(pos, neg, abort)?;
            }
        }
        for mos in &circuit.mosfets.devices {
            for (pos, neg) in mos.shooting_terminal_storage_nodes().into_iter().flatten() {
                self.port(pos, neg, abort)?;
            }
        }
        for mos in &circuit.bsim3v3.devices {
            for (pos, neg) in mos.shooting_terminal_storage_nodes().into_iter().flatten() {
                self.port(pos, neg, abort)?;
            }
        }
        for mos in &circuit.bsim4v8.devices {
            for (pos, neg) in mos.shooting_terminal_storage_nodes().into_iter().flatten() {
                self.port(pos, neg, abort)?;
            }
        }
        let mut words = circuit.num_nodes().saturating_mul(3);
        self.reserve_retained_words(words)?;
        let mut node_forms = Vec::with_capacity(circuit.num_nodes());
        for node in 1..=circuit.num_nodes() {
            let row = self.port_row(node, 0, abort)?;
            words = words.saturating_add(row.values.len().saturating_mul(FORM_TERM_WORDS));
            node_forms.push(self.compile_row(row)?);
        }
        Ok(PssVoltageConstraints {
            node_forms: std::sync::Arc::new(node_forms),
            max_values: self.limits.max_result_values,
            retained_words: words,
        })
    }
}

impl PssVoltageConstraints {
    fn ensure_evaluation_work(
        &self,
        extra_words: usize,
        terms: usize,
    ) -> Result<(), SimulationError> {
        PssVoltageConstraintBuilder::ensure_words(
            self.retained_words
                .saturating_add(extra_words)
                .saturating_add(terms.saturating_mul(2)),
            self.max_values,
        )
    }

    fn max_terms(&self) -> usize {
        self.node_forms.iter().map(Vec::len).max().unwrap_or(0)
    }

    pub(super) fn solve(
        &self,
        solution: &mut [Value],
        mut value: impl FnMut(ForestValue) -> Result<Value, SimulationError>,
    ) -> Result<(), SimulationError> {
        let max_terms = self.max_terms();
        self.ensure_evaluation_work(solution.len(), max_terms)?;
        let mut terms = Vec::with_capacity(max_terms);
        for (node, form) in self.node_forms.iter().enumerate() {
            solution[node + 1] = evaluate_form(form, &mut terms, &mut value)?;
        }
        Ok(())
    }
}

fn evaluate_form(
    form: &[(ForestValue, Value)],
    terms: &mut Vec<(Value, Value)>,
    mut value: impl FnMut(ForestValue) -> Result<Value, SimulationError>,
) -> Result<Value, SimulationError> {
    terms.clear();
    for &(source, weight) in form {
        terms.push((weight, value(source)?));
    }
    let result = rspice_veriloga_runtime::arithmetic::sum_products(terms.iter().copied())
        .map_err(|_| precision_error())?;
    if result.is_finite() {
        Ok(result)
    } else {
        Err(precision_error())
    }
}

impl InitialChargeRates {
    pub(super) fn new(constraints: &PssVoltageConstraints, branches: Vec<usize>) -> Self {
        Self {
            retained_words: constraints.retained_words.saturating_add(branches.len()),
            branches,
            node_forms: std::sync::Arc::clone(&constraints.node_forms),
            positions: Vec::new(),
            max_values: constraints.max_values,
        }
    }

    pub(super) fn link_pattern(
        &mut self,
        matrix: &StaticMatrix,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        let mut positions = Vec::new();
        for position in matrix.stored_positions() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            self.ensure_work(positions.len().saturating_add(1).saturating_mul(2))?;
            positions.push(position);
        }
        self.positions = positions;
        Ok(())
    }

    pub(super) fn extra_pattern(
        &self,
        matrix: &StaticMatrix,
        nodes: usize,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<(usize, usize)>, SimulationError> {
        let mut pattern = std::collections::BTreeSet::new();
        for (row, col) in matrix.stored_positions() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if let Some(form) = self.node_forms.get(col) {
                for &(value, _) in form {
                    if let ForestValue::State(index) = value {
                        self.ensure_work(pattern.len().saturating_add(1).saturating_mul(12))?;
                        pattern.insert((row, nodes + self.branches[index] - 1));
                    }
                }
            }
        }
        Ok(pattern.into_iter().collect())
    }

    fn ensure_work(&self, words: usize) -> Result<(), SimulationError> {
        PssVoltageConstraintBuilder::ensure_words(
            self.retained_words
                .saturating_add(self.positions.len().saturating_mul(2))
                .saturating_add(words),
            self.max_values,
        )
    }

    /// Map dQ/dv into independent voltage rates. The companion RHS is never
    /// used: this is an instantaneous derivative, with no fictitious timestep.
    pub(super) fn project(
        &self,
        values: &[Value],
        circuit: &CircuitData,
        rhs: &mut [Value],
        stamps: &mut Vec<(usize, usize, Value)>,
    ) -> Result<(), SimulationError> {
        let nodes = circuit.num_nodes();
        let mut projected: BTreeMap<_, Vec<(Value, Value)>> = BTreeMap::new();
        let mut term_count = 0_usize;
        for (&(row, col), &coefficient) in self.positions.iter().zip(values) {
            if coefficient == 0.0 {
                continue;
            }
            let Some(form) = self.node_forms.get(col) else {
                return Err(SimulationError::Circuit(
                    "PSS charge Jacobian contains an unsupported current derivative".to_owned(),
                ));
            };
            for &(value, weight) in form {
                term_count = term_count.saturating_add(1);
                self.ensure_work(term_count.saturating_mul(24))?;
                projected
                    .entry((row, value))
                    .or_default()
                    .push((coefficient, weight));
            }
        }
        let mut source_rates = BTreeMap::new();
        for ((row, value), terms) in projected {
            let weight = rspice_veriloga_runtime::arithmetic::sum_products(terms.iter().copied())
                .map_err(|_| precision_error())?;
            if weight == 0.0 {
                if !rspice_veriloga_runtime::arithmetic::sum_products_is_zero(terms.iter().copied())
                    .map_err(|_| precision_error())?
                {
                    return Err(precision_error());
                }
                continue;
            }
            if let ForestValue::State(index) = value {
                stamps.push((row, nodes + self.branches[index] - 1, weight));
            } else {
                let rate = *source_rates.entry(value).or_insert_with(|| match value {
                    ForestValue::Source(index) => {
                        circuit.voltage_sources.right_derivative_at_time(index, 0.0)
                    }
                    ForestValue::BehavioralSource(index) => {
                        circuit.behavioral_sources.voltage_sources[index]
                            .explicit_time_derivative(0.0)
                            .unwrap_or(Value::NAN)
                    }
                    ForestValue::Zero | ForestValue::State(_) => 0.0,
                    ForestValue::CurrentState(_) | ForestValue::SourceDerivative { .. } => {
                        Value::NAN
                    }
                });
                if !rate.is_finite() {
                    let name = match value {
                        ForestValue::Source(index) => &circuit.voltage_sources.names[index],
                        ForestValue::BehavioralSource(index) => {
                            &circuit.behavioral_sources.voltage_sources[index].name
                        }
                        ForestValue::Zero
                        | ForestValue::State(_)
                        | ForestValue::CurrentState(_)
                        | ForestValue::SourceDerivative { .. } => return Err(precision_error()),
                    };
                    return Err(SimulationError::Circuit(format!(
                        "PSS initial displacement current requires a finite analytic outgoing derivative for source {name} at t=0"
                    )));
                }
                rhs[row] = rspice_veriloga_runtime::arithmetic::sum_products(
                    [(rhs[row], 1.0), (-weight, rate)].into_iter(),
                )
                .map_err(|_| precision_error())?;
                if !rhs[row].is_finite() {
                    return Err(precision_error());
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::{CountingAbort, NoAbort};
    use crate::resource::ResourceLimits;

    #[test]
    fn projection_workspace_keeps_prior_mappings_in_the_compilation_budget() {
        let mut deck = String::from("Stacked source projections\n");
        for index in 1..=64 {
            let negative = if index == 1 {
                "0".to_owned()
            } else {
                format!("n{}", index - 1)
            };
            deck.push_str(&format!("V{index} n{index} {negative} 1\n"));
        }
        deck.push_str(".end\n");
        let circuit = Engine::default()
            .build_circuit(&Netlist::parse(&deck).unwrap())
            .unwrap();
        let mut builder =
            PssVoltageConstraintBuilder::new(circuit.num_nodes() + 1, ResourceLimits::default())
                .unwrap();
        for (index, (&positive, &negative)) in circuit
            .voltage_sources
            .node_pos
            .iter()
            .zip(&circuit.voltage_sources.node_neg)
            .enumerate()
        {
            builder
                .add(
                    [(positive, 1.0), (negative, -1.0)],
                    ForestValue::Source(index),
                    &NoAbort,
                )
                .unwrap();
        }
        let mappings = builder.clone().finish(&circuit, &NoAbort).unwrap();
        // This budget can hold both retained representations, but leaves no
        // room for the query/conversion work while the last mapping is built.
        builder.limits.max_result_values = builder.retained_words + mappings.retained_words;
        assert!(matches!(
            builder.finish(&circuit, &NoAbort),
            Err(SimulationError::ResourceLimit(_))
        ));
    }

    #[test]
    fn projection_workspace_is_checked_before_evaluating_sources() {
        let mut mappings = PssVoltageConstraints {
            node_forms: std::sync::Arc::new(vec![
                vec![(ForestValue::Source(0), 2.0)],
                vec![
                    (ForestValue::Source(1), 1e300),
                    (ForestValue::Source(2), -1e300),
                    (ForestValue::Source(0), 1.0),
                ],
            ]),
            retained_words: 4 * FORM_TERM_WORDS + 6,
            max_values: 4 * FORM_TERM_WORDS + 7,
        };
        let mut solution = [0.0, 42.0, 43.0];
        let mut calls = 0;
        assert!(matches!(
            mappings.solve(&mut solution, |_| {
                calls += 1;
                Ok(3.0)
            }),
            Err(SimulationError::ResourceLimit(_))
        ));
        assert_eq!(calls, 0);
        assert_eq!(solution, [0.0, 42.0, 43.0]);
        mappings.max_values = usize::MAX;
        mappings
            .solve(&mut solution, |source| {
                calls += 1;
                Ok(if source == ForestValue::Source(0) {
                    3.0
                } else {
                    1.0
                })
            })
            .unwrap();
        assert_eq!(solution, [0.0, 6.0, 3.0]);
        assert_eq!(calls, 4, "exact summation must not re-evaluate a source");
    }

    #[test]
    fn vcvs_rank_preserves_a_control_loop_below_binary64_product_precision() {
        let epsilon = Value::EPSILON;
        assert_eq!((1.0 + epsilon) * (1.0 - epsilon), 1.0);
        let mut constraints =
            PssVoltageConstraintBuilder::new(4, ResourceLimits::default()).unwrap();
        for row in [
            [(2, 1.0), (1, -(1.0 + epsilon))],
            [(3, 1.0), (2, -(1.0 - epsilon))],
            [(1, 1.0), (3, -1.0)],
        ] {
            assert!(constraints.add(row, ForestValue::Zero, &NoAbort).unwrap());
        }
        // The exact loop gain is 1-epsilon^2, so v1 is constrained to zero.
        assert!(
            !constraints
                .add([(1, 1.0)], ForestValue::State(0), &NoAbort)
                .unwrap()
        );
        assert!(constraints.port(1, 0, &NoAbort).unwrap().is_empty());
    }

    #[test]
    fn vcvs_rational_projection_rounds_ties_and_preserves_finite_extremes() {
        let one = BigInt::from(1_u32);
        for value in [
            Value::MAX,
            -Value::MAX,
            Value::MIN_POSITIVE,
            Value::from_bits(1),
            -0.125,
            1.0,
        ] {
            let numerator = integer_coefficient(value).unwrap();
            let denominator = &one << 1074;
            assert_eq!(coefficient_ratio(&numerator, &denominator).unwrap(), value);
        }
        let denominator = &one << 53;
        assert_eq!(
            coefficient_ratio(&(&denominator + 1), &denominator).unwrap(),
            1.0
        );
        assert_eq!(
            coefficient_ratio(&(&denominator + 3), &denominator).unwrap(),
            1.0 + 2.0 * Value::EPSILON
        );
        assert_eq!(
            coefficient_ratio(&BigInt::from(3), &(&one << 1075)).unwrap(),
            Value::from_bits(2)
        );
        assert!(coefficient_ratio(&one, &(&one << 1075)).is_err());
        assert!(coefficient_ratio(&(&one << 1024), &one).is_err());
    }

    #[test]
    fn vcvs_initial_charge_pattern_checks_storage_before_retaining_entries() {
        let constraints = PssVoltageConstraints {
            node_forms: std::sync::Arc::new(Vec::new()),
            max_values: 3,
            retained_words: 0,
        };
        let mut rates = InitialChargeRates::new(&constraints, Vec::new());
        let matrix = StaticMatrix::from_triplets(2, 2, &[(0, 0, 0.0), (1, 1, 0.0)]).unwrap();
        assert!(matches!(
            rates.link_pattern(&matrix, &NoAbort),
            Err(SimulationError::ResourceLimit(_))
        ));
        assert!(rates.positions.is_empty());
        assert!(matches!(
            rates.link_pattern(&matrix, &CountingAbort::new(1)),
            Err(SimulationError::Aborted)
        ));
        assert!(rates.positions.is_empty());
    }

    #[test]
    fn vcvs_exact_reduction_obeys_cancellation_and_storage_limits() {
        let mut constraints =
            PssVoltageConstraintBuilder::new(4, ResourceLimits::default()).unwrap();
        let abort = CountingAbort::new(1);
        assert!(matches!(
            constraints.add([(1, 1.0)], ForestValue::State(0), &abort),
            Err(SimulationError::Aborted)
        ));
        let limits = ResourceLimits {
            max_result_values: 1,
            ..ResourceLimits::default()
        };
        assert!(matches!(
            PssVoltageConstraintBuilder::new(4, limits),
            Err(SimulationError::ResourceLimit(_))
        ));
        let limits = ResourceLimits {
            max_result_values: 128,
            ..ResourceLimits::default()
        };
        let mut constraints = PssVoltageConstraintBuilder::new(4, limits).unwrap();
        assert!(matches!(
            constraints.add(
                [(1, 1.0), (2, Value::from_bits(1))],
                ForestValue::Zero,
                &NoAbort
            ),
            Err(SimulationError::ResourceLimit(_))
        ));
    }
}

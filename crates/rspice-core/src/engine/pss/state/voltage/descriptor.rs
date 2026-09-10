//! Constant MNA descriptor closure for coupled charge and flux constraints.

use super::*;

#[derive(Debug, Clone)]
pub(in crate::engine::pss::state) struct PssDescriptor {
    solution: PssVoltageConstraints,
    charge_forcing: std::sync::Arc<Vec<Vec<(ForestValue, Value)>>>,
}

impl PssDescriptor {
    pub(in crate::engine::pss::state) fn applies(circuit: &CircuitData) -> bool {
        use crate::engine::periodic_capability::PeriodicDeviceFamily as F;
        // Ordinary independent-source networks retain their linear-time
        // topological basis. Nonlinear/delayed devices need their own manifold
        // closure; a DC linearization is not their large-signal descriptor.
        (!circuit.vcvs.is_empty()
            || !circuit.vccs.is_empty()
            || !circuit.cccs.is_empty()
            || !circuit.ccvs.is_empty())
            && circuit
                .capacitors
                .value_expressions
                .iter()
                .all(Option::is_none)
            && F::ALL.into_iter().all(|family| {
                matches!(
                    family,
                    F::Resistor
                        | F::ResistorBranch
                        | F::Capacitor
                        | F::Inductor
                        | F::VoltageSource
                        | F::CurrentSource
                        | F::Vcvs
                        | F::Vccs
                        | F::Cccs
                        | F::Ccvs
                        | F::InductorCoupling
                        | F::CoupledInductorPair
                ) || family.instance_count(circuit) == 0
            })
    }

    pub(in crate::engine::pss::state) fn build(
        circuit: &CircuitData,
        limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<PssStateBasis, SimulationError> {
        let size = circuit.matrix_size();
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        // Each admitted linear device stamps at most eight static entries.
        // Include the triplet builder's initial capacity and geometric growth
        // before it allocates, alongside both reducers and pending row maps.
        let stamp_bound = crate::engine::periodic_capability::PeriodicDeviceFamily::ALL
            .into_iter()
            .fold(circuit.num_nodes(), |sum, family| {
                sum.saturating_add(family.instance_count(circuit).saturating_mul(8))
            });
        let triplet_words = stamp_bound.max(size.saturating_mul(6)).saturating_mul(6);
        PssVoltageConstraintBuilder::ensure_words(
            size.saturating_mul(48).saturating_add(triplet_words),
            limits.max_result_values,
        )?;
        let mut dynamic =
            PssVoltageConstraintBuilder::new(size.saturating_mul(2).saturating_add(1), limits)?;
        let mut algebraic = PssVoltageConstraintBuilder::new(size.saturating_add(1), limits)?;
        let overhead = size.saturating_mul(32);
        let mut words = overhead
            .saturating_add(dynamic.retained_words)
            .saturating_add(algebraic.retained_words)
            .saturating_add(size.saturating_mul(VoltageRow::default().words()))
            .saturating_add(triplet_words);
        PssVoltageConstraintBuilder::ensure_words(words, limits.max_result_values)?;
        let mut rows = vec![VoltageRow::default(); size];
        let mut dc = circuit.create_matrix();
        circuit.stamp_dc(&mut dc, &mut circuit.create_rhs());
        let mut add = |row: usize,
                       node: usize,
                       source: Option<ForestValue>,
                       value: Value|
         -> Result<(), SimulationError> {
            if value == 0.0 || (node == 0 && source.is_none()) {
                return Ok(());
            }
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            // A binary64 coefficient occupies at most 2098 bits before row
            // normalization. Include insertion and integer-sum temporaries.
            PssVoltageConstraintBuilder::ensure_words(
                words.saturating_add(160),
                limits.max_result_values,
            )?;
            words = words.saturating_sub(rows[row].words());
            let coefficient = integer_coefficient(value)?;
            if let Some(source) = source {
                PssVoltageConstraintBuilder::add_integer(
                    &mut rows[row].values,
                    source,
                    coefficient,
                );
            } else {
                PssVoltageConstraintBuilder::add_integer(&mut rows[row].nodes, node, coefficient);
            }
            words = words.saturating_add(rows[row].words());
            Ok(())
        };
        let nodes = circuit.num_nodes();
        let ic_rows: std::collections::BTreeSet<_> = circuit
            .capacitors
            .ic_branch_indices
            .iter()
            .flatten()
            .map(|branch| nodes + branch - 1)
            .collect();
        for (row, col, value) in dc.entries() {
            // An IC capacitor's voltage clamp is OP-only. Its physical
            // dynamic branch equation is I - C*d(Vpos-Vneg)/dt = 0.
            if !ic_rows.contains(&row) {
                add(row, col + 1, None, value)?;
            }
        }
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let cap = circuit.capacitors.capacitances[index];
            let terminals = [(stamp.pp.row, 1.0), (stamp.nn.row, -1.0)];
            if let Some(branch) = circuit.capacitors.ic_branch_indices[index] {
                let row = nodes + branch - 1;
                add(row, row + 1, None, 1.0)?;
                for (node, sign) in terminals {
                    if node != 0 {
                        add(row, size + node, None, -sign * cap)?;
                    }
                }
            } else {
                for (row, row_sign) in terminals {
                    for (node, col_sign) in terminals {
                        if row != 0 && node != 0 {
                            add(row - 1, size + node, None, row_sign * col_sign * cap)?;
                        }
                    }
                }
            }
        }
        for (index, &branch) in circuit.inductors.branch_indices.iter().enumerate() {
            add(
                nodes + branch - 1,
                size + nodes + branch,
                None,
                -circuit.inductors.inductances[index],
            )?;
        }
        for pair in &circuit.coupled_inductor_pairs {
            for (row, col) in [
                (pair.branch1_ordinal, pair.branch2_ordinal),
                (pair.branch2_ordinal, pair.branch1_ordinal),
            ] {
                add(nodes + row - 1, size + nodes + col, None, -pair.device.m)?;
            }
        }
        for (index, &branch) in circuit.voltage_sources.branch_indices.iter().enumerate() {
            add(nodes + branch - 1, 0, Some(ForestValue::Source(index)), 1.0)?;
        }
        for (index, (&pos, &neg)) in circuit
            .current_sources
            .node_pos
            .iter()
            .zip(&circuit.current_sources.node_neg)
            .enumerate()
        {
            for (node, sign) in [(pos, -1.0), (neg, 1.0)] {
                if node != 0 {
                    add(
                        node - 1,
                        0,
                        Some(ForestValue::SourceDerivative {
                            index,
                            order: 0,
                            current: true,
                        }),
                        sign,
                    )?;
                }
            }
        }
        drop(dc);
        // E*x' + A*x = b(t). Eliminate only derivative columns first.
        // Each independent residual C*x=d(t) also implies C*x'=d'(t).
        // Feeding that derivative back exposes hidden higher-index constraints.
        let mut pending_words = rows.iter().map(VoltageRow::words).sum::<usize>();
        while let Some(row) = rows.pop() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            pending_words -= row.words();
            dynamic.limits.max_result_values = limits.max_result_values.saturating_sub(
                overhead
                    .saturating_add(pending_words)
                    .saturating_add(algebraic.retained_words),
            );
            let Some(row) = dynamic.admit(row, size + 1, abort)? else {
                continue;
            };
            algebraic.limits.max_result_values = limits.max_result_values.saturating_sub(
                overhead
                    .saturating_add(pending_words)
                    .saturating_add(dynamic.retained_words),
            );
            if let Some(remainder) = algebraic.admit(row, 1, abort)? {
                if !remainder.values.is_empty() {
                    return Err(SimulationError::Circuit("PSS linear descriptor imposes an inconsistent or nonunique source constraint".to_owned()));
                }
                continue;
            }
            let row = &algebraic.rows.last().unwrap().1;
            algebraic.check_cost(row.words().saturating_mul(3))?;
            let derivative = VoltageRow {
                nodes: row
                    .nodes
                    .iter()
                    .map(|(&node, value)| (size + node, value.clone()))
                    .collect(),
                values: row
                    .values
                    .iter()
                    .map(|(&source, value)| Ok((source.differentiated()?, value.clone())))
                    .collect::<Result<_, SimulationError>>()?,
                query: BigInt::default(),
            };
            pending_words = pending_words.saturating_add(derivative.words());
            rows.push(derivative);
        }
        drop(dynamic);
        drop(rows);
        algebraic.limits.max_result_values = limits.max_result_values.saturating_sub(overhead);
        // Prefer physical winding currents before charge voltages. A voltage
        // controlled by a winding must not displace that winding's coordinate.
        let mut representatives = Vec::new();
        for (index, &branch) in circuit.inductors.branch_indices.iter().enumerate() {
            if algebraic.add(
                [(nodes + branch, 1.0)],
                ForestValue::CurrentState(representatives.len()),
                abort,
            )? {
                representatives.push(index);
            }
        }
        let mut voltage_branches = Vec::new();
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit.capacitors.capacitances[index] != 0.0
                && algebraic.add(
                    [(stamp.pp.row, 1.0), (stamp.nn.row, -1.0)],
                    ForestValue::State(voltage_branches.len()),
                    abort,
                )?
            {
                voltage_branches.push(VoltageBranch::Capacitor(index));
            }
        }
        let mut retained_words = size.saturating_mul(3);
        algebraic.reserve_retained_words(retained_words)?;
        let mut forms = Vec::with_capacity(size);
        for unknown in 1..=size {
            let row = algebraic.port_row(unknown, 0, abort)?;
            if !row.nodes.is_empty() {
                return Err(SimulationError::Circuit(
                    "PSS linear descriptor has an undetermined algebraic voltage or current"
                        .to_owned(),
                ));
            }
            retained_words =
                retained_words.saturating_add(row.values.len().saturating_mul(FORM_TERM_WORDS));
            forms.push(algebraic.compile_row(row)?);
        }
        retained_words = retained_words.saturating_add(circuit.capacitors.len().saturating_mul(3));
        algebraic.reserve_retained_words(circuit.capacitors.len().saturating_mul(3))?;
        let mut charge_forcing = Vec::with_capacity(circuit.capacitors.len());
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            let form: Vec<_> = if circuit.capacitors.capacitances[index] == 0.0 {
                Vec::new()
            } else {
                let mut row = algebraic.port_row(stamp.pp.row, stamp.nn.row, abort)?;
                if !row.nodes.is_empty() {
                    return Err(SimulationError::Circuit(
                        "PSS charge forcing is outside its independent state basis".to_owned(),
                    ));
                }
                // Filter before conversion so an empty forcing does not keep
                // an unused vector allocation for discarded state terms.
                row.values.retain(|value, _| value.source().is_some());
                algebraic.compile_row(row)?
            };
            retained_words =
                retained_words.saturating_add(form.len().saturating_mul(FORM_TERM_WORDS));
            charge_forcing.push(form);
        }
        Ok(PssStateBasis {
            voltage_branches,
            forest: Vec::new(),
            voltage_constraints: None,
            currents: PssCurrentBasis::from_descriptor(circuit, representatives),
            descriptor: Some(Self {
                charge_forcing: std::sync::Arc::new(charge_forcing),
                solution: PssVoltageConstraints {
                    node_forms: std::sync::Arc::new(forms),
                    max_values: limits.max_result_values,
                    retained_words,
                },
            }),
        })
    }

    pub(in crate::engine::pss::state) fn solve(
        &self,
        circuit: &mut CircuitData,
        state: &[Value],
        voltage_count: usize,
        solution: &mut [Value],
    ) -> Result<(), SimulationError> {
        self.solution
            .ensure_evaluation_work(solution.len().saturating_mul(2), self.solution.max_terms())?;
        let mut trial = vec![0.0; solution.len()];
        self.solution.solve(&mut trial, |value| {
            value.evaluate(circuit, state, voltage_count)
        })?;
        solution.copy_from_slice(&trial);
        Ok(())
    }

    fn winding_form<'a>(
        &'a self,
        circuit: &CircuitData,
        index: usize,
    ) -> &'a [(ForestValue, Value)] {
        &self.solution.node_forms[circuit.num_nodes() + circuit.inductors.branch_indices[index] - 1]
    }

    pub(in crate::engine::pss::state) fn projection(
        &self,
        circuit: &CircuitData,
        index: usize,
    ) -> Vec<(usize, Value)> {
        self.winding_form(circuit, index)
            .iter()
            .filter_map(|&(value, weight)| {
                if let ForestValue::CurrentState(index) = value {
                    Some((index, weight))
                } else {
                    None
                }
            })
            .collect()
    }

    pub(in crate::engine::pss::state) fn has_prescribed_currents(
        &self,
        circuit: &CircuitData,
    ) -> bool {
        (0..circuit.inductors.len()).any(|index| {
            self.winding_form(circuit, index)
                .iter()
                .any(|(value, _)| value.source().is_some())
        })
    }

    pub(in crate::engine::pss::state) fn has_prescribed_charge(&self) -> bool {
        self.charge_forcing.iter().any(|form| !form.is_empty())
    }

    pub(in crate::engine::pss::state) fn charge_companion(
        &self,
        circuit: &CircuitData,
        rates: &mut [Value],
        offsets: &mut [Vec<Value>; 3],
        times: [Value; 2],
        step: PssCompanionStep<'_>,
    ) -> Result<(), SimulationError> {
        self.forcing_companion(
            self.charge_forcing.iter().map(Vec::as_slice),
            circuit,
            rates,
            offsets,
            times,
            step,
        )
    }

    pub(in crate::engine::pss::state) fn ensure_regular_forcing(
        &self,
        circuit: &CircuitData,
        period: Value,
    ) -> Result<(), SimulationError> {
        let mut orders = BTreeMap::new();
        for form in self.solution.node_forms.iter() {
            for &(value, _) in form {
                if let Some((current, index, order)) = value.source() {
                    let max_order = orders.entry((current, index)).or_insert(0);
                    *max_order = (*max_order).max(order);
                }
            }
        }
        for index in 0..circuit.inductors.len() {
            for &(value, _) in self.winding_form(circuit, index) {
                if let Some((current, index, order)) = value.source() {
                    let max_order = orders.entry((current, index)).or_insert(0);
                    *max_order = (*max_order).max(order.saturating_add(1));
                }
            }
        }
        for form in self.charge_forcing.iter() {
            for &(value, _) in form {
                if let Some((current, index, order)) = value.source() {
                    let max_order = orders.entry((current, index)).or_insert(0);
                    *max_order = (*max_order).max(order.saturating_add(1));
                }
            }
        }
        for ((current, index), order) in orders {
            if order == 0 {
                continue;
            }
            let (name, regular) = if current {
                (
                    &circuit.current_sources.names[index],
                    circuit
                        .current_sources
                        .has_regular_periodic_derivative(index, period, order),
                )
            } else {
                (
                    &circuit.voltage_sources.names[index],
                    circuit
                        .voltage_sources
                        .has_regular_periodic_derivative(index, period, order),
                )
            };
            if !regular {
                return Err(SimulationError::Circuit(format!(
                    "PSS coupled state constraint requires a regular periodic derivative of order {order} for source {name}"
                )));
            }
        }
        Ok(())
    }

    pub(in crate::engine::pss::state) fn source_companion(
        &self,
        circuit: &CircuitData,
        rates: &mut [Value],
        offsets: &mut [Vec<Value>; 3],
        times: [Value; 2],
        step: PssCompanionStep<'_>,
    ) -> Result<(), SimulationError> {
        self.forcing_companion(
            (0..circuit.inductors.len()).map(|index| self.winding_form(circuit, index)),
            circuit,
            rates,
            offsets,
            times,
            step,
        )
    }

    fn forcing_companion<'a>(
        &self,
        forms: impl Iterator<Item = &'a [(ForestValue, Value)]> + Clone,
        circuit: &CircuitData,
        rates: &mut [Value],
        offsets: &mut [Vec<Value>; 3],
        times: [Value; 2],
        step: PssCompanionStep<'_>,
    ) -> Result<(), SimulationError> {
        let max_terms = forms.clone().map(<[_]>::len).max().unwrap_or(0);
        let workspace_words = offsets.iter().fold(rates.len(), |words, offsets| {
            words.saturating_add(offsets.len())
        });
        self.solution
            .ensure_evaluation_work(workspace_words, max_terms)?;
        let mut terms = Vec::with_capacity(max_terms);
        for (index, (rate, form)) in rates.iter_mut().zip(forms).enumerate() {
            let mut forcing = |time, order| -> Result<Value, SimulationError> {
                evaluate_form(form, &mut terms, |value| {
                    if value.source().is_some() {
                        value.forcing(circuit, time, order)
                    } else {
                        Ok(0.0)
                    }
                })
            };
            let previous = if step.coeff.coeff_i_n == 0.0 {
                0.0
            } else {
                forcing(times[0], 1)?
            };
            *rate = rspice_veriloga_runtime::arithmetic::sum_products(
                [
                    (forcing(step.t_next, 1)?, 1.0),
                    (previous, step.coeff.coeff_i_n),
                ]
                .into_iter(),
            )
            .map_err(|_| precision_error())?;
            for (offset, time) in offsets.iter_mut().zip([step.t_next, times[0], times[1]]) {
                offset[index] = forcing(time, 0)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::abort_signal::NoAbort;

    #[test]
    fn coupled_descriptor_limits_cancellation_and_failed_state_installation() {
        let (engine, mut circuit) = build(
            "V1 in 0 SIN(0 1 1)\nL1 in mid 0.1\nR1 mid 0 1\nH1 out 0 L1 2\nC1 out 0 0.2",
            crate::config::SpiceDialect::Ngspice,
        );
        for calls in [1, 30] {
            assert!(matches!(
                PssStateBasis::new(
                    &circuit,
                    crate::resource::ResourceLimits::default(),
                    &crate::abort_signal::CountingAbort::new(calls)
                ),
                Err(SimulationError::Aborted)
            ));
        }
        assert!(matches!(
            PssStateBasis::new(
                &circuit,
                crate::resource::ResourceLimits {
                    max_result_values: 128,
                    ..Default::default()
                },
                &NoAbort
            ),
            Err(SimulationError::ResourceLimit(_))
        ));
        circuit.set_state(&[0.3]).unwrap();
        circuit.current_source_times = [0.4, 0.2];
        let solution = circuit.solution_scratch.clone();
        assert!(circuit.set_state(&[Value::NAN]).is_err());
        assert_eq!(circuit.solution_scratch, solution);
        assert_eq!(circuit.extract_state(), [0.3]);
        assert_eq!(circuit.current_source_times, [0.4, 0.2]);
        let mut config = engine.config.clone();
        config.resource_limits.max_matrix_unknowns = circuit.matrix_size();
        Engine::new(config.clone())
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        config.resource_limits.max_matrix_unknowns -= 1;
        assert!(matches!(
            Engine::new(config).pss_initial_node_solution(&mut circuit, &NoAbort),
            Err(SimulationError::ResourceLimit(_))
        ));
    }

    #[test]
    fn coupled_descriptor_rejects_impulses_and_undetermined_physical_unknowns() {
        let (_, circuit) = build(
            "V1 in 0 PULSE(0 1 0 0.1 0.1 0.3 1)\nR1 in 0 4\nCin in 0 0.3\nH1 out 0 V1 2\nCout out 0 0.2",
            crate::config::SpiceDialect::Ngspice,
        );
        let error = circuit
            .ensure_regular_prescribed_currents(1.0)
            .unwrap_err()
            .to_string();
        assert!(error.contains("order 2") && error.contains("V1"), "{error}");
        for devices in [
            "E1 out 0 in 0 2\nR1 out 0 0\nCout out 0 0.2",
            "E1 out 0 in 0 0\nR1 out 0 0\nCout out 0 0.2",
        ] {
            let deck = Netlist::parse(&format!(
                "Singular descriptor\nV1 in 0 0\n{devices}\n.end\n"
            ))
            .unwrap();
            assert!(PssCircuit::new(Engine::default().build_circuit(&deck).unwrap()).is_err());
        }
    }

    fn build(deck: &str, dialect: crate::config::SpiceDialect) -> (Engine, PssCircuit) {
        let engine =
            Engine::new(crate::engine::SimulationConfig::default().with_spice_dialect(dialect));
        let netlist = Netlist::parse(&format!("Coupled descriptor\n{deck}\n.end\n")).unwrap();
        let circuit = PssCircuit::new(engine.build_circuit(&netlist).unwrap()).unwrap();
        assert!(circuit.basis.descriptor.is_some());
        (engine, circuit)
    }

    fn close(actual: Value, expected: Value) {
        assert!(
            (actual - expected).abs() <= 2e-13 * expected.abs().max(1.0),
            "{actual:e} != {expected:e}"
        );
    }

    #[test]
    fn coupled_descriptor_resolves_source_current_and_second_derivative_charge() {
        for dialect in [
            crate::config::SpiceDialect::Ngspice,
            crate::config::SpiceDialect::Xyce,
        ] {
            for gain in [0.0, 2.0, -0.5] {
                for input_cap in [0.0, 0.3] {
                    let (engine, mut circuit) = build(
                        &format!(
                            "V1 in 0 SIN(0.7 1 1 0 0 37)\nR1 in 0 4\nCin in 0 {input_cap}\nH1 out 0 V1 {gain}\nCout out 0 0.2 IC=0"
                        ),
                        dialect,
                    );
                    assert_eq!(circuit.state_dimension(), 0);
                    circuit.ensure_regular_prescribed_currents(1.0).unwrap();
                    circuit.set_state(&[]).unwrap();
                    let solution = engine
                        .pss_initial_node_solution(&mut circuit, &NoAbort)
                        .unwrap();
                    let phase = 37_f64.to_radians();
                    let omega = std::f64::consts::TAU;
                    let v = 0.7 + phase.sin();
                    let slope = omega * phase.cos();
                    let acceleration = -omega * omega * phase.sin();
                    let input_current = -v / 4.0 - input_cap * slope;
                    let output_current = 0.2 * gain * (slope / 4.0 + input_cap * acceleration);
                    close(circuit.capacitors.v_prev[1], gain * input_current);
                    close(
                        solution
                            [circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1],
                        input_current,
                    );
                    close(
                        solution[circuit.num_nodes() + circuit.ccvs.branch_indices[0] - 1],
                        output_current,
                    );
                    if let Some(branch) = circuit.capacitors.ic_branch_indices[1] {
                        close(solution[circuit.num_nodes() + branch - 1], -output_current);
                    }
                }
            }
        }
    }

    #[test]
    fn coupled_descriptor_preserves_winding_state_and_algebraic_divider() {
        let (engine, mut circuit) = build(
            "V1 in 0 SIN(0.7 1 1 0 0 37)\nL1 in mid 0.1\nR1 mid 0 1\nH1 out 0 L1 2\nC1 out 0 0.2",
            crate::config::SpiceDialect::Ngspice,
        );
        assert_eq!(circuit.state_basis_names(), ["L:L1"]);
        assert_eq!(
            circuit.inductor_probe_projection("L1").unwrap().1,
            [(0, 1.0)]
        );
        for current in [-0.8, 0.4] {
            circuit.set_state(&[current]).unwrap();
            close(circuit.capacitors.v_prev[0], 2.0 * current);
            let solution = engine
                .pss_initial_node_solution(&mut circuit, &NoAbort)
                .unwrap();
            let rate = (0.7 + 37_f64.to_radians().sin() - current) / 0.1;
            close(
                solution[circuit.num_nodes() + circuit.ccvs.branch_indices[0] - 1],
                -0.4 * rate,
            );
            close(
                solution[circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1],
                -current,
            );
        }
        let (engine, mut circuit) = build(
            "V1 src 0 SIN(0.7 1 1 0 0 37)\nR1 src in 1\nR2 in 0 1\nE1 out 0 in 0 2\nC1 out 0 0.2",
            crate::config::SpiceDialect::Ngspice,
        );
        assert_eq!(circuit.state_dimension(), 0);
        circuit.set_state(&[]).unwrap();
        close(
            circuit.capacitors.v_prev[0],
            0.7 + 37_f64.to_radians().sin(),
        );
        let solution = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        close(
            solution[circuit.num_nodes() + circuit.vcvs.branch_indices[0] - 1],
            -0.2 * std::f64::consts::TAU * 37_f64.to_radians().cos(),
        );
    }
}

//! Constant MNA descriptor closure for coupled charge and flux constraints.

use super::*;

mod nonlinear;
use nonlinear::NonlinearForcing;

#[derive(Debug, Clone)]
pub(in crate::engine::pss::state) struct PssDescriptor {
    solution: PssVoltageConstraints,
    charge_forcing: std::sync::Arc<Vec<Vec<(ForestValue, Value)>>>,
    behavioral: BehavioralForcing,
    nonlinear: Option<NonlinearForcing>,
}

impl PssDescriptor {
    pub(in crate::engine::pss::state) fn applies(circuit: &CircuitData) -> bool {
        use crate::engine::periodic_capability::PeriodicDeviceFamily as F;
        // Ordinary independent-source networks retain their linear-time
        // topological basis. Nonlinear current symbols are admitted only after
        // their exact port dependencies certify a unique algebraic island.
        // A DC linearization is never used as a large-signal rank certificate.
        (!circuit.vcvs.is_empty()
            || !circuit.vccs.is_empty()
            || !circuit.cccs.is_empty()
            || !circuit.ccvs.is_empty())
            && circuit
                .capacitors
                .value_expressions
                .iter()
                .all(Option::is_none)
            && circuit
                .behavioral_sources
                .voltage_sources
                .iter()
                .all(|source| {
                    source
                        .prescribed_time_program()
                        .is_some_and(|(program, _)| crate::expr::TimeDerivatives::supports(program))
                })
            && circuit
                .behavioral_sources
                .current_sources
                .iter()
                .all(|source| {
                    source
                        .prescribed_time_program()
                        .is_some_and(|(program, _)| crate::expr::TimeDerivatives::supports(program))
                })
            && circuit
                .diodes
                .devices
                .iter()
                .all(|diode| diode.has_monotone_c1_conduction())
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
                        | F::BehavioralSource
                        | F::Diode
                ) || family.instance_count(circuit) == 0
            })
    }

    pub(in crate::engine::pss::state) fn build(
        circuit: &CircuitData,
        limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Option<PssStateBasis>, SimulationError> {
        Self::build_with_ports(circuit, limits, abort, None)
    }

    fn build_with_ports(
        circuit: &CircuitData,
        limits: crate::resource::ResourceLimits,
        abort: &dyn AbortSignal,
        nonlinear_ports: Option<Vec<Vec<(ForestValue, Value)>>>,
    ) -> Result<Option<PssStateBasis>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        let size = circuit.matrix_size();
        let mut prepared_words = 0_usize;
        if let Some(ports) = &nonlinear_ports {
            prepared_words = ports.len().saturating_mul(3);
            for port in ports {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                prepared_words =
                    prepared_words.saturating_add(port.len().saturating_mul(FORM_TERM_WORDS));
            }
        }
        let port_overhead = if nonlinear_ports.is_some() {
            circuit.diodes.len().saturating_mul(32)
        } else {
            0
        };
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
            size.saturating_mul(48)
                .saturating_add(triplet_words)
                .saturating_add(prepared_words)
                .saturating_add(port_overhead),
            limits.max_result_values,
        )?;
        let mut dynamic =
            PssVoltageConstraintBuilder::new(size.saturating_mul(2).saturating_add(1), limits)?;
        let mut algebraic = PssVoltageConstraintBuilder::new(size.saturating_add(1), limits)?;
        let overhead = size
            .saturating_mul(32)
            .saturating_add(prepared_words)
            .saturating_add(port_overhead);
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
                            kind: ConstraintSource::Current,
                        }),
                        sign,
                    )?;
                }
            }
        }
        for (index, source) in circuit
            .behavioral_sources
            .voltage_sources
            .iter()
            .enumerate()
        {
            let branch = nodes + source.branch_ordinal;
            for (node, sign) in [(source.node_pos, 1.0), (source.node_neg, -1.0)] {
                if node != 0 {
                    add(node - 1, branch, None, sign)?;
                    add(branch - 1, node, None, sign)?;
                }
            }
            add(
                branch - 1,
                0,
                Some(ForestValue::BehavioralSource(index)),
                1.0,
            )?;
        }
        for (index, source) in circuit
            .behavioral_sources
            .current_sources
            .iter()
            .enumerate()
        {
            for (node, sign) in [(source.node_pos, -1.0), (source.node_neg, 1.0)] {
                if node != 0 {
                    add(
                        node - 1,
                        0,
                        Some(ForestValue::SourceDerivative {
                            index,
                            order: 0,
                            kind: ConstraintSource::BehavioralCurrent,
                        }),
                        sign,
                    )?;
                }
            }
        }
        for (index, diode) in circuit.diodes.devices.iter().enumerate() {
            for (node, sign) in [(diode.node_anode, -1.0), (diode.node_cathode, 1.0)] {
                if node != 0 {
                    add(
                        node - 1,
                        0,
                        Some(ForestValue::SourceDerivative {
                            index,
                            order: 0,
                            kind: ConstraintSource::Diode,
                        }),
                        sign,
                    )?;
                }
            }
        }
        drop(dc);
        if nonlinear_ports.is_some() {
            // Replay the physical equations with the solved diode voltages
            // first. Their differentiated constraints preserve small port
            // rates in downstream capacitor/source currents as well.
            PssVoltageConstraintBuilder::ensure_words(
                words.saturating_add(circuit.diodes.len().saturating_mul(128)),
                limits.max_result_values,
            )?;
            rows.reserve(circuit.diodes.len());
            for (index, diode) in circuit.diodes.devices.iter().enumerate() {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                let mut row = VoltageRow::default();
                for (node, sign) in [(diode.node_anode, 1), (diode.node_cathode, -1)] {
                    if node != 0 {
                        PssVoltageConstraintBuilder::add_integer(
                            &mut row.nodes,
                            node,
                            BigInt::from(sign),
                        );
                    }
                }
                row.values.insert(
                    ForestValue::SourceDerivative {
                        index,
                        order: 0,
                        kind: ConstraintSource::DiodeVoltage,
                    },
                    BigInt::from(1),
                );
                rows.push(row);
            }
        }
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
                    if nonlinear_ports.is_some() {
                        // These are the original port equations (and their
                        // derivatives), evaluated by NonlinearForcing. The
                        // first pass already rejected independent source-only
                        // constraints and ports carrying dynamic coordinates.
                        continue;
                    }
                    // A constitutive inverse or differential nonlinear closure
                    // is needed here; do not certify it by linearization.
                    if !circuit.diodes.is_empty() {
                        return Ok(None);
                    }
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
        let mut charge_branches = Vec::new();
        for (index, stamp) in circuit.capacitors.stamps.iter().enumerate() {
            if circuit.capacitors.capacitances[index] != 0.0
                && algebraic.add(
                    [(stamp.pp.row, 1.0), (stamp.nn.row, -1.0)],
                    ForestValue::State(charge_branches.len()),
                    abort,
                )?
            {
                charge_branches.push(ChargeBranch::Capacitor(index));
            }
        }
        if nonlinear_ports.is_none() && !circuit.diodes.is_empty() {
            let port_headers = circuit.diodes.len().saturating_mul(3);
            algebraic.reserve_retained_words(port_headers)?;
            let mut nonlinear_ports = Vec::with_capacity(circuit.diodes.len());
            for diode in &circuit.diodes.devices {
                let row = algebraic.port_row(diode.node_anode, diode.node_cathode, abort)?;
                // State dependence or derivatives of a nonlinear current can
                // carry real charge/flux dynamics. Keep that existing basis.
                if !row.nodes.is_empty()
                    || row.values.keys().any(|value| match value.source() {
                        Some((ConstraintSource::Diode, _, order)) => order != 0,
                        Some(_) => false,
                        None => true,
                    })
                {
                    return Ok(None);
                }
                nonlinear_ports.push(algebraic.compile_row(row)?);
            }
            // At most one replay: the second pass receives the original
            // constitutive response and never compiles a new one. Release
            // the first reducer and state vectors before allocating it.
            drop(algebraic);
            drop(representatives);
            drop(charge_branches);
            drop(ic_rows);
            return Self::build_with_ports(circuit, limits, abort, Some(nonlinear_ports));
        }
        let nonlinear_ports = nonlinear_ports.unwrap_or_default();
        let mut retained_words = size.saturating_mul(3).saturating_add(prepared_words);
        algebraic.reserve_retained_words(size.saturating_mul(3))?;
        let mut forms = Vec::with_capacity(size);
        for unknown in 1..=size {
            let row = algebraic.port_row(unknown, 0, abort)?;
            if !row.nodes.is_empty() {
                if !circuit.diodes.is_empty() {
                    return Ok(None);
                }
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
        drop(algebraic);
        // Reserve source-order map nodes and traversal work before collecting
        // derivative requests. They coexist with compiled forms and caches.
        let source_count = circuit
            .voltage_sources
            .len()
            .saturating_add(circuit.current_sources.len())
            .saturating_add(circuit.behavioral_sources.voltage_sources.len())
            .saturating_add(circuit.behavioral_sources.current_sources.len())
            .saturating_add(circuit.diodes.len().saturating_mul(2));
        retained_words = retained_words.saturating_add(source_count.saturating_mul(64));
        PssVoltageConstraintBuilder::ensure_words(retained_words, limits.max_result_values)?;
        let mut descriptor = Self {
            behavioral: BehavioralForcing::default(),
            nonlinear: None,
            charge_forcing: std::sync::Arc::new(charge_forcing),
            solution: PssVoltageConstraints {
                node_forms: std::sync::Arc::new(forms),
                max_values: limits.max_result_values,
                retained_words,
            },
        };
        // Exact closure determines the derivative orders. Evaluate each
        // prescribed B expression once at the initialization time, with the
        // same resource and cancellation contract as subsequent trial times.
        let mut orders = descriptor.forcing_orders(circuit, abort)?;
        if !nonlinear_ports.is_empty() {
            let Some(nonlinear) = NonlinearForcing::new(
                nonlinear_ports,
                &mut orders,
                &mut descriptor.solution.retained_words,
                limits.max_result_values,
                abort,
            )?
            else {
                return Ok(None);
            };
            descriptor.nonlinear = Some(nonlinear);
        }
        descriptor.behavioral = BehavioralForcing::new(
            circuit,
            &orders,
            &mut descriptor.solution.retained_words,
            limits.max_result_values,
            abort,
        )?;
        if let Some(nonlinear) = &mut descriptor.nonlinear {
            nonlinear.initialize(
                circuit,
                &descriptor.behavioral,
                descriptor
                    .solution
                    .max_values
                    .saturating_sub(descriptor.solution.retained_words),
                abort,
            )?;
        }
        Ok(Some(PssStateBasis {
            charge_branches,
            node_units: vec![CoordinateUnit::Voltage; circuit.num_nodes() + 1],
            forest: Vec::new(),
            voltage_constraints: None,
            currents: PssCurrentBasis::from_descriptor(circuit, representatives),
            descriptor: Some(descriptor),
        }))
    }

    pub(in crate::engine::pss::state) fn solve(
        &self,
        circuit: &mut CircuitData,
        state: &[Value],
        voltage_count: usize,
        time: Value,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = self.solution.node_forms.len().saturating_add(1);
        self.solution
            .ensure_evaluation_work(size.saturating_mul(2), self.solution.max_terms())?;
        let mut trial = vec![0.0; size];
        self.solution.solve(&mut trial, |value| {
            if value.source().is_some() {
                self.forcing_value(circuit, value, time, 0)
            } else {
                value.evaluate(circuit, state, voltage_count)
            }
        })?;
        Ok(trial)
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

    fn forcing_orders(
        &self,
        circuit: &CircuitData,
        abort: &dyn AbortSignal,
    ) -> Result<BTreeMap<(ConstraintSource, usize), usize>, SimulationError> {
        let mut orders = BTreeMap::new();
        let mut count = 0_usize;
        let mut record = |value: ForestValue, extra: usize| -> Result<(), SimulationError> {
            if count.is_multiple_of(64) && abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            count += 1;
            if let Some((kind, index, order)) = value.source() {
                let order = order.checked_add(extra).ok_or_else(|| {
                    SimulationError::Circuit("PSS source derivative order overflow".to_owned())
                })?;
                let maximum = orders.entry((kind, index)).or_insert(0);
                *maximum = (*maximum).max(order);
            }
            Ok(())
        };
        for form in self.solution.node_forms.iter() {
            for &(value, _) in form {
                record(value, 0)?;
            }
        }
        for index in 0..circuit.inductors.len() {
            for &(value, _) in self.winding_form(circuit, index) {
                record(value, 1)?;
            }
        }
        for form in self.charge_forcing.iter() {
            for &(value, _) in form {
                record(value, 1)?;
            }
        }
        if let Some(nonlinear) = &self.nonlinear {
            nonlinear.extend_orders(&mut orders, abort)?;
        }
        Ok(orders)
    }

    pub(in crate::engine::pss::state) fn ensure_regular_forcing(
        &self,
        circuit: &CircuitData,
        period: Value,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        self.behavioral.ensure_regular(circuit, period, abort)?;
        for ((kind, index), order) in self.forcing_orders(circuit, abort)? {
            if kind.is_behavioral() || kind.is_nonlinear() {
                continue;
            }
            if order == 0 {
                continue;
            }
            let (name, regular) = if kind == ConstraintSource::Current {
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

    pub(in crate::engine::pss::state) fn prepare_forcing(
        &mut self,
        circuit: &CircuitData,
        times: [Value; 3],
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        self.behavioral.prepare(
            circuit,
            times,
            self.solution
                .max_values
                .saturating_sub(self.solution.retained_words),
            abort,
        )?;
        if let Some(nonlinear) = &mut self.nonlinear {
            nonlinear.prepare(
                circuit,
                &self.behavioral,
                times,
                self.solution
                    .max_values
                    .saturating_sub(self.solution.retained_words),
                abort,
            )?;
        }
        Ok(())
    }

    fn forcing_value(
        &self,
        circuit: &CircuitData,
        value: ForestValue,
        time: Value,
        order: usize,
    ) -> Result<Value, SimulationError> {
        match value.source() {
            Some((kind, _, _)) if kind.is_nonlinear() => self
                .nonlinear
                .as_ref()
                .ok_or_else(precision_error)?
                .value(value, time, order),
            Some((kind, _, _)) if kind.is_behavioral() => self.behavioral.value(value, time, order),
            Some(_) => value.forcing(circuit, time, order),
            None => Ok(0.0),
        }
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
                    self.forcing_value(circuit, value, time, order)
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
    fn nonlinear_descriptor_preserves_port_voltages_and_rates_after_large_drops() {
        for dialect in [
            crate::config::SpiceDialect::Ngspice,
            crate::config::SpiceDialect::Xyce,
        ] {
            for (source, resistance, saturation, gain, output_scale, current_scale) in [
                (1.0, 1e20, 1.0, 1e22, 100.0, 1e-20),
                (1e300, 1e300, 1e298, 1e298, 1.0, 1.0),
            ] {
                let (engine, mut circuit) = build(
                    &format!(
                        "V1 src 0 SIN({source:e} {:e} 1)\nR1 src in {resistance:e}\nD1 in 0 DM\n.model DM D(IS={saturation:e})\nE1 out 0 in 0 {gain:e}\nCout out 0 1u",
                        0.5 * source,
                    ),
                    dialect,
                );
                assert_eq!(circuit.state_dimension(), 0);
                circuit.set_state(&[]).unwrap();
                let solution = engine
                    .pss_initial_node_solution(&mut circuit, &NoAbort)
                    .unwrap();
                // Both biases are within 1e-18 relative error of the linear
                // Shockley limit. Large resistor drops must not erase them.
                let thermal = circuit.diodes.devices[0].vt;
                let expected_output = output_scale * thermal;
                let input = solution[circuit.get_node_by_name("in").unwrap() - 1];
                let output = solution[circuit.get_node_by_name("out").unwrap() - 1];
                assert!(
                    (input * gain / expected_output - 1.0).abs() < 2e-12,
                    "{dialect:?}: input={input:e}"
                );
                assert!(
                    (output / expected_output - 1.0).abs() < 2e-12,
                    "{dialect:?}: output={output:e}"
                );
                let source_current =
                    solution[circuit.num_nodes() + circuit.voltage_sources.branch_indices[0] - 1];
                let output_current =
                    solution[circuit.num_nodes() + circuit.vcvs.branch_indices[0] - 1];
                assert!((source_current / -current_scale - 1.0).abs() < 2e-12);
                let expected_current = -1e-6 * expected_output * 0.5 * std::f64::consts::TAU;
                assert!(
                    (output_current / expected_current - 1.0).abs() < 2e-12,
                    "{dialect:?}: current={output_current:e}"
                );
            }
        }
    }

    #[test]
    fn nonlinear_descriptor_solves_feedforward_ports_with_zero_external_drive() {
        let (engine, mut circuit) = build(
            "V1 src 0 SIN(0.3 0.01 1)\nD1 src 0 DM\nH1 out 0 V1 1e6\nD2 out 0 DM\n.model DM D(IS=1e-12)\nCout out 0 1u",
            crate::config::SpiceDialect::Ngspice,
        );
        assert_eq!(circuit.state_dimension(), 0);
        circuit.set_state(&[]).unwrap();
        let solution = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        let (current, conductance) = circuit.diodes.devices[0].stamped_current_and_conductance(0.3);
        let output = -1e6 * current;
        let rate = -1e6 * conductance * 0.01 * std::f64::consts::TAU;
        let load = circuit.diodes.devices[1].stamped_conduction_current(output);
        close(
            solution[circuit.get_node_by_name("out").unwrap() - 1],
            output,
        );
        close(
            solution[circuit.num_nodes() + circuit.ccvs.branch_indices[0] - 1],
            -1e-6 * rate - load,
        );
    }

    #[test]
    fn nonlinear_descriptor_keeps_small_signal_constraint_accuracy_under_gain() {
        let (engine, mut circuit) = build(
            "V1 src 0 1e-14\nR1 src in 1\nD1 in 0 DM\n.model DM D(IS=0.01)\nE1 out 0 in 0 1e14\nCout out 0 1u",
            crate::config::SpiceDialect::Ngspice,
        );
        assert_eq!(circuit.state_dimension(), 0);
        circuit.set_state(&[]).unwrap();
        let solution = engine
            .pss_initial_node_solution(&mut circuit, &NoAbort)
            .unwrap();
        let input = solution[circuit.get_node_by_name("in").unwrap() - 1];
        let output = solution[circuit.get_node_by_name("out").unwrap() - 1];
        let diode = &circuit.diodes.devices[0];
        // expm1 gives an independent forward-law oracle near zero bias.
        let slope = diode.stamped_current_and_conductance(0.0).1;
        let gmin = slope - diode.is / (diode.n * diode.vt);
        let current = diode.is * (input / (diode.n * diode.vt)).exp_m1() + gmin * input;
        assert!(
            (1e-14 - input - current).abs() < 1e-27,
            "KCL at {input:e}, output={output:e}: residual {:e}",
            1e-14 - input - current
        );
        let expected = 1.0 / (1.0 + slope);
        assert!(
            (output - expected).abs() < 2e-12,
            "{output:e} vs {expected:e}"
        );
    }

    #[test]
    fn nonlinear_descriptor_solves_implicit_divider_and_controlled_charge() {
        for dialect in [
            crate::config::SpiceDialect::Ngspice,
            crate::config::SpiceDialect::Xyce,
        ] {
            let (engine, mut circuit) = build(
                "V1 src 0 SIN(0.8 0.4 1)\nR1 src in 100\nR2 in 0 200\nD1 in 0 DM\n.model DM D(IS=1e-12)\nE1 out 0 in 0 2\nCout out 0 1u",
                dialect,
            );
            assert_eq!(circuit.state_dimension(), 0);
            circuit
                .ensure_regular_prescribed_currents(1.0, &NoAbort)
                .unwrap();
            circuit.set_state(&[]).unwrap();
            let solution = engine
                .pss_initial_node_solution(&mut circuit, &NoAbort)
                .unwrap();
            let voltage = solution[circuit.get_node_by_name("in").unwrap() - 1];
            let diode = &circuit.diodes.devices[0];
            let (current, slope) = diode.stamped_current_and_conductance(voltage);
            close((0.8 - voltage) / 100.0, voltage / 200.0 + current);
            let rate = 0.4 * std::f64::consts::TAU / (1.5 + 100.0 * slope);
            close(circuit.capacitors.v_prev[0], 2.0 * voltage);
            close(
                solution[circuit.num_nodes() + circuit.vcvs.branch_indices[0] - 1],
                -2e-6 * rate,
            );
            circuit.prepare_prescribed_forcing(0.25, &NoAbort).unwrap();
            let descriptor = circuit.basis.descriptor.as_ref().unwrap();
            let source = ForestValue::SourceDerivative {
                kind: ConstraintSource::Diode,
                index: 0,
                order: 1,
            };
            close(
                descriptor
                    .nonlinear
                    .as_ref()
                    .unwrap()
                    .value(source, 0.25, 0)
                    .unwrap(),
                0.0,
            );
        }
    }

    #[test]
    fn nonlinear_descriptor_preserves_dynamic_and_nonpassive_feedback_modes() {
        for devices in [
            "I1 0 in SIN(0 1m 1)\nD1 in 0 DM\nL1 in 0 0.1\nH1 out 0 L1 2\nR1 out 0 1",
            "V1 src 0 SIN(0 1 1)\nR1 src in 1\nD1 in 0 DM\nC1 in 0 0.1\nE1 out 0 in 0 2\nR2 out 0 1",
        ] {
            let netlist = Netlist::parse(&format!(
                "Nonlinear mode\n{devices}\n.model DM D(IS=1e-12)\n.end\n"
            ))
            .unwrap();
            let data = Engine::default().build_circuit(&netlist).unwrap();
            assert!(PssDescriptor::applies(&data));
            let circuit = PssCircuit::new(data).unwrap();
            assert!(circuit.basis.descriptor.is_none());
            assert_eq!(circuit.state_dimension(), 1);
        }
        let netlist = Netlist::parse("Positive feedback\nV1 src 0 SIN(0 1 1)\nR1 src in -1\nD1 in 0 DM\nE1 out 0 in 0 2\nCout out 0 1u\n.model DM D(IS=1e-12)\n.end\n").unwrap();
        let data = Engine::default().build_circuit(&netlist).unwrap();
        assert!(
            PssDescriptor::build(&data, Default::default(), &NoAbort)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn nonlinear_descriptor_cancellation_budget_and_retry() {
        let (_, mut circuit) = build(
            "V1 in 0 SIN(0 1 1)\nR1 in 0 1k\nH1 out 0 V1 2\nCout out 0 1u\nD1 out 0 DM\n.model DM D(IS=1e-12)",
            crate::config::SpiceDialect::Ngspice,
        );
        let descriptor = circuit.basis.descriptor.as_ref().unwrap();
        assert!(matches!(
            PssStateBasis::new(
                &circuit,
                crate::resource::ResourceLimits {
                    max_result_values: descriptor.solution.retained_words,
                    ..Default::default()
                },
                &NoAbort
            ),
            Err(SimulationError::ResourceLimit(_))
        ));
        for calls in [1, 4, 8] {
            assert!(matches!(
                circuit.prepare_prescribed_forcing(
                    0.125,
                    &crate::abort_signal::CountingAbort::new(calls)
                ),
                Err(SimulationError::Aborted)
            ));
        }
        circuit.prepare_prescribed_forcing(0.125, &NoAbort).unwrap();
        assert!(
            circuit
                .basis
                .descriptor
                .as_ref()
                .unwrap()
                .nonlinear
                .as_ref()
                .unwrap()
                .value(
                    ForestValue::SourceDerivative {
                        kind: ConstraintSource::Diode,
                        index: 0,
                        order: 0
                    },
                    0.125,
                    0
                )
                .unwrap()
                .is_finite()
        );
    }

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
            .ensure_regular_prescribed_currents(1.0, &NoAbort)
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
                    circuit
                        .ensure_regular_prescribed_currents(1.0, &NoAbort)
                        .unwrap();
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

    #[test]
    fn behavioral_descriptor_resolves_higher_charge_derivatives_and_current_forcing() {
        for dialect in [
            crate::config::SpiceDialect::Ngspice,
            crate::config::SpiceDialect::Xyce,
        ] {
            let (engine, mut circuit) = build(
                "B1 in 0 V=0.7+sin(2*pi*time+0.3)\nR1 in 0 4\nCin in 0 0.3\nH1 out 0 B1 2\nCout out 0 0.2 IC=0",
                dialect,
            );
            assert_eq!(circuit.state_dimension(), 0);
            circuit
                .ensure_regular_prescribed_currents(1.0, &NoAbort)
                .unwrap();
            circuit.set_state(&[]).unwrap();
            let solution = engine
                .pss_initial_node_solution(&mut circuit, &NoAbort)
                .unwrap();
            let omega = std::f64::consts::TAU;
            let v = 0.7 + 0.3_f64.sin();
            let slope = omega * 0.3_f64.cos();
            let acceleration = -omega * omega * 0.3_f64.sin();
            let current = -v / 4.0 - 0.3 * slope;
            let output_current = 0.4 * (slope / 4.0 + 0.3 * acceleration);
            close(circuit.capacitors.v_prev[1], 2.0 * current);
            close(
                solution[circuit.num_nodes()
                    + circuit.behavioral_sources.voltage_sources[0].branch_ordinal
                    - 1],
                current,
            );
            close(
                solution[circuit.num_nodes() + circuit.ccvs.branch_indices[0] - 1],
                output_current,
            );
            if let Some(branch) = circuit.capacitors.ic_branch_indices[1] {
                close(solution[circuit.num_nodes() + branch - 1], -output_current);
            }
            circuit.prepare_prescribed_forcing(0.17, &NoAbort).unwrap();
            let source = ForestValue::BehavioralSource(0)
                .differentiated()
                .unwrap()
                .differentiated()
                .unwrap();
            close(
                circuit
                    .basis
                    .descriptor
                    .as_ref()
                    .unwrap()
                    .behavioral
                    .value(source, 0.17, 0)
                    .unwrap(),
                -omega * omega * (omega * 0.17 + 0.3).sin(),
            );

            let (engine, mut circuit) = build(
                "B1 in 0 I=sin(2*pi*time+0.3)\nL1 in 0 0.1\nH1 out 0 L1 2\nCout out 0 0.2",
                dialect,
            );
            assert_eq!(circuit.state_dimension(), 0);
            circuit
                .ensure_regular_prescribed_currents(1.0, &NoAbort)
                .unwrap();
            circuit.set_state(&[]).unwrap();
            let solution = engine
                .pss_initial_node_solution(&mut circuit, &NoAbort)
                .unwrap();
            close(circuit.inductors.i_prev[0], -0.3_f64.sin());
            close(circuit.capacitors.v_prev[0], -2.0 * 0.3_f64.sin());
            close(
                solution[circuit.get_node_by_name("in").unwrap() - 1],
                -0.1 * slope,
            );
            close(
                solution[circuit.num_nodes() + circuit.ccvs.branch_indices[0] - 1],
                0.4 * slope,
            );
        }
    }

    #[test]
    fn behavioral_descriptor_certifies_the_orbit_and_preserves_solution_dependent_modes() {
        let (_, circuit) = build(
            "B1 in 0 V=abs(sin(2*pi*time))\nR1 in 0 4\nH1 out 0 B1 2\nCout out 0 0.2",
            crate::config::SpiceDialect::Ngspice,
        );
        circuit
            .ensure_regular_prescribed_currents(1.0, &NoAbort)
            .unwrap();
        for expression in ["abs(sin(2*pi*time))", "1/(0.125+sin(2*pi*time))"] {
            let (_, circuit) = build(
                &format!(
                    "B1 in 0 V={expression}\nR1 in 0 4\nCin in 0 0.3\nH1 out 0 B1 2\nCout out 0 0.2"
                ),
                crate::config::SpiceDialect::Ngspice,
            );
            let error = circuit
                .ensure_regular_prescribed_currents(1.0, &NoAbort)
                .unwrap_err()
                .to_string();
            assert!(error.contains("B1") && error.contains("order 2"), "{error}");
        }
        let netlist = Netlist::parse(
            "Feedback mode\nB1 in 0 V=sin(v(out))\nCin in 0 0.3\nH1 out 0 B1 2\n.end\n",
        )
        .unwrap();
        let circuit = Engine::default().build_circuit(&netlist).unwrap();
        assert!(!PssDescriptor::applies(&circuit));
        let circuit = PssCircuit::new(circuit).unwrap();
        assert_eq!(circuit.state_dimension(), 1);
        let netlist = Netlist::parse("Legacy conditional\nB1 in 0 V=if(sin(2*pi*time)>0,1,0)\nR1 in 0 4\nH1 out 0 B1 2\n.end\n").unwrap();
        assert!(!PssDescriptor::applies(
            &Engine::default().build_circuit(&netlist).unwrap()
        ));
    }

    #[test]
    fn behavioral_descriptor_cancellation_and_storage_are_bounded() {
        let (_, mut circuit) = build(
            "B1 in 0 V=sin(2*pi*time)\nCin in 0 0.3\nH1 out 0 B1 2\nCout out 0 0.2",
            crate::config::SpiceDialect::Ngspice,
        );
        let descriptor = circuit.basis.descriptor.as_ref().unwrap();
        let budget = descriptor.solution.retained_words;
        let error = PssStateBasis::new(
            &circuit,
            crate::resource::ResourceLimits {
                max_result_values: budget,
                ..Default::default()
            },
            &NoAbort,
        )
        .unwrap_err();
        assert!(matches!(error, SimulationError::ResourceLimit(_)));
        assert!(matches!(
            circuit.prepare_prescribed_forcing(0.125, &crate::abort_signal::CountingAbort::new(3)),
            Err(SimulationError::Aborted)
        ));
        assert!(matches!(
            circuit.ensure_regular_prescribed_currents(
                1.0,
                &crate::abort_signal::CountingAbort::new(1)
            ),
            Err(SimulationError::Aborted)
        ));
        circuit.prepare_prescribed_forcing(0.125, &NoAbort).unwrap();
        close(
            circuit
                .basis
                .descriptor
                .as_ref()
                .unwrap()
                .behavioral
                .value(ForestValue::BehavioralSource(0), 0.125, 0)
                .unwrap(),
            std::f64::consts::FRAC_1_SQRT_2,
        );
    }
}

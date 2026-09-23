use super::*;

fn aligned(name: &str, count: usize, lengths: &[usize]) -> Result<()> {
    if lengths.iter().any(|length| *length != count) {
        return Err(error(format!("unaligned {name} circuit storage")));
    }
    Ok(())
}

impl<'a> PreparedEventCircuit<'a> {
    pub(in crate::engine::transient) fn new(
        circuit: &'a crate::CircuitData,
        flux_tolerance: Value,
        options: &EventOptions,
        abort: &dyn AbortSignal,
    ) -> Result<Self> {
        check_abort(abort)?;
        options.validate()?;
        let nodes = circuit.num_nodes();
        let size = circuit.matrix_size();
        ResourceLimitError::ensure(
            ResourceKind::MatrixUnknowns,
            size,
            options.limits.max_matrix_unknowns,
        )?;
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            size.saturating_mul(64).saturating_add(
                circuit
                    .bjts
                    .len()
                    .saturating_mul(std::mem::size_of::<Bjt>().div_ceil(8)),
            ),
            options.limits.max_result_values,
        )?;
        if size == 0 || !flux_tolerance.is_finite() || flux_tolerance <= 0.0 {
            return Err(error("invalid event size or flux tolerance"));
        }
        for family in PeriodicDeviceFamily::ALL {
            check_abort(abort)?;
            if family.instance_count(circuit) == 0 {
                continue;
            }
            use PeriodicDeviceFamily::{
                Bjt, Capacitor, CoupledInductorPair, CurrentSource, Inductor, InductorCoupling,
                Resistor, ResistorBranch, TransmissionLine, VoltageSource,
            };
            if !matches!(
                family,
                Resistor
                    | ResistorBranch
                    | Capacitor
                    | Inductor
                    | VoltageSource
                    | CurrentSource
                    | Bjt
                    | InductorCoupling
                    | CoupledInductorPair
                    | TransmissionLine
            ) {
                return Err(error(format!(
                    "{} require a prepared physical event sampler",
                    family.label()
                )));
            }
        }
        let r = &circuit.resistors;
        let rb = &circuit.resistor_branches;
        let c = &circuit.capacitors;
        let l = &circuit.inductors;
        let vs = &circuit.voltage_sources;
        let is = &circuit.current_sources;
        aligned(
            "resistor",
            r.len(),
            &[r.stamps.len(), r.conductances.len(), r.thermal.len()],
        )?;
        aligned(
            "resistor branch",
            rb.len(),
            &[
                rb.node_pos.len(),
                rb.node_neg.len(),
                rb.branch_indices.len(),
                rb.resistances.len(),
            ],
        )?;
        aligned(
            "capacitor",
            c.len(),
            &[
                c.stamps.len(),
                c.capacitances.len(),
                c.value_expressions.len(),
                c.ic_branch_indices.len(),
            ],
        )?;
        aligned(
            "inductor",
            l.len(),
            &[
                l.node_pos.len(),
                l.node_neg.len(),
                l.branch_indices.len(),
                l.inductances.len(),
            ],
        )?;
        aligned(
            "voltage source",
            vs.len(),
            &[
                vs.node_pos.len(),
                vs.node_neg.len(),
                vs.branch_indices.len(),
                vs.dc_values.len(),
                vs.source_specs.len(),
            ],
        )?;
        aligned(
            "current source",
            is.len(),
            &[
                is.node_pos.len(),
                is.node_neg.len(),
                is.dc_values.len(),
                is.source_specs.len(),
            ],
        )?;
        if r.thermal.iter().any(Option::is_some) {
            return Err(error(
                "thermal resistors require their event-state and time-partial owner",
            ));
        }
        if c.value_expressions.iter().any(Option::is_some)
            || c.ic_branch_indices.iter().any(Option::is_some)
        {
            return Err(error(
                "capacitor expressions and auxiliary current rows require their event descriptor",
            ));
        }
        if r.conductances
            .iter()
            .chain(&rb.resistances)
            .chain(&c.capacitances)
            .chain(&l.inductances)
            .chain(&vs.dc_values)
            .chain(&is.dc_values)
            .any(|value| !value.is_finite())
        {
            return Err(error("nonfinite prepared circuit coefficient"));
        }
        let mut ports = Vec::new();
        let mut equations = vec![None; size - nodes];
        let mut constant_sources = Vec::new();
        let claim = |equations: &mut [Option<EventBranchEquation>],
                     ordinal: usize,
                     row: EventBranchEquation|
         -> Result<()> {
            let slot = ordinal
                .checked_sub(1)
                .and_then(|index| equations.get_mut(index))
                .ok_or_else(|| error("branch outside the prepared circuit"))?;
            if slot.replace(row).is_some() {
                return Err(error("multiple owners for an event branch"));
            }
            Ok(())
        };
        let terminals = |positive: usize, negative: usize| -> Result<()> {
            if positive > nodes || negative > nodes {
                Err(error("terminal outside the prepared circuit"))
            } else {
                Ok(())
            }
        };
        for (index, stamp) in r.stamps.iter().enumerate() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            terminals(stamp.pp.row, stamp.nn.row)?;
        }
        for line in &circuit.tlines {
            check_abort(abort)?;
            if !line.supports_sided_history_events() || line.ltra_branch_matrix_indices().is_some()
            {
                return Err(error(format!(
                    "transmission line '{}' requires a prepared distributed or branch event sampler",
                    line.name
                )));
            }
            terminals(line.node1_pos, line.node1_neg)?;
            terminals(line.node2_pos, line.node2_neg)?;
        }
        for (index, stamp) in c.stamps.iter().enumerate() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            terminals(stamp.pp.row, stamp.nn.row)?;
            if c.capacitances[index] != 0.0 {
                ResourceLimitError::ensure(
                    ResourceKind::ResultValues,
                    size.saturating_mul(64)
                        .saturating_add(ports.len().saturating_add(1).saturating_mul(2)),
                    options.limits.max_result_values,
                )?;
                ports.push((stamp.pp.row, stamp.nn.row));
            }
        }
        for (index, &ordinal) in vs.branch_indices.iter().enumerate() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            terminals(vs.node_pos[index], vs.node_neg[index])?;
            claim(
                &mut equations,
                ordinal,
                EventBranchEquation::Algebraic(options.voltage_tolerance),
            )?;
        }
        for index in 0..is.len() {
            if index % 64 == 0 {
                check_abort(abort)?;
            }
            terminals(is.node_pos[index], is.node_neg[index])?;
        }
        for (magnetic, count) in [(false, rb.len()), (true, l.len())] {
            for index in 0..count {
                if index % 64 == 0 {
                    check_abort(abort)?;
                }
                let (p, n, ordinal, value) = if magnetic {
                    (
                        l.node_pos[index],
                        l.node_neg[index],
                        l.branch_indices[index],
                        l.inductances[index],
                    )
                } else {
                    (
                        rb.node_pos[index],
                        rb.node_neg[index],
                        rb.branch_indices[index],
                        rb.resistances[index],
                    )
                };
                terminals(p, n)?;
                let row = if magnetic && value != 0.0 {
                    EventBranchEquation::Flux {
                        flux_tolerance,
                        voltage_tolerance: options.voltage_tolerance,
                    }
                } else {
                    EventBranchEquation::Algebraic(options.voltage_tolerance)
                };
                claim(&mut equations, ordinal, row)?;
                if value == 0.0 {
                    constant_sources.push(EventVoltageSource {
                        positive: p,
                        negative: n,
                        branch: nodes + ordinal - 1,
                        value: 0.0,
                        slope: 0.0,
                    });
                }
            }
        }
        let mut models = Vec::with_capacity(circuit.bjts.len());
        for original in &circuit.bjts.devices {
            check_abort(abort)?;
            if !original.uses_legacy_gummel_poon() {
                return Err(error(format!(
                    "BJT '{}' requires a prepared VBIC/thermal event sampler",
                    original.name
                )));
            }
            original.validate_legacy_excess_phase().map_err(error)?;
            let mut model = original.clone();
            if !model.mna_promoted() {
                let mut missing = false;
                model.assign_mna_internal_nodes(|_| {
                    missing = true;
                    0
                });
                if missing {
                    return Err(error(format!(
                        "BJT '{}' has unprepared internal event nodes",
                        model.name
                    )));
                }
            }
            if model.mna_coupling_nodes().iter().any(|node| *node > nodes) {
                return Err(error(format!(
                    "BJT '{}' references missing event nodes",
                    model.name
                )));
            }
            if model.needs_mna_rbi_branch() {
                let row = model
                    .mna_rbi_branch_matrix_node(nodes)
                    .ok_or_else(|| error(format!("BJT '{}' has no RBI branch", model.name)))?;
                claim(
                    &mut equations,
                    row - nodes,
                    EventBranchEquation::Algebraic(options.voltage_tolerance),
                )?;
            }
            model.resolve_mna_rbi_branch(nodes);
            ResourceLimitError::ensure(
                ResourceKind::ResultValues,
                size.saturating_mul(64).saturating_add(
                    ports
                        .len()
                        .saturating_add(model.charge_storage_nodes().len())
                        .saturating_mul(2),
                ),
                options.limits.max_result_values,
            )?;
            ports.extend(model.charge_storage_nodes().into_iter().flatten());
            models.push(model);
        }
        coupling::validate(circuit, options, abort)?;
        let equations = equations
            .into_iter()
            .collect::<Option<Vec<_>>>()
            .ok_or_else(|| error("MNA branch has no physical event owner"))?;
        Ok(Self {
            circuit,
            forward_charge_limits: vec![false; models.len()],
            models,
            ports,
            equations,
            constant_sources,
        })
    }
}

//! Galerkin charge jumps and outgoing finite-current/rate reconstruction.
use super::*;

/// Structurally prepared event equations, in the original MNA row order.
/// The owner must certify that algebraic combinations have identically zero
/// storage and that ideal-source currents occur only in their KCL incidence.
/// Bias-dependent numerical rank is not a substitute for that certificate.
#[derive(Debug, Clone)]
pub enum SpectralEnvelopeEventEquation {
    /// Preserve this physical row's charge/flux, allowing declared voltage
    /// sources to supply a separate integrated-current impulse.
    Charge,
    /// A charge-free combination of physical F rows. Floating capacitive
    /// groups, for example, use their summed KCL as the algebraic constraint.
    Algebraic(Vec<(usize, Value)>),
    /// This row/coordinate is an ideal voltage-source branch. Terminals are
    /// zero-based physical voltage coordinates; None is ground.
    VoltageSource {
        positive: Option<usize>,
        negative: Option<usize>,
    },
}

#[derive(Debug, Clone)]
pub struct SpectralEnvelopeEventConfig {
    pub solver: QuasiPeriodicSolveConfig,
    /// Per physical row, in charge/flux units. Also audits storage equations
    /// replaced by algebraic combinations, so no conservation row disappears.
    pub charge_tolerances: Vec<Value>,
    /// Per reconstruction row, in its own rate-equation units. Differentiated
    /// algebraic constraints cannot borrow volt/amp floors through a made-up dt.
    pub rate_tolerances: Vec<Value>,
}

#[derive(Debug, Clone)]
pub struct SpectralEnvelopeEvent {
    pub state: SpectralEnvelopeState,
    /// Integrated source currents in coulombs, indexed by original MNA row.
    /// Empty rows are non-source coordinates. These are not finite currents.
    pub current_impulses: Vec<Vec<Complex64>>,
    /// Slow-time rates of non-source coordinates; source rows are empty.
    pub slow_rates: Vec<Vec<Complex64>>,
}

struct Topology<'a> {
    rows: &'a [SpectralEnvelopeEventEquation],
    source_columns: Vec<bool>,
    incidence: Vec<JacobianEntry>,
    algebraic_targets: Vec<Vec<(usize, Value)>>,
    limit: usize,
    abort: &'a dyn AbortSignal,
}

impl<'a> Topology<'a> {
    fn retained_values(&self) -> usize {
        self.rows
            .len()
            .saturating_mul(4)
            .saturating_add(
                self.algebraic_targets
                    .iter()
                    .map(|rows| rows.len())
                    .sum::<usize>()
                    .saturating_mul(2),
            )
            .saturating_add(self.incidence.len().saturating_mul(3))
    }
    fn new(
        rows: &'a [SpectralEnvelopeEventEquation],
        n: usize,
        limits: &ResourceLimits,
        abort: &'a dyn AbortSignal,
    ) -> Result<Self, Error> {
        if rows.len() != n {
            return Err(Error::InvalidCircuit(
                "event equations do not match the MNA basis".into(),
            ));
        }
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            n.saturating_mul(6),
            limits.max_result_values,
        )?;
        let mut source_columns = vec![false; n];
        let mut incidence = Vec::new();
        let mut algebraic_targets = vec![Vec::new(); n];
        let mut target_count = 0usize;
        for (row, equation) in rows.iter().enumerate() {
            check_abort(abort)?;
            match equation {
                SpectralEnvelopeEventEquation::Charge => {}
                SpectralEnvelopeEventEquation::Algebraic(terms) => {
                    if terms.is_empty()
                        || terms
                            .iter()
                            .any(|&(r, v)| r >= n || !v.is_finite() || v == 0.0)
                    {
                        return Err(Error::InvalidCircuit(
                            "invalid algebraic event combination".into(),
                        ));
                    }
                    for &(physical, weight) in terms {
                        if target_count.is_multiple_of(256) {
                            check_abort(abort)?;
                        }
                        target_count = target_count.saturating_add(1);
                        ResourceLimitError::ensure(
                            ResourceKind::ResultValues,
                            n.saturating_mul(6)
                                .saturating_add(target_count.saturating_mul(2)),
                            limits.max_result_values,
                        )?;
                        algebraic_targets[physical].push((row, weight));
                    }
                }
                SpectralEnvelopeEventEquation::VoltageSource { positive, negative } => {
                    if positive == negative
                        || positive.iter().chain(negative).any(|&r| r >= n || r == row)
                    {
                        return Err(Error::InvalidCircuit(
                            "invalid event voltage-source terminals".into(),
                        ));
                    }
                    source_columns[row] = true;
                    for (terminal, sign) in [(positive, 1.0), (negative, -1.0)] {
                        if let Some(node) = terminal {
                            incidence.push((*node, row, sign));
                        }
                    }
                }
            }
        }
        if incidence.iter().any(|&(r, _, _)| source_columns[r]) {
            return Err(Error::InvalidCircuit(
                "event voltage terminal names a source-current coordinate".into(),
            ));
        }
        Ok(Self {
            rows,
            source_columns,
            incidence,
            algebraic_targets,
            limit: limits.max_result_values,
            abort,
        })
    }

    fn physical_state(&self, state: &[Value]) -> Vec<Value> {
        state
            .iter()
            .zip(&self.source_columns)
            .map(|(&v, &source)| if source { 0.0 } else { v })
            .collect()
    }

    fn validate_sample(&self, sample: &Sample) -> Result<(), Error> {
        if sample
            .conductance
            .iter()
            .chain(&sample.capacitance)
            .any(|&(r, c, v)| {
                r >= self.rows.len() || c >= self.rows.len() || (self.source_columns[c] && v != 0.0)
            })
        {
            return Err(Error::InvalidCircuit(
                "event F/Q depends on an ideal-source current".into(),
            ));
        }
        if sample
            .current
            .iter()
            .chain(&sample.charge)
            .any(|&(r, _)| r >= self.rows.len())
        {
            return Err(Error::InvalidCircuit(
                "event F/Q row is outside the MNA basis".into(),
            ));
        }
        Ok(())
    }

    fn check_output(&self, terms: usize, words: usize) -> Result<(), Error> {
        if terms.is_multiple_of(256) {
            check_abort(self.abort)?;
        }
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            terms.saturating_add(1).saturating_mul(words),
            self.limit,
        )?;
        Ok(())
    }

    fn map_terms(
        &self,
        terms: &[(usize, Value)],
        charge: bool,
        scales: &[Value],
    ) -> Result<Vec<(usize, Value)>, Error> {
        let mut result = Vec::new();
        for (index, &(physical, value)) in terms.iter().enumerate() {
            if index.is_multiple_of(256) {
                check_abort(self.abort)?;
            }
            if charge {
                if matches!(self.rows[physical], SpectralEnvelopeEventEquation::Charge) {
                    self.check_output(result.len(), 2)?;
                    result.push((physical, value / scales[physical]));
                }
            } else {
                for &(row, weight) in &self.algebraic_targets[physical] {
                    self.check_output(result.len(), 2)?;
                    result.push((row, weight * (value / scales[row])));
                }
            }
        }
        Ok(result)
    }

    fn map_matrix(
        &self,
        entries: &[JacobianEntry],
        charge: bool,
        scales: &[Value],
    ) -> Result<Vec<JacobianEntry>, Error> {
        let mut result = Vec::new();
        for (index, &(physical, column, value)) in entries.iter().enumerate() {
            if index.is_multiple_of(256) {
                check_abort(self.abort)?;
            }
            if charge {
                if matches!(self.rows[physical], SpectralEnvelopeEventEquation::Charge) {
                    self.check_output(result.len(), 3)?;
                    result.push((physical, column, value / scales[physical]));
                }
            } else {
                for &(row, weight) in &self.algebraic_targets[physical] {
                    self.check_output(result.len(), 3)?;
                    result.push((row, column, weight * (value / scales[row])));
                }
            }
        }
        Ok(result)
    }

    fn source_entries(&self, scales: &[Value]) -> Vec<JacobianEntry> {
        let mut result = self
            .incidence
            .iter()
            .filter(|&&(r, _, _)| matches!(self.rows[r], SpectralEnvelopeEventEquation::Charge))
            .map(|&(r, c, v)| (r, c, v / scales[r]))
            .collect::<Vec<_>>();
        for (row, equation) in self.rows.iter().enumerate() {
            if let SpectralEnvelopeEventEquation::VoltageSource { positive, negative } = equation {
                for (terminal, sign) in [(positive, 1.0), (negative, -1.0)] {
                    if let Some(c) = terminal {
                        result.push((row, *c, sign / scales[row]));
                    }
                }
            }
        }
        result
    }
}

struct Jump<'a, C> {
    physical: &'a mut C,
    topology: &'a Topology<'a>,
    scales: &'a [Value],
    linear: Vec<JacobianEntry>,
}

impl<C: Circuit> Circuit for Jump<'_, C> {
    fn unknowns(&self) -> usize {
        self.topology.rows.len()
    }
    fn voltage_equation(&self, _: usize) -> bool {
        false
    }
    fn linear_entries(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
        Ok(self
            .linear
            .iter()
            .map(|&(r, c, v)| (r, c, Complex64::new(v, 0.0)))
            .collect())
    }
    fn sample(&mut self, state: &[Value], jacobian: bool) -> Result<Sample, Error> {
        self.sample_at_phases(state, &[], jacobian)
    }
    fn sample_at_phases(
        &mut self,
        state: &[Value],
        phases: &[Value],
        _: bool,
    ) -> Result<Sample, Error> {
        let sample =
            self.physical
                .sample_at_phases(&self.topology.physical_state(state), phases, true)?;
        self.topology.validate_sample(&sample)?;
        let mut current = self.topology.map_terms(&sample.charge, true, self.scales)?;
        current.extend(
            self.topology
                .map_terms(&sample.current, false, self.scales)?,
        );
        let mut conductance = self
            .topology
            .map_matrix(&sample.capacitance, true, self.scales)?;
        conductance.extend(
            self.topology
                .map_matrix(&sample.conductance, false, self.scales)?,
        );
        Ok(Sample {
            current,
            conductance,
            ..Default::default()
        })
    }
}

struct Rate<'a> {
    topology: &'a Topology<'a>,
    grid: &'a QuasiPeriodicGrid,
    frames: Vec<Sample>,
    scales: &'a [Value],
    linear: Vec<JacobianEntry>,
    audit_rows: Option<&'a [bool]>,
}

impl Circuit for Rate<'_> {
    fn unknowns(&self) -> usize {
        self.topology.rows.len()
    }
    fn voltage_equation(&self, row: usize) -> bool {
        self.audit_rows.is_some_and(|rows| rows[row])
    }
    fn linear_entries(&self, _: Value) -> Result<Vec<LinearEntry>, Error> {
        Ok(self
            .linear
            .iter()
            .map(|&(r, c, v)| (r, c, Complex64::new(v, 0.0)))
            .collect())
    }
    fn sample(&mut self, state: &[Value], jacobian: bool) -> Result<Sample, Error> {
        self.sample_at_phases(state, &[], jacobian)
    }
    fn sample_at_phases(
        &mut self,
        rates: &[Value],
        phases: &[Value],
        _: bool,
    ) -> Result<Sample, Error> {
        // The Galerkin evaluator supplies exact grid phases. Invert that
        // mapping and verify it, without approximate phase interpolation.
        let mut index = 0;
        let mut stride = 1;
        if phases.len() != self.grid.dimensions().len() {
            return Err(Error::InvalidCircuit(
                "event rate phase dimensions changed".into(),
            ));
        }
        for (&phase, &size) in phases.iter().zip(self.grid.dimensions()) {
            if !phase.is_finite() || phase < 0.0 {
                return Err(Error::InvalidCircuit("invalid event rate phase".into()));
            }
            let coordinate = (phase * size as Value / std::f64::consts::TAU).round() as usize;
            if coordinate >= size {
                return Err(Error::InvalidCircuit(
                    "event rate phase outside grid".into(),
                ));
            }
            index += coordinate * stride;
            stride *= size;
        }
        if self.grid.phases(index).as_deref() != Some(phases) {
            return Err(Error::InvalidCircuit(
                "event rate phase is not a prepared collocation point".into(),
            ));
        }
        let frame = &self.frames[index];
        let conductance = if self.audit_rows.is_some() {
            frame.capacitance.clone()
        } else {
            let mut entries = self
                .topology
                .map_matrix(&frame.capacitance, true, self.scales)?;
            entries.extend(
                self.topology
                    .map_matrix(&frame.conductance, false, self.scales)?,
            );
            entries
        };
        // A nonlinear Jacobian's sparse pattern may vary across phases. Keep
        // one delivered term per physical row so the transform term identity
        // is stable even when a derivative crosses an exact zero.
        let mut products = vec![0.0; self.unknowns()];
        for &(r, c, v) in &conductance {
            products[r] -= v * rates[c];
        }
        let mut current = products.into_iter().enumerate().collect::<Vec<_>>();
        // On storage rows the physical F terms are known at the outgoing
        // state. On algebraic rows it is their differentiated constraint.
        if self.audit_rows.is_some() {
            current.extend_from_slice(&frame.current);
        } else {
            current.extend(self.topology.map_terms(&frame.current, true, self.scales)?);
        }
        Ok(Sample {
            current,
            conductance,
            ..Default::default()
        })
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "physical history, structural equations, sided drives, numerical policy and resources are independent"
)]
pub(crate) fn transition(
    circuit: &mut impl Circuit,
    previous: &SpectralEnvelopeState,
    rows: &[SpectralEnvelopeEventEquation],
    config: &SpectralEnvelopeEventConfig,
    sources: &[Vec<Complex64>],
    slow_source_rates: &[Vec<Complex64>],
    storage: &[JacobianEntry],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
) -> Result<SpectralEnvelopeEvent, Error> {
    check_abort(abort)?;
    config.solver.validate()?;
    let n = circuit.unknowns();
    if n != previous.spectra().len()
        || previous
            .voltage_rows
            .iter()
            .enumerate()
            .any(|(r, &v)| v != circuit.voltage_equation(r))
        || config.charge_tolerances.len() != n
        || config.rate_tolerances.len() != n
        || config
            .charge_tolerances
            .iter()
            .chain(&config.rate_tolerances)
            .any(|v| !v.is_finite() || *v <= 0.0)
    {
        return Err(Error::InvalidConfig(
            "event state and positive physical tolerances must cover every MNA row".into(),
        ));
    }
    let topology = Topology::new(rows, n, limits, abort)?;
    storage_valid(storage, n)?;
    if storage
        .iter()
        .any(|&(_, c, v)| topology.source_columns[c] && v != 0.0)
    {
        return Err(Error::InvalidCircuit(
            "event storage depends on an ideal-source current".into(),
        ));
    }
    let grid = previous.grid();
    validate_spectra(sources, n, grid, "outgoing event sources", abort)?;
    validate_spectra(
        slow_source_rates,
        n,
        grid,
        "outgoing slow source derivatives",
        abort,
    )?;
    // Retained history, both projected solutions, source/rate spectra, and
    // phase samples coexist with the generic Galerkin solver workspace.
    let retained = n
        .saturating_mul(
            grid.len()
                .saturating_mul(48)
                .saturating_add(grid.sample_count().saturating_mul(12)),
        )
        .saturating_add(storage.len().saturating_mul(6))
        .saturating_add(topology.retained_values());
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        retained,
        limits.max_result_values,
    )?;
    let mut bounded = *limits;
    bounded.max_result_values -= retained;
    check_workload(n, grid, &config.solver.linear, &bounded)?;
    let mut linear = Vec::new();
    let mut incidence = std::collections::BTreeMap::new();
    for (r, c, v) in circuit.linear_entries(0.0)? {
        if r >= n || c >= n || !finite(v) || v.im != 0.0 {
            return Err(Error::InvalidCircuit(
                "event requires finite real static F coefficients".into(),
            ));
        }
        if topology.source_columns[c] {
            *incidence.entry((r, c)).or_insert(0.0) += v.re;
        } else {
            linear.push((r, c, v.re));
        }
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            linear
                .len()
                .saturating_mul(3)
                .saturating_add(incidence.len().saturating_mul(4)),
            bounded.max_result_values,
        )?;
    }
    for &(r, c, v) in &topology.incidence {
        *incidence.entry((r, c)).or_insert(0.0) -= v;
    }
    if incidence.values().any(|v| *v != 0.0) {
        return Err(Error::InvalidCircuit(
            "ideal-source impulse path differs from declared KCL incidence".into(),
        ));
    }
    drop(incidence);
    let jump_scales = rows
        .iter()
        .enumerate()
        .map(|(r, kind)| match kind {
            SpectralEnvelopeEventEquation::Charge => config.charge_tolerances[r],
            _ if circuit.voltage_equation(r) => config.solver.voltage_absolute_tolerance,
            _ => config.solver.current_absolute_tolerance,
        })
        .collect::<Vec<_>>();
    let mut jump_linear = topology.map_matrix(storage, true, &jump_scales)?;
    jump_linear.extend(topology.map_matrix(&linear, false, &jump_scales)?);
    jump_linear.extend(topology.source_entries(&jump_scales));
    let mut rhs = vec![vec![Complex64::ZERO; grid.len()]; n];
    for (r, equation) in rows.iter().enumerate() {
        check_abort(abort)?;
        match equation {
            SpectralEnvelopeEventEquation::Charge => {
                for (b, q) in rhs[r].iter_mut().zip(&previous.charge[r]) {
                    *b = -*q / jump_scales[r];
                }
            }
            SpectralEnvelopeEventEquation::Algebraic(weights) => {
                for &(physical, weight) in weights {
                    for (b, s) in rhs[r].iter_mut().zip(&sources[physical]) {
                        *b += weight * (*s / jump_scales[r]);
                    }
                }
            }
            SpectralEnvelopeEventEquation::VoltageSource { .. } => {
                for (b, s) in rhs[r].iter_mut().zip(&sources[r]) {
                    *b = *s / jump_scales[r];
                }
            }
        }
    }
    let mut settings = config.solver.clone();
    settings.current_absolute_tolerance = 1.0;
    settings.voltage_absolute_tolerance = 1.0;
    let mut guess = previous.spectra().to_vec();
    for (row, source) in guess.iter_mut().zip(&topology.source_columns) {
        if *source {
            row.fill(Complex64::ZERO);
        }
    }
    let projected = solve_with_abort(
        &mut Jump {
            physical: circuit,
            topology: &topology,
            scales: &jump_scales,
            linear: jump_linear,
        },
        grid.clone(),
        &settings,
        &rhs,
        Some(&guess),
        &bounded,
        abort,
    )?;
    let mut outgoing = projected.spectra().to_vec();
    let impulses = outgoing
        .iter_mut()
        .zip(&topology.source_columns)
        .map(|(row, &source)| {
            if source {
                std::mem::replace(row, vec![Complex64::ZERO; grid.len()])
            } else {
                Vec::new()
            }
        })
        .collect::<Vec<_>>();
    let charge = charge_spectra(circuit, grid.clone(), &outgoing, storage, abort)?;
    for r in 0..n {
        check_abort(abort)?;
        for k in 0..grid.len() {
            let mut residual = previous.charge[r][k] - charge[r][k];
            let mut scale = previous.charge[r][k].norm() + charge[r][k].norm();
            for &(row, col, value) in &topology.incidence {
                if row == r {
                    residual += value * impulses[col][k];
                    scale += (value * impulses[col][k]).norm();
                }
            }
            let tolerance = config.charge_tolerances[r] + config.solver.relative_tolerance * scale;
            if !finite(residual)
                || !scale.is_finite()
                || !tolerance.is_finite()
                || residual.norm() > tolerance
            {
                return Err(Error::InvalidCircuit(format!(
                    "event charge/flux conservation failed at row {r}, tuple {k}"
                )));
            }
        }
    }
    let mut transform = QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?;
    let waves = outgoing
        .iter()
        .map(|row| transform.to_real_samples_with_abort(row, abort))
        .collect::<Result<Vec<_>, _>>()?;
    let mut frames = Vec::with_capacity(grid.sample_count());
    let mut frame_values = 0usize;
    for t in 0..grid.sample_count() {
        check_abort(abort)?;
        let state = waves.iter().map(|row| row[t]).collect::<Vec<_>>();
        let mut frame =
            circuit.sample_at_phases(&state, &grid.phases(t).expect("bounded sample"), true)?;
        topology.validate_sample(&frame)?;
        frame
            .current
            .extend(linear.iter().map(|&(r, c, v)| (r, -v * state[c])));
        frame.conductance.extend_from_slice(&linear);
        frame.capacitance.extend_from_slice(storage);
        frame.charge = Vec::new();
        frame_values = frame_values
            .saturating_add(frame.current.len().saturating_mul(2))
            .saturating_add(
                frame
                    .conductance
                    .len()
                    .saturating_add(frame.capacitance.len())
                    .saturating_mul(3),
            );
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            frame_values,
            bounded.max_result_values,
        )?;
        frames.push(frame);
    }
    bounded.max_result_values -= frame_values;
    check_workload(n, grid, &config.solver.linear, &bounded)?;
    for (r, equation) in rows.iter().enumerate() {
        rhs[r].fill(Complex64::ZERO);
        match equation {
            SpectralEnvelopeEventEquation::Charge => {
                let fast = grid.differentiate_with_abort(&charge[r], abort)?;
                for ((b, s), q) in rhs[r].iter_mut().zip(&sources[r]).zip(fast) {
                    *b = (*s + q) / config.rate_tolerances[r];
                }
            }
            SpectralEnvelopeEventEquation::Algebraic(weights) => {
                for &(physical, weight) in weights {
                    for (b, s) in rhs[r].iter_mut().zip(&slow_source_rates[physical]) {
                        *b += weight * (*s / config.rate_tolerances[r]);
                    }
                }
            }
            SpectralEnvelopeEventEquation::VoltageSource { .. } => {
                for (b, s) in rhs[r].iter_mut().zip(&slow_source_rates[r]) {
                    *b = *s / config.rate_tolerances[r];
                }
            }
        }
    }
    let mut rate_circuit = Rate {
        topology: &topology,
        grid,
        frames,
        scales: &config.rate_tolerances,
        linear: topology.source_entries(&config.rate_tolerances),
        audit_rows: None,
    };
    let rates = solve_with_abort(
        &mut rate_circuit,
        grid.clone(),
        &settings,
        &rhs,
        None,
        &bounded,
        abort,
    )?;
    // Recheck every original physical equation, including the KCL rows
    // replaced by differentiated algebraic constraints in the square solve.
    // This uses the same per-coefficient contribution scaling as Newton.
    rate_circuit.audit_rows = Some(&previous.voltage_rows);
    rate_circuit.linear = topology.incidence.clone();
    for r in 0..n {
        let fast = grid.differentiate_with_abort(&charge[r], abort)?;
        for ((b, s), q) in rhs[r].iter_mut().zip(&sources[r]).zip(fast) {
            *b = *s + q;
        }
    }
    let (base_values, value_limit) = check_workload(n, grid, &config.solver.linear, &bounded)?;
    let mut audit = Workspace {
        grid: grid.clone(),
        transform: QuasiPeriodicTransform::new_with_abort(grid.clone(), abort)?,
        config: &config.solver,
        unknowns: n,
        voltage_rows: previous.voltage_rows.clone(),
        base_values,
        value_limit,
        linear: Vec::new(),
    };
    for &frequency in grid.frequencies_hz() {
        let entries = rate_circuit.linear_entries(frequency)?;
        audit.base_values = audit
            .base_values
            .saturating_add(entries.len().saturating_mul(4));
        audit.budget(0)?;
        audit.linear.push(entries);
    }
    let certificate = audit
        .evaluate(&mut rate_circuit, rates.spectra(), &rhs, false, abort)?
        .merit;
    if certificate > 1.0 {
        return Err(Error::InvalidCircuit(format!(
            "event finite-current equations failed (normalized residual {certificate:e})"
        )));
    }
    let slow_rates = rates
        .spectra()
        .iter()
        .zip(&topology.source_columns)
        .enumerate()
        .map(|(r, (row, &source))| {
            if source {
                outgoing[r] = row.clone();
                Vec::new()
            } else {
                row.clone()
            }
        })
        .collect();
    check_abort(abort)?;
    Ok(SpectralEnvelopeEvent {
        state: SpectralEnvelopeState {
            time: previous.time,
            solution: QuasiPeriodicSolution {
                grid: grid.clone(),
                spectra: outgoing,
                iterations: projected.iterations() + rates.iterations(),
                normalized_residual: projected
                    .normalized_residual()
                    .max(rates.normalized_residual())
                    .max(certificate),
            },
            charge,
            older: None,
            voltage_rows: previous.voltage_rows.clone(),
            order: 0,
        },
        current_impulses: impulses,
        slow_rates,
    })
}

//! Resource, identity and reconstruction checks for retained noise evidence.
use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicNoiseProjector;
use std::{collections::BTreeSet, io::Write};

pub(super) fn retained_values(
    m: &QpnoiseResultMetadata,
    sources: &[QuasiPeriodicNoiseSource],
    grid: &QuasiPeriodicGrid,
) -> usize {
    let points = m.request.frequencies_hz.len();
    let outputs = m.request.outputs.len();
    let rows = m.node_names.len().saturating_add(m.branch_names.len());
    let dimensions = grid.dimensions().len();
    let adjoints = outputs
        .saturating_mul(rows)
        .saturating_mul(grid.len())
        .saturating_mul(2);
    let covariance = outputs
        .saturating_mul(outputs)
        .saturating_mul(3)
        .saturating_mul(sources.len().saturating_add(2));
    // Both canonical derived data and its validation reconstruction are live.
    let mut values = points
        .saturating_mul(
            adjoints
                .saturating_add(covariance)
                .saturating_add(outputs.saturating_mul(24)),
        )
        .saturating_add(outputs.saturating_mul(sources.len()).saturating_mul(10));
    values = values
        .saturating_add(m.input_lattices.len().saturating_mul(dimensions))
        .saturating_add(outputs.saturating_mul(dimensions + 8));
    for source in sources {
        values = values
            .saturating_add(source.injections.len().saturating_mul(3))
            .saturating_add(match &source.spectrum {
                QuasiPeriodicNoiseSpectrum::White { density, .. } => density.len(),
                QuasiPeriodicNoiseSpectrum::PowerLaw {
                    modulation,
                    modulation_lattices,
                    ..
                } => modulation.len().saturating_mul(2).saturating_add(
                    modulation_lattices.as_ref().map_or(0, |ts| {
                        ts.iter().fold(0usize, |n, t| n.saturating_add(t.len()))
                    }),
                ),
            });
    }
    values
}
impl QpnoiseAnalysisResult {
    pub(super) fn payload_identity(
        &self,
        abort: &dyn AbortSignal,
    ) -> Result<String, SimulationError> {
        struct Sink<'a> {
            hash: blake3::Hasher,
            abort: &'a dyn AbortSignal,
        }
        impl Write for Sink<'_> {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if self.abort.is_aborted() {
                    return Err(std::io::Error::other("aborted"));
                }
                self.hash.update(bytes);
                Ok(bytes.len())
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut sink = Sink {
            hash: blake3::Hasher::new(),
            abort,
        };
        sink.hash.update(b"rspice-qpnoise-complete-response-v1\0");
        let mut metadata = self.metadata.clone();
        metadata.retained_identity.clear();
        let serialized =
            serde_json::to_writer(&mut sink, &(&metadata, &self.sources, &self.points));
        check_abort(abort)?;
        serialized.map_err(|e| qpnoise_error(e.to_string()))?;
        Ok(sink.hash.finalize().to_hex().to_string())
    }
    pub fn validate_retained_payload_with_abort(
        &self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        let m = &self.metadata;
        m.request.validate()?;
        let outputs = m.request.outputs.len();
        let count = m.request.frequencies_hz.len();
        let rows = m.node_names.len().saturating_add(m.branch_names.len());
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            count
                .saturating_mul(outputs)
                .saturating_mul(self.sources.len().max(1)),
            limits.max_analysis_points,
        )?;
        if rows == 0
            || rows > 65_536
            || self.points.len() != count
            || self.outputs.len() != outputs
            || self.total_covariances.len() != count
            || m.observations.len() != outputs
            || m.version != 1
            || !is_canonical_blake3_identity(&m.retained_identity)
            || !is_canonical_blake3_identity(&m.operating_point_identity)
        {
            return Err(qpnoise_error(
                "retained result dimensions, version or identities are invalid",
            ));
        }
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(m.grid.clone(), limits, abort)
                .map_err(numerical_error)?,
        );
        let retained = retained_values(m, &self.sources, &grid);
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            retained,
            limits.max_result_values.min(32_000_000),
        )?;
        for names in [&m.node_names, &m.branch_names] {
            let mut seen = BTreeSet::new();
            for name in names {
                check_abort(abort)?;
                if name.trim().is_empty()
                    || name.contains(['\r', '\n'])
                    || !seen.insert(name.trim().to_ascii_lowercase())
                {
                    return Err(qpnoise_error("retained MNA names are empty or duplicated"));
                }
            }
        }
        if m.input_lattices != m.request.input_lattices.resolve(&grid)? {
            return Err(qpnoise_error(
                "retained noise-input lattice differs from the request",
            ));
        }
        for (output, observation) in m.request.outputs.iter().zip(&m.observations) {
            if grid.index_of(&output.lattice).is_none()
                || *observation
                    != bindings::output_from_names(
                        m.ground_policy,
                        &m.node_names,
                        &m.branch_names,
                        &output.observation,
                    )?
            {
                return Err(qpnoise_error(
                    "retained output binding differs from the request",
                ));
            }
        }
        validate_input(m, &grid)?;
        validate_reference(m, &self.sources)?;
        let anchor = m.request.frequency_anchor();
        for (i, point) in self.points.iter().enumerate() {
            check_abort(abort)?;
            if point.adjoints.len() != outputs
                || point.source_covariances.len() != self.sources.len()
            {
                return Err(qpnoise_error(
                    "retained point has inconsistent output or source counts",
                ));
            }
            for adjoint in &point.adjoints {
                if adjoint.frequency_hz != m.request.frequencies_hz[i]
                    || adjoint.frequency_lattice != anchor
                    || !adjoint.normalized_residual.is_finite()
                    || !(0.0..=1.0).contains(&adjoint.normalized_residual)
                    || adjoint.sensitivities.len() != rows
                {
                    return Err(qpnoise_error(
                        "retained adjoint certificate or frequency anchor is invalid",
                    ));
                }
                for row in &adjoint.sensitivities {
                    if row.len() != grid.len() {
                        return Err(qpnoise_error(
                            "retained adjoint has incomplete tone coordinates",
                        ));
                    }
                    for (j, value) in row.iter().enumerate() {
                        if j.is_multiple_of(256) {
                            check_abort(abort)?;
                        }
                        if !value.re.is_finite() || !value.im.is_finite() {
                            return Err(qpnoise_error(
                                "retained adjoint has nonfinite coefficients",
                            ));
                        }
                    }
                }
            }
            for covariance in &point.source_covariances {
                validate_covariance(covariance, outputs, abort)?;
            }
        }
        let mut names = BTreeSet::new();
        let mut workspace = limits.clone();
        workspace.max_result_values = workspace
            .max_result_values
            .min(32_000_000)
            .saturating_sub(retained);
        if !self.sources.is_empty() {
            let projector = QuasiPeriodicNoiseProjector::new_with_abort(
                grid.clone(),
                rows,
                &m.input_lattices,
                &workspace,
                abort,
            )
            .map_err(numerical_error)?;
            for source in &self.sources {
                check_abort(abort)?;
                if source.name.contains(['\r', '\n'])
                    || !names.insert(source.name.trim().to_ascii_lowercase())
                {
                    return Err(qpnoise_error(
                        "retained source labels are duplicated or invalid",
                    ));
                }
                projector
                    .validate(&self.points[0].adjoints, source, abort)
                    .map_err(numerical_error)?;
            }
        }
        match &m.request.sources {
            QpnoiseSources::All => {}
            QpnoiseSources::Only(selected) => {
                if selected.len() != names.len()
                    || selected
                        .iter()
                        .any(|s| !names.contains(&s.trim().to_ascii_lowercase()))
                {
                    return Err(qpnoise_error(
                        "retained mechanisms differ from included selection",
                    ));
                }
            }
            QpnoiseSources::Except(excluded) => {
                if excluded
                    .iter()
                    .any(|s| names.contains(&s.trim().to_ascii_lowercase()))
                {
                    return Err(qpnoise_error(
                        "retained mechanisms contain an excluded source",
                    ));
                }
            }
        }
        if m.retained_identity != self.payload_identity(abort)? {
            return Err(qpnoise_error(
                "retained primary noise evidence is incompatible or altered",
            ));
        }
        let (totals, outputs) = derived::reconstruct(m, &self.points, &grid, abort)?;
        if totals != self.total_covariances || outputs != self.outputs {
            return Err(qpnoise_error(
                "retained measurements differ from complete noise evidence",
            ));
        }
        Ok(grid)
    }
}
fn validate_input(
    m: &QpnoiseResultMetadata,
    grid: &QuasiPeriodicGrid,
) -> Result<(), SimulationError> {
    match (&m.request.input, &m.input_source) {
        (None, None) => Ok(()),
        (Some(request), Some(source)) => {
            let nodes = m.node_names.len();
            let valid = source.name.eq_ignore_ascii_case(request.source.trim())
                && grid.index_of(&request.lattice).is_some()
                && match source.quantity {
                    QpnoiseQuantity::Voltage => {
                        source.injections.len() == 1
                            && source.injections[0].1 == Complex64::ONE
                            && source.injections[0]
                                .0
                                .checked_sub(nodes)
                                .and_then(|i| m.branch_names.get(i))
                                .is_some_and(|n| n.eq_ignore_ascii_case(&source.name))
                    }
                    QpnoiseQuantity::Current => {
                        (1..=2).contains(&source.injections.len())
                            && source.injections.iter().all(|(row, b)| {
                                *row < nodes && (*b == Complex64::ONE || *b == -Complex64::ONE)
                            })
                            && (source.injections.len() == 1
                                || (source.injections[0].0 != source.injections[1].0
                                    && source.injections[0].1 == -source.injections[1].1))
                    }
                };
            if !valid {
                return Err(qpnoise_error(
                    "retained input source or direction is invalid",
                ));
            }
            Ok(())
        }
        _ => Err(qpnoise_error(
            "retained input source differs from referral request",
        )),
    }
}
fn validate_reference(
    m: &QpnoiseResultMetadata,
    sources: &[QuasiPeriodicNoiseSource],
) -> Result<(), SimulationError> {
    match (&m.request.noise_figure, &m.reference) {
        (None, None) => Ok(()),
        (Some(request), Some(reference)) => {
            let input = m.request.input.as_ref().expect("validated figure input");
            let tuples = request
                .reference_lattices
                .clone()
                .unwrap_or_else(|| vec![input.lattice.clone()]);
            if !reference
                .source_resistor
                .eq_ignore_ascii_case(request.source_resistor.trim())
                || !sources.get(reference.source_index).is_some_and(|s| {
                    s.name
                        .eq_ignore_ascii_case(&format!("{} thermal", reference.source_resistor))
                })
                || [
                    reference.resistance,
                    reference.temperature,
                    reference.boltzmann,
                ]
                .iter()
                .any(|v| !v.is_finite() || *v <= 0.0)
                || reference.lattices != tuples
                || tuples.iter().any(|t| !m.input_lattices.contains(t))
                || m.input_source
                    .as_ref()
                    .is_none_or(|s| s.quantity != QpnoiseQuantity::Voltage)
            {
                return Err(qpnoise_error(
                    "retained noise-figure physical reference differs from the request",
                ));
            }
            Ok(())
        }
        _ => Err(qpnoise_error(
            "retained noise-figure reference is missing or unsolicited",
        )),
    }
}
fn validate_covariance(
    c: &QuasiPeriodicNoiseCovariance,
    outputs: usize,
    abort: &dyn AbortSignal,
) -> Result<(), SimulationError> {
    let count = outputs.saturating_mul(outputs);
    if c.outputs != outputs || c.values.len() != count || c.roundoff_bounds.len() != count {
        return Err(qpnoise_error(
            "retained source covariance has invalid dimensions",
        ));
    }
    for row in 0..outputs {
        for column in 0..outputs {
            if (row * outputs + column).is_multiple_of(256) {
                check_abort(abort)?;
            }
            let index = row * outputs + column;
            let transpose = column * outputs + row;
            let value = c.values[index];
            let bound = c.roundoff_bounds[index];
            if !value.re.is_finite()
                || !value.im.is_finite()
                || !bound.is_finite()
                || bound < 0.0
                || value != c.values[transpose].conj()
                || bound != c.roundoff_bounds[transpose]
                || (row == column && (value.im != 0.0 || value.re < 0.0))
            {
                return Err(qpnoise_error(
                    "retained source covariance is not finite Hermitian with nonnegative diagonal and bounds",
                ));
            }
        }
    }
    Ok(())
}

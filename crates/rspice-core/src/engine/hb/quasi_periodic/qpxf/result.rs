//! Versioned complete QPXF evidence with bounded worker and saved-result reconstruction.
use super::*;
use crate::netlist::GroundPolicy;
use crate::{ResourceKind, ResourceLimitError, ResourceLimits};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpxfInputSource {
    pub name: String,
    pub quantity: QpxfQuantity,
    /// Complete unit excitation direction in canonical MNA coordinates.
    pub injections: Vec<(usize, Complex64)>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpxfTransfer {
    pub input_source: usize,
    pub input_lattice: Vec<i32>,
    pub input_frequencies_hz: Vec<Value>,
    /// Output quantity per unit input quantity, with no AC source scaling.
    pub values: Vec<Complex64>,
    /// Sampled phase derivative on the physical output-frequency grid.
    pub group_delay: Option<Vec<QpxfGroupDelay>>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpxfResultMetadata {
    pub(super) version: u32,
    pub request: QpxfRequest,
    pub operating_point_identity: String,
    pub grid: QuasiPeriodicGridConfig,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub ground_policy: GroundPolicy,
    /// Rounded origin offsets, for inspection only. The exact authored axis
    /// and its tuple anchor govern all translated-frequency calculations.
    pub probe_offsets_hz: Vec<Value>,
    pub output_frequencies_hz: Vec<Value>,
    pub input_sources: Vec<QpxfInputSource>,
    pub input_lattices: Vec<Vec<i32>>,
    pub normalized_residuals: Vec<Value>,
    pub(super) observation: Vec<(usize, Complex64)>,
    pub(super) retained_identity: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpxfAnalysisResult {
    pub metadata: QpxfResultMetadata,
    /// Complete adjoint sensitivity, sweep-major then MNA coordinate and
    /// canonical signed tuple. The residual is algebraic, not KCL/KVL.
    pub solutions: Vec<QuasiPeriodicAdjointSolution>,
    /// Source-major, then authored input-tuple order.
    pub transfers: Vec<QpxfTransfer>,
}

impl QpxfResultMetadata {
    pub fn retained_identity(&self) -> &str {
        &self.retained_identity
    }

    pub fn frequency_anchor(&self) -> Vec<i32> {
        match self.request.frequency_axis {
            QpxfFrequencyAxis::Output => self.request.output_lattice.clone(),
            QpxfFrequencyAxis::Offset => vec![0; self.request.output_lattice.len()],
        }
    }

    /// Validate all metadata and declared complex buffer lengths before copying
    /// data. Rows are sweep-major MNA sensitivities, followed by source-major
    /// input-tuple transfers. No frequency folding or scalar-offset recovery.
    pub fn validate_transfer_layout_with_abort(
        &self,
        lengths: &[usize],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        self.request.validate()?;
        let points = self.request.frequencies_hz.len();
        ResourceLimitError::ensure(
            ResourceKind::AnalysisPoints,
            points,
            limits.max_analysis_points,
        )?;
        let rows = self
            .node_names
            .len()
            .saturating_add(self.branch_names.len());
        let spectra = points.saturating_mul(rows);
        let paths = self
            .input_sources
            .len()
            .saturating_mul(self.input_lattices.len());
        let scalar_values = lengths
            .iter()
            .fold(0usize, |n, len| n.saturating_add(len.saturating_mul(2)))
            .saturating_add(
                points.saturating_mul(
                    paths
                        .saturating_mul(if self.request.group_delay { 3 } else { 1 })
                        .saturating_add(4),
                ),
            )
            .saturating_add(self.input_sources.len().saturating_mul(8))
            .saturating_add(
                self.input_lattices
                    .len()
                    .saturating_mul(self.grid.frequencies_hz.len()),
            );
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            scalar_values,
            limits.max_result_values,
        )?;
        if rows == 0
            || rows > 65_536
            || paths == 0
            || lengths.len() != spectra.saturating_add(paths)
        {
            return Err(qpxf_error(
                "retained sweep, source, tuple or MNA dimensions are invalid",
            ));
        }
        if self.version != 1
            || !is_canonical_blake3_identity(&self.operating_point_identity)
            || !is_canonical_blake3_identity(&self.retained_identity)
            || self.normalized_residuals.len() != points
            || self
                .normalized_residuals
                .iter()
                .any(|r| !r.is_finite() || !(0.0..=1.0).contains(r))
            || self.probe_offsets_hz.len() != points
            || self.output_frequencies_hz.len() != points
        {
            return Err(qpxf_error(
                "retained result version, identity or residual evidence is invalid",
            ));
        }
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(self.grid.clone(), limits, abort)
                .map_err(numerical_error)?,
        );
        if grid.index_of(&self.request.output_lattice).is_none()
            || lengths[..spectra].iter().any(|n| *n != grid.len())
            || lengths[spectra..].iter().any(|n| *n != points)
            || self.input_lattices != self.request.input_lattices.resolve(&grid)?
        {
            return Err(qpxf_error(
                "retained buffer lengths or selected tuples differ from the request",
            ));
        }
        for names in [&self.node_names, &self.branch_names] {
            let mut seen = BTreeSet::new();
            if names.iter().any(|n| {
                n.trim().is_empty()
                    || n.contains(['\r', '\n'])
                    || !seen.insert(n.trim().to_ascii_lowercase())
            }) {
                return Err(qpxf_error("retained MNA names are empty or duplicated"));
            }
        }
        self.validate_bindings()?;
        let anchor = self.frequency_anchor();
        let origin = vec![0; anchor.len()];
        for (i, frequency) in self.request.frequencies_hz.iter().enumerate() {
            if i % 256 == 0 {
                check_abort(abort)?;
            }
            if self.probe_offsets_hz[i]
                != grid
                    .frequency_relative_to(*frequency, &anchor, &origin)
                    .map_err(numerical_error)?
                || self.output_frequencies_hz[i]
                    != grid
                        .frequency_relative_to(*frequency, &anchor, &self.request.output_lattice)
                        .map_err(numerical_error)?
            {
                return Err(qpxf_error(
                    "retained physical frequencies differ from their exact anchor",
                ));
            }
        }
        if self.output_frequencies_hz.windows(2).any(|f| f[0] >= f[1]) {
            return Err(qpxf_error(
                "retained output grid collapses at this tone tuple",
            ));
        }
        Ok(grid)
    }

    fn validate_bindings(&self) -> Result<(), SimulationError> {
        let node_count = self.node_names.len();
        let node = |name: &str| -> Result<Option<usize>, SimulationError> {
            if self.ground_policy.is_ground(name.trim()) {
                return Ok(None);
            }
            self.node_names
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name.trim()))
                .map(Some)
                .ok_or_else(|| qpxf_error("retained output node is missing"))
        };
        let mut observation = match &self.request.output {
            QpxfOutput::Voltage { positive, negative } => {
                let positive = node(positive)?;
                let negative = node(negative)?;
                if positive == negative {
                    return Err(qpxf_error("retained output nodes are identical"));
                }
                positive
                    .map(|row| (row, Complex64::ONE))
                    .into_iter()
                    .chain(negative.map(|row| (row, -Complex64::ONE)))
                    .collect::<Vec<_>>()
            }
            QpxfOutput::BranchCurrent { branch } => {
                let row = self
                    .branch_names
                    .iter()
                    .position(|n| n.eq_ignore_ascii_case(branch.trim()))
                    .ok_or_else(|| qpxf_error("retained output branch is missing"))?;
                vec![(node_count + row, Complex64::ONE)]
            }
        };
        observation.sort_by_key(|(row, _)| *row);
        if observation != self.observation {
            return Err(qpxf_error(
                "retained output observation differs from its request",
            ));
        }
        let mut seen = BTreeSet::new();
        for source in &self.input_sources {
            if source.name.trim().is_empty()
                || source.name.contains(['\r', '\n'])
                || !seen.insert(source.name.trim().to_ascii_lowercase())
            {
                return Err(qpxf_error("retained sources are empty or duplicated"));
            }
            let valid = match source.quantity {
                QpxfQuantity::Voltage => {
                    source.injections.len() == 1
                        && source.injections[0].1 == Complex64::ONE
                        && source.injections[0]
                            .0
                            .checked_sub(node_count)
                            .and_then(|row| self.branch_names.get(row))
                            .is_some_and(|name| name.eq_ignore_ascii_case(&source.name))
                }
                QpxfQuantity::Current => {
                    (1..=2).contains(&source.injections.len())
                        && source.injections.iter().all(|(row, value)| {
                            *row < node_count
                                && (*value == Complex64::ONE || *value == -Complex64::ONE)
                        })
                        && (source.injections.len() == 1
                            || (source.injections[0].0 != source.injections[1].0
                                && source.injections[0].1 == -source.injections[1].1))
                }
            };
            if !valid {
                return Err(qpxf_error(
                    "retained independent source direction is invalid",
                ));
            }
        }
        match &self.request.input_sources {
            QpxfSources::Named(names) => {
                if names.len() != self.input_sources.len()
                    || names
                        .iter()
                        .zip(&self.input_sources)
                        .any(|(name, source)| !name.trim().eq_ignore_ascii_case(&source.name))
                {
                    return Err(qpxf_error(
                        "retained source selection differs from its request",
                    ));
                }
            }
            QpxfSources::AllIndependent => {
                if self.input_sources.windows(2).any(|sources| {
                    sources[0].name.to_ascii_lowercase() >= sources[1].name.to_ascii_lowercase()
                }) {
                    return Err(qpxf_error(
                        "retained all-source selection is not in canonical order",
                    ));
                }
            }
        }
        Ok(())
    }
}

impl QpxfAnalysisResult {
    pub(super) fn bind(
        metadata: QpxfResultMetadata,
        solutions: Vec<QuasiPeriodicAdjointSolution>,
        transfers: Vec<QpxfTransfer>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let mut result = Self {
            metadata,
            solutions,
            transfers,
        };
        result.metadata.retained_identity = result.payload_identity(abort)?;
        result.validate_retained_payload_with_abort(limits, abort)?;
        Ok(result)
    }

    fn payload_identity(&self, abort: &dyn AbortSignal) -> Result<String, SimulationError> {
        check_abort(abort)?;
        let mut hash = blake3::Hasher::new();
        hash.update(b"rspice-qpxf-complete-response-v1\0");
        let mut metadata = self.metadata.clone();
        metadata.retained_identity.clear();
        let bytes = serde_json::to_vec(&metadata).map_err(|e| qpxf_error(e.to_string()))?;
        hb_identity_field(&mut hash, "metadata", &bytes);
        for row in self
            .solutions
            .iter()
            .flat_map(|s| &s.sensitivities)
            .chain(self.transfers.iter().map(|t| &t.values))
        {
            hash.update(&(row.len() as u64).to_le_bytes());
            for (i, value) in row.iter().enumerate() {
                if i % 256 == 0 {
                    check_abort(abort)?;
                }
                hash.update(&value.re.to_bits().to_le_bytes());
                hash.update(&value.im.to_bits().to_le_bytes());
            }
        }
        // Frequencies, group-delay statuses and solution anchors are derived
        // exactly from metadata and complex data and checked below.
        Ok(hash.finalize().to_hex().to_string())
    }

    pub fn validate_retained_payload_with_abort(
        &self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        let m = &self.metadata;
        let points = m.request.frequencies_hz.len();
        let coordinates = m.node_names.len().saturating_add(m.branch_names.len());
        let paths = m.input_sources.len().saturating_mul(m.input_lattices.len());
        if self.solutions.len() != points
            || self
                .solutions
                .iter()
                .any(|s| s.sensitivities.len() != coordinates)
            || self.transfers.len() != paths
        {
            return Err(qpxf_error("retained complete result has an invalid shape"));
        }
        let lengths: Vec<_> = self
            .solutions
            .iter()
            .flat_map(|s| s.sensitivities.iter().map(Vec::len))
            .chain(self.transfers.iter().map(|t| t.values.len()))
            .collect();
        let grid = m.validate_transfer_layout_with_abort(&lengths, limits, abort)?;
        let anchor = m.frequency_anchor();
        for (i, solution) in self.solutions.iter().enumerate() {
            check_abort(abort)?;
            if solution.frequency_hz != m.request.frequencies_hz[i]
                || solution.frequency_lattice != anchor
                || solution.normalized_residual != m.normalized_residuals[i]
            {
                return Err(qpxf_error(
                    "retained adjoint anchor or convergence evidence is invalid",
                ));
            }
            for (j, value) in solution.sensitivities.iter().flatten().enumerate() {
                if j % 256 == 0 {
                    check_abort(abort)?;
                }
                if !value.re.is_finite() || !value.im.is_finite() {
                    return Err(qpxf_error(
                        "retained adjoint contains a nonfinite coefficient",
                    ));
                }
            }
        }
        for (path, transfer) in self.transfers.iter().enumerate() {
            check_abort(abort)?;
            let source_index = path / m.input_lattices.len();
            let tuple = &m.input_lattices[path % m.input_lattices.len()];
            let index = grid.index_of(tuple).expect("validated input tuple");
            if transfer.input_source != source_index
                || transfer.input_lattice != *tuple
                || transfer.input_frequencies_hz.len() != points
            {
                return Err(qpxf_error(
                    "retained transfer path differs from the selected source/tuple",
                ));
            }
            for (i, solution) in self.solutions.iter().enumerate() {
                if i % 256 == 0 {
                    check_abort(abort)?;
                }
                let expected: Complex64 = m.input_sources[source_index]
                    .injections
                    .iter()
                    .map(|(row, direction)| solution.sensitivities[*row][index].conj() * direction)
                    .sum();
                if !expected.re.is_finite()
                    || !expected.im.is_finite()
                    || transfer.values[i] != expected
                    || transfer.input_frequencies_hz[i]
                        != grid
                            .frequency_relative_to(m.request.frequencies_hz[i], &anchor, tuple)
                            .map_err(numerical_error)?
                {
                    return Err(qpxf_error(
                        "retained transfer differs from the complete adjoint projection",
                    ));
                }
            }
            let expected_delay = m
                .request
                .group_delay
                .then(|| {
                    delay::from_samples(
                        &m.output_frequencies_hz,
                        &transfer.values,
                        m.request.group_delay_magnitude_floor,
                        abort,
                    )
                })
                .transpose()?;
            if transfer.group_delay != expected_delay {
                return Err(qpxf_error(
                    "retained group delay differs from the sampled phase derivative",
                ));
            }
        }
        if m.retained_identity != self.payload_identity(abort)? {
            return Err(qpxf_error(
                "retained complete result identity is incompatible or altered",
            ));
        }
        Ok(grid)
    }

    pub fn into_transfer_parts(self) -> (QpxfResultMetadata, Vec<Vec<Complex64>>) {
        let rows = self
            .solutions
            .into_iter()
            .flat_map(|s| s.sensitivities)
            .chain(self.transfers.into_iter().map(|t| t.values))
            .collect();
        (self.metadata, rows)
    }

    pub fn from_transfer_parts_with_abort(
        metadata: QpxfResultMetadata,
        rows: Vec<Vec<Complex64>>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let grid = metadata.validate_transfer_layout_with_abort(
            &rows.iter().map(Vec::len).collect::<Vec<_>>(),
            limits,
            abort,
        )?;
        let coordinates = metadata.node_names.len() + metadata.branch_names.len();
        let anchor = metadata.frequency_anchor();
        let mut rows = rows.into_iter();
        let solutions = metadata
            .request
            .frequencies_hz
            .iter()
            .zip(&metadata.normalized_residuals)
            .map(|(frequency, residual)| QuasiPeriodicAdjointSolution {
                frequency_hz: *frequency,
                frequency_lattice: anchor.clone(),
                normalized_residual: *residual,
                sensitivities: rows.by_ref().take(coordinates).collect(),
            })
            .collect();
        let mut transfers = Vec::new();
        for input_source in 0..metadata.input_sources.len() {
            for input_lattice in &metadata.input_lattices {
                check_abort(abort)?;
                let values = rows.next().expect("validated transfer path");
                let input_frequencies_hz = metadata
                    .request
                    .frequencies_hz
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        if i % 256 == 0 {
                            check_abort(abort)?;
                        }
                        grid.frequency_relative_to(*f, &anchor, input_lattice)
                            .map_err(numerical_error)
                    })
                    .collect::<Result<_, _>>()?;
                let group_delay = metadata
                    .request
                    .group_delay
                    .then(|| {
                        delay::from_samples(
                            &metadata.output_frequencies_hz,
                            &values,
                            metadata.request.group_delay_magnitude_floor,
                            abort,
                        )
                    })
                    .transpose()?;
                transfers.push(QpxfTransfer {
                    input_source,
                    input_lattice: input_lattice.clone(),
                    input_frequencies_hz,
                    values,
                    group_delay,
                });
            }
        }
        let result = Self {
            metadata,
            solutions,
            transfers,
        };
        result.validate_retained_payload_with_abort(limits, abort)?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests;

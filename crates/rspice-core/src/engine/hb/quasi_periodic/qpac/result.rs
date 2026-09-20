//! Versioned QPAC evidence and bounded transfer of the complete complex solve.
use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicGridConfig;
use crate::{ResourceKind, ResourceLimitError, ResourceLimits};

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QpacInputQuantity {
    Voltage,
    Current,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpacResultMetadata {
    pub(super) version: u32,
    pub request: QpacRequest,
    pub operating_point_identity: String,
    pub grid: QuasiPeriodicGridConfig,
    pub input_quantity: QpacInputQuantity,
    pub tone_frequencies_hz: Vec<Value>,
    pub tuples: Vec<Vec<i32>>,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub input_frequencies_hz: Vec<Value>,
    pub output_frequencies_hz: Vec<Value>,
    pub normalized_residuals: Vec<Value>,
    pub(super) output_rows: [Option<usize>; 2],
    pub(super) retained_identity: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpacAnalysisResult {
    pub metadata: QpacResultMetadata,
    /// Offset, then MNA coordinate, then signed tuple; phasors per unit input.
    pub unit_solutions: Vec<QuasiPeriodicAcSolution>,
    /// Differential voltage per unit input at the selected output tuple.
    pub output_transfer: Vec<Complex64>,
    /// Differential voltage with authored magnitude and phase applied.
    pub output_response: Vec<Complex64>,
}

impl QpacResultMetadata {
    pub fn retained_identity(&self) -> &str {
        &self.retained_identity
    }

    pub fn drive(&self) -> Complex64 {
        Complex64::from_polar(
            self.request.magnitude,
            (self.request.phase_degrees % 360.0).to_radians(),
        )
    }

    /// Validate declared buffer sizes before any complex data is copied.
    /// Rows are offset-major MNA spectra followed by transfer and response.
    pub fn validate_transfer_layout_with_abort(
        &self,
        lengths: &[usize],
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        self.request.validate()?;
        let points = self.request.offsets_hz.len();
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
        let values = lengths
            .iter()
            .fold(0usize, |a, n| a.saturating_add(n.saturating_mul(2)))
            .saturating_add(points.saturating_mul(8))
            .saturating_add(
                self.tuples
                    .len()
                    .saturating_mul(self.grid.frequencies_hz.len()),
            )
            .saturating_add(self.grid.frequencies_hz.len());
        ResourceLimitError::ensure(ResourceKind::ResultValues, values, limits.max_result_values)?;
        if self.node_names.is_empty() || rows > 65_536 || lengths.len() != spectra.saturating_add(2)
        {
            return Err(qpac_error("retained transfer has an invalid MNA row count"));
        }
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(self.grid.clone(), limits, abort)
                .map_err(numerical_error)?,
        );
        if lengths[..spectra].iter().any(|n| *n != grid.len())
            || lengths[spectra..] != [points, points]
            || self.version != 1
            || !is_canonical_blake3_identity(&self.operating_point_identity)
            || !is_canonical_blake3_identity(&self.retained_identity)
            || self.tone_frequencies_hz != self.grid.frequencies_hz
            || self.tuples != grid.indices()
            || self.normalized_residuals.len() != points
            || self
                .normalized_residuals
                .iter()
                .any(|r| !r.is_finite() || !(0.0..=1.0).contains(r))
            || self.input_frequencies_hz.len() != points
            || self.output_frequencies_hz.len() != points
        {
            return Err(qpac_error(
                "retained result shape, lattice or convergence evidence is invalid",
            ));
        }
        for names in [&self.node_names, &self.branch_names] {
            let mut seen = BTreeSet::new();
            if names.iter().any(|name| {
                name.trim().is_empty()
                    || name.contains(['\r', '\n'])
                    || !seen.insert(name.to_ascii_lowercase())
            }) {
                return Err(qpac_error(
                    "retained MNA coordinate names are empty or duplicated",
                ));
            }
        }
        if self.output_rows[0] == self.output_rows[1] {
            return Err(qpac_error("retained output nodes are identical"));
        }
        for (row, name) in self
            .output_rows
            .iter()
            .zip([&self.request.output_node, &self.request.output_ref])
        {
            if let Some(row) = row {
                if !self
                    .node_names
                    .get(*row)
                    .is_some_and(|node| node.eq_ignore_ascii_case(name.trim()))
                {
                    return Err(qpac_error(
                        "retained output coordinate differs from its request",
                    ));
                }
            }
        }
        for (tuple, frequencies) in [
            (&self.request.input_lattice, &self.input_frequencies_hz),
            (&self.request.output_lattice, &self.output_frequencies_hz),
        ] {
            let index = grid
                .index_of(tuple)
                .ok_or_else(|| qpac_error("retained probe tuple is absent from the lattice"))?;
            for (offset, actual) in self.request.offsets_hz.iter().zip(frequencies) {
                if !actual.is_finite() || *actual != *offset + grid.frequencies_hz()[index] {
                    return Err(qpac_error(
                        "retained translated frequency differs from its signed tuple",
                    ));
                }
            }
        }
        Ok(grid)
    }
}

impl QpacAnalysisResult {
    pub(super) fn bind(
        metadata: QpacResultMetadata,
        unit_solutions: Vec<QuasiPeriodicAcSolution>,
        output_transfer: Vec<Complex64>,
        output_response: Vec<Complex64>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let mut result = Self {
            metadata,
            unit_solutions,
            output_transfer,
            output_response,
        };
        result.metadata.retained_identity = result.payload_identity(abort)?;
        result.validate_retained_payload_with_abort(limits, abort)?;
        Ok(result)
    }

    fn payload_identity(&self, abort: &dyn AbortSignal) -> Result<String, SimulationError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"rspice-qpac-complete-response-v1\0");
        let mut metadata = self.metadata.clone();
        metadata.retained_identity.clear();
        let bytes = serde_json::to_vec(&metadata).map_err(|e| qpac_error(e.to_string()))?;
        hb_identity_field(&mut hasher, "metadata", &bytes);
        for solution in &self.unit_solutions {
            hasher.update(&solution.offset_hz.to_bits().to_le_bytes());
            hasher.update(&solution.normalized_residual.to_bits().to_le_bytes());
        }
        for row in self
            .unit_solutions
            .iter()
            .flat_map(|s| &s.spectra)
            .chain([&self.output_transfer, &self.output_response])
        {
            check_abort(abort)?;
            hasher.update(&(row.len() as u64).to_le_bytes());
            for value in row {
                hasher.update(&value.re.to_bits().to_le_bytes());
                hasher.update(&value.im.to_bits().to_le_bytes());
            }
        }
        Ok(hasher.finalize().to_hex().to_string())
    }

    pub fn validate_retained_payload_with_abort(
        &self,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        let metadata = &self.metadata;
        let rows = metadata
            .node_names
            .len()
            .saturating_add(metadata.branch_names.len());
        if self.unit_solutions.len() != metadata.request.offsets_hz.len()
            || self.unit_solutions.iter().any(|s| s.spectra.len() != rows)
        {
            return Err(qpac_error(
                "retained solutions differ from the requested sweep or MNA shape",
            ));
        }
        let lengths: Vec<_> = self
            .unit_solutions
            .iter()
            .flat_map(|s| s.spectra.iter().map(Vec::len))
            .chain([self.output_transfer.len(), self.output_response.len()])
            .collect();
        let grid = metadata.validate_transfer_layout_with_abort(&lengths, limits, abort)?;
        let output = grid
            .index_of(&metadata.request.output_lattice)
            .expect("validated tuple");
        let drive = metadata.drive();
        for (i, solution) in self.unit_solutions.iter().enumerate() {
            check_abort(abort)?;
            if solution.offset_hz != metadata.request.offsets_hz[i]
                || solution.normalized_residual != metadata.normalized_residuals[i]
                || solution
                    .spectra
                    .iter()
                    .flatten()
                    .any(|v| !v.re.is_finite() || !v.im.is_finite())
            {
                return Err(qpac_error(
                    "retained solution offset, residual or coefficients are invalid",
                ));
            }
            let voltage = |row: Option<usize>| {
                row.map_or(Complex64::ZERO, |row| solution.spectra[row][output])
            };
            let transfer = voltage(metadata.output_rows[0]) - voltage(metadata.output_rows[1]);
            let response = transfer * drive;
            if !response.re.is_finite()
                || !response.im.is_finite()
                || self.output_transfer[i] != transfer
                || self.output_response[i] != response
            {
                return Err(qpac_error(
                    "retained differential response differs from the complete MNA solve",
                ));
            }
        }
        if metadata.retained_identity != self.payload_identity(abort)? {
            return Err(qpac_error(
                "retained result identity is incompatible or altered",
            ));
        }
        Ok(grid)
    }

    pub fn into_transfer_parts(self) -> (QpacResultMetadata, Vec<Vec<Complex64>>) {
        let mut rows: Vec<_> = self
            .unit_solutions
            .into_iter()
            .flat_map(|s| s.spectra)
            .collect();
        rows.push(self.output_transfer);
        rows.push(self.output_response);
        (self.metadata, rows)
    }

    pub fn from_transfer_parts_with_abort(
        metadata: QpacResultMetadata,
        rows: Vec<Vec<Complex64>>,
        limits: &ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        metadata.validate_transfer_layout_with_abort(
            &rows.iter().map(Vec::len).collect::<Vec<_>>(),
            limits,
            abort,
        )?;
        let count = metadata.node_names.len() + metadata.branch_names.len();
        let mut rows = rows.into_iter();
        let unit_solutions = metadata
            .request
            .offsets_hz
            .iter()
            .zip(&metadata.normalized_residuals)
            .map(|(offset, residual)| QuasiPeriodicAcSolution {
                offset_hz: *offset,
                normalized_residual: *residual,
                spectra: rows.by_ref().take(count).collect(),
            })
            .collect();
        let result = Self {
            metadata,
            unit_solutions,
            output_transfer: rows.next().expect("validated transfer"),
            output_response: rows.next().expect("validated response"),
        };
        result.validate_retained_payload_with_abort(limits, abort)?;
        Ok(result)
    }
}

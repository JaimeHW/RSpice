//! Versioned producer and complete numerical-payload identity for QPSS.
use super::*;

fn payload_version(integral_names: &[String]) -> u32 {
    if integral_names.is_empty() { 1 } else { 2 }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Producer {
    semantic_netlist: String,
    resolved_simulation: String,
    analysis: String,
}

impl Producer {
    pub(super) fn capture(
        netlist: &Netlist,
        simulation: &crate::config::SimulationConfig,
        config: &QpssConfig,
    ) -> Result<Self, SimulationError> {
        let semantic_netlist = netlist_checkpoint_identity(netlist)
            .ok_or_else(|| invalid("netlist has no canonical semantic identity"))?;
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"rspice-qpss-torus-source-config-v1\0");
        let bytes = serde_json::to_vec(config)
            .map_err(|error| invalid(format!("configuration identity failed: {error}")))?;
        hasher.update(&bytes);
        Ok(Self {
            semantic_netlist,
            resolved_simulation: hb_resolved_simulation_identity(simulation),
            analysis: hasher.finalize().to_hex().to_string(),
        })
    }
}

/// Complete, versioned independent-tone operating point. Deserializing this
/// structure alone does not authenticate it: consumers must call the engine's
/// validation method against their netlist and resolved configuration.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssOperatingPoint {
    version: u32,
    producer: Producer,
    config: QpssConfig,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    integral_names: Vec<String>,
    spectra: Vec<Vec<Complex64>>,
    iterations: usize,
    normalized_residual: Value,
    retained_identity: String,
}

/// Opaque scalar metadata for a QPSS point whose spectral rows travel in
/// separate numeric buffers. Rejoining always validates the complete payload.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpssOperatingPointMetadata {
    version: u32,
    producer: Producer,
    config: QpssConfig,
    node_names: Vec<String>,
    branch_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    integral_names: Vec<String>,
    iterations: usize,
    normalized_residual: Value,
    retained_identity: String,
}

impl QpssOperatingPointMetadata {
    /// Bound a transport's declared row layout before it copies any referenced
    /// numeric buffer. Rejoining the rows still checks their complete identity.
    pub fn validate_transfer_layout_with_abort(
        &self,
        row_lengths: &[usize],
        limits: &crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        check_abort(abort)?;
        self.config.solver.validate().map_err(numerical_error)?;
        let grid = QuasiPeriodicGrid::new_with_abort(self.config.grid.clone(), limits, abort)
            .map_err(numerical_error)?;
        crate::analysis::quasi_periodic::solve::check_workload(
            row_lengths.len(),
            &grid,
            &self.config.solver.linear,
            limits,
        )
        .map_err(numerical_error)?;
        if self.node_names.is_empty()
            || self.version != payload_version(&self.integral_names)
            || self
                .node_names
                .len()
                .checked_add(self.branch_names.len())
                .and_then(|rows| rows.checked_add(self.integral_names.len()))
                != Some(row_lengths.len())
            || row_lengths.iter().any(|length| *length != grid.len())
        {
            return Err(invalid(
                "retained transfer layout differs from its MNA tone lattice",
            ));
        }
        Ok(())
    }
}

impl QpssOperatingPoint {
    pub fn config(&self) -> &QpssConfig {
        &self.config
    }
    pub fn node_names(&self) -> &[String] {
        &self.node_names
    }
    pub fn branch_names(&self) -> &[String] {
        &self.branch_names
    }
    /// Node coordinates first, then canonical branch currents, then any
    /// distributed-network auxiliary currents named by `branch_names`.
    pub fn spectra(&self) -> &[Vec<Complex64>] {
        &self.spectra[..self.physical_rows()]
    }
    fn physical_rows(&self) -> usize {
        self.node_names
            .len()
            .saturating_add(self.branch_names.len())
            .min(self.spectra.len())
    }
    /// Canonical behavioral SDT identities, separate from electrical currents.
    pub fn integral_names(&self) -> &[String] {
        &self.integral_names
    }
    /// Integral values in their authored integrand-times-seconds units.
    pub fn integral_spectra(&self) -> &[Vec<Complex64>] {
        &self.spectra[self.physical_rows()..]
    }
    /// Complete numerical state: physical rows followed by integral states.
    /// Dependent solvers and transport must retain every row.
    pub fn complete_spectra(&self) -> &[Vec<Complex64>] {
        &self.spectra
    }
    pub fn iterations(&self) -> usize {
        self.iterations
    }
    pub fn normalized_residual(&self) -> Value {
        self.normalized_residual
    }
    pub fn retained_identity(&self) -> &str {
        &self.retained_identity
    }

    /// Check a saved or transported numerical payload without asserting that
    /// it belongs to a particular circuit. Dependent analyses must additionally
    /// call the engine validator against the current resolved deck.
    pub fn validate_retained_payload_with_abort(
        &self,
        limits: &crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        self.config.solver.validate().map_err(numerical_error)?;
        let grid = Arc::new(
            QuasiPeriodicGrid::new_with_abort(self.config.grid.clone(), limits, abort)
                .map_err(numerical_error)?,
        );
        crate::analysis::quasi_periodic::solve::check_workload(
            self.spectra.len(),
            &grid,
            &self.config.solver.linear,
            limits,
        )
        .map_err(numerical_error)?;
        if !is_canonical_blake3_identity(&self.producer.semantic_netlist)
            || !is_canonical_blake3_identity(&self.producer.resolved_simulation)
            || !is_canonical_blake3_identity(&self.producer.analysis)
        {
            return Err(invalid("retained state has invalid producer identities"));
        }
        self.validate_payload(&grid, abort)?;
        Ok(grid)
    }

    pub fn into_transfer_parts(self) -> (QpssOperatingPointMetadata, Vec<Vec<Complex64>>) {
        let metadata = QpssOperatingPointMetadata {
            version: self.version,
            producer: self.producer,
            config: self.config,
            node_names: self.node_names,
            branch_names: self.branch_names,
            integral_names: self.integral_names,
            iterations: self.iterations,
            normalized_residual: self.normalized_residual,
            retained_identity: self.retained_identity,
        };
        (metadata, self.spectra)
    }

    pub fn from_transfer_parts_with_abort(
        metadata: QpssOperatingPointMetadata,
        spectra: Vec<Vec<Complex64>>,
        limits: &crate::ResourceLimits,
        abort: &dyn AbortSignal,
    ) -> Result<Self, SimulationError> {
        let point = Self {
            version: metadata.version,
            producer: metadata.producer,
            config: metadata.config,
            node_names: metadata.node_names,
            branch_names: metadata.branch_names,
            integral_names: metadata.integral_names,
            spectra,
            iterations: metadata.iterations,
            normalized_residual: metadata.normalized_residual,
            retained_identity: metadata.retained_identity,
        };
        point.validate_retained_payload_with_abort(limits, abort)?;
        Ok(point)
    }

    pub(super) fn bind(
        producer: Producer,
        config: QpssConfig,
        node_names: Vec<String>,
        branch_names: Vec<String>,
        integral_names: Vec<String>,
        solution: QuasiPeriodicSolution,
    ) -> Result<Self, SimulationError> {
        let mut point = Self {
            version: payload_version(&integral_names),
            producer,
            config,
            node_names,
            branch_names,
            integral_names,
            spectra: solution.spectra().to_vec(),
            iterations: solution.iterations(),
            normalized_residual: solution.normalized_residual(),
            retained_identity: String::new(),
        };
        point.retained_identity = point.payload_identity()?;
        Ok(point)
    }

    fn payload_identity(&self) -> Result<String, SimulationError> {
        let mut hasher = blake3::Hasher::new();
        hasher.update(b"rspice-qpss-complete-mna-state-v1\0");
        let metadata = serde_json::to_vec(&(
            self.version,
            &self.producer,
            &self.config,
            &self.node_names,
            &self.branch_names,
            self.iterations,
        ))
        .map_err(|error| invalid(format!("retained state identity failed: {error}")))?;
        hb_identity_field(&mut hasher, "metadata", &metadata);
        // Preserve the v1 identity byte-for-byte for existing memoryless points.
        if !self.integral_names.is_empty() {
            let names = serde_json::to_vec(&self.integral_names)
                .map_err(|error| invalid(format!("integral identity failed: {error}")))?;
            hb_identity_field(&mut hasher, "behavioral-sdt/v1", &names);
        }
        hasher.update(&self.normalized_residual.to_bits().to_le_bytes());
        hasher.update(&(self.spectra.len() as u64).to_le_bytes());
        for spectrum in &self.spectra {
            hasher.update(&(spectrum.len() as u64).to_le_bytes());
            for coefficient in spectrum {
                hasher.update(&coefficient.re.to_bits().to_le_bytes());
                hasher.update(&coefficient.im.to_bits().to_le_bytes());
            }
        }
        Ok(hasher.finalize().to_hex().to_string())
    }

    fn validate_payload(
        &self,
        grid: &QuasiPeriodicGrid,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        check_abort(abort)?;
        // Bound the actual deserialized arrays before hashing their contents;
        // metadata describing a small lattice must not conceal oversized rows.
        if self
            .spectra
            .iter()
            .any(|spectrum| spectrum.len() != grid.len())
        {
            return Err(invalid("retained spectrum differs from its tone lattice"));
        }
        if self.node_names.is_empty()
            || self.spectra.len()
                != self
                    .node_names
                    .len()
                    .saturating_add(self.branch_names.len())
                    .saturating_add(self.integral_names.len())
            || !self.normalized_residual.is_finite()
            || !(0.0..=1.0).contains(&self.normalized_residual)
            || self.iterations > self.config.solver.max_iterations
        {
            return Err(invalid(
                "retained state has incomplete shape or convergence evidence",
            ));
        }
        if self.version != payload_version(&self.integral_names)
            || !is_canonical_blake3_identity(&self.retained_identity)
            || self.retained_identity != self.payload_identity()?
        {
            return Err(invalid(
                "retained operating-point identity is incompatible or altered",
            ));
        }
        for names in [&self.node_names, &self.branch_names, &self.integral_names] {
            let mut seen = BTreeSet::new();
            for name in names {
                if name.trim().is_empty() || !seen.insert(name.to_ascii_lowercase()) {
                    return Err(invalid(
                        "retained MNA coordinate names are empty or duplicated",
                    ));
                }
            }
        }
        for spectrum in &self.spectra {
            check_abort(abort)?;
            if spectrum.len() != grid.len() {
                return Err(invalid("retained spectrum differs from its tone lattice"));
            }
            for (k, value) in spectrum.iter().enumerate() {
                if !value.re.is_finite()
                    || !value.im.is_finite()
                    || *value != spectrum[grid.len() - 1 - k].conj()
                {
                    return Err(invalid(
                        "retained spectrum is non-finite or lacks real conjugate symmetry",
                    ));
                }
            }
        }
        Ok(())
    }
}

impl Engine {
    /// Validate a deserialized point's exact producer, basis, convergence
    /// evidence and complete MNA payload before a dependent analysis uses it.
    pub fn validate_qpss_operating_point_with_abort(
        &self,
        netlist: &Netlist,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<Arc<QuasiPeriodicGrid>, SimulationError> {
        check_abort(abort)?;
        let engine = self.resolved_for_netlist(netlist);
        let grid =
            point.validate_retained_payload_with_abort(&engine.config.resource_limits, abort)?;
        engine.ensure_matrix_unknowns(point.spectra.len().saturating_mul(grid.len()))?;
        engine.ensure_result_values(
            point
                .spectra
                .len()
                .saturating_mul(grid.len())
                .saturating_mul(2),
        )?;
        if point.producer != Producer::capture(netlist, &engine.config, &point.config)? {
            return Err(invalid(
                "retained state belongs to different source, model, simulation or analysis settings",
            ));
        }
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        let solver = engine.qpss_circuit_solver(&circuit, &grid, false)?;
        let names = solver
            .try_periodic_mna_branch_names()
            .map_err(|error| invalid(error.to_string()))?;
        let (branches, integrals) = names.split_at(solver.physical_branch_count());
        if point.node_names != engine.hb_build_node_names(&circuit, circuit.num_nodes())
            || point.branch_names != branches
            || point.integral_names != integrals
        {
            return Err(invalid(
                "retained MNA coordinate map differs from the current circuit",
            ));
        }
        check_abort(abort)?;
        if point.producer != Producer::capture(netlist, &engine.config, &point.config)? {
            return Err(invalid(
                "semantic producer inputs changed during retained-state validation",
            ));
        }
        Ok(grid)
    }
}

//! Struct-of-Arrays storage for independent sources.
//!
//! [`VoltageSources`] and [`CurrentSources`] hold the waveform specification
//! and, for voltage sources, the branch unknown each one adds to the MNA
//! system. Values are evaluated per analysis point rather than stored, so
//! these containers keep the specification and the bindings, not a sampled
//! waveform.

use super::*;
use crate::circuit::{CircuitError, projection_changed};
use std::cell::Cell;
use std::collections::VecDeque;

#[derive(Debug, Default, Clone)]
pub struct VoltageSources {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub branch_indices: Vec<NodeId>,
    pub dc_values: Vec<Value>,
    /// AC magnitude for AC/HB analysis
    pub ac_magnitudes: Vec<Value>,
    /// AC phase in radians for AC/HB analysis
    pub ac_phases: Vec<Value>,
    /// Full source specification for transient waveform evaluation
    pub source_specs: Vec<Option<crate::netlist::SourceSpec>>,
    /// Circuit-owned external PWL snapshots aligned with `source_specs`.
    ///
    /// External files are resolved once while the circuit is built. Keeping the
    /// loaded waveform here makes a simulation deterministic if the file later
    /// changes and avoids filesystem/cache work in the transient hot path.
    pwl_waveforms: Vec<Option<Arc<crate::device::pwl_file::PwlWaveform>>>,
    /// Pre-baked CSC indices: [br->np, np->br, br->nn, nn->br] per source
    csc_indices: Vec<[Option<CscIndex>; 4]>,
    /// Optional transient context used to resolve source defaults.
    transient_context: Option<TransientSourceContext>,
    /// Validated independent-source topology and allocation-free projection
    /// scratch, finalized after circuit node identities are stable.
    constraint_projection: VoltageConstraintProjectionState,
}

#[derive(Debug, Default, Clone)]
enum VoltageConstraintProjectionState {
    #[default]
    Uninitialized,
    Ready(VoltageConstraintProjection),
    Invalid(String),
}

#[derive(Debug, Clone)]
struct VoltageConstraintProjection {
    constraints: Vec<VoltageConstraintTopology>,
    components: Vec<VoltageConstraintProjectionComponent>,
}

#[derive(Debug, Clone, Copy)]
struct VoltageConstraintTopology {
    node_pos: NodeId,
    node_neg: NodeId,
}

#[derive(Debug, Clone)]
struct VoltageConstraintProjectionComponent {
    grounded: bool,
    /// Breadth-first forest order. Every non-root entry refers to an earlier
    /// parent, so projection can publish the complete affine component without
    /// allocating per Newton iteration.
    nodes: Vec<VoltageConstraintProjectionNode>,
}

#[derive(Debug, Clone)]
struct VoltageConstraintProjectionNode {
    node: NodeId,
    parent: Option<VoltageConstraintProjectionParent>,
    /// Per-call scratch. Components are immutable after finalization and each
    /// worker owns a circuit clone, so interior mutation avoids hot-path
    /// allocation without introducing shared state.
    relative: Cell<Value>,
    signed_target: Cell<Value>,
    projected: Cell<Value>,
}

#[derive(Debug, Clone, Copy)]
struct VoltageConstraintProjectionParent {
    node_index: usize,
    source_index: usize,
    target_sign: Value,
}

#[derive(Debug, Clone, Copy)]
struct TransientSourceContext {
    tstep: Value,
    tstop: Value,
    dialect: crate::config::SpiceDialect,
    resource_limits: crate::resource::ResourceLimits,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PwlCacheKey {
    path: String,
    file_len: Option<u64>,
    modified_nanos: Option<u128>,
    time_scale_bits: u64,
    value_scale_bits: u64,
    time_offset_bits: u64,
    value_offset_bits: u64,
}

impl PwlCacheKey {
    fn new(
        path: &str,
        time_scale: Value,
        value_scale: Value,
        time_offset: Value,
        value_offset: Value,
    ) -> Self {
        let metadata = std::fs::metadata(path).ok();
        let file_len = metadata.as_ref().map(std::fs::Metadata::len);
        let modified_nanos = metadata
            .and_then(|metadata| metadata.modified().ok())
            .and_then(|modified| modified.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|duration| duration.as_nanos());
        Self {
            path: path.to_string(),
            file_len,
            modified_nanos,
            time_scale_bits: time_scale.to_bits(),
            value_scale_bits: value_scale.to_bits(),
            time_offset_bits: time_offset.to_bits(),
            value_offset_bits: value_offset.to_bits(),
        }
    }
}

type PwlWaveformCache =
    crate::resource::BoundedCache<PwlCacheKey, Arc<crate::device::pwl_file::PwlWaveform>>;

fn pwl_waveform_cache() -> &'static RwLock<PwlWaveformCache> {
    static CACHE: OnceLock<RwLock<PwlWaveformCache>> = OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(PwlWaveformCache::default()))
}

fn pwl_error_log_cache() -> &'static RwLock<crate::resource::BoundedCache<PwlCacheKey, ()>> {
    static CACHE: OnceLock<RwLock<crate::resource::BoundedCache<PwlCacheKey, ()>>> =
        OnceLock::new();
    CACHE.get_or_init(|| RwLock::new(crate::resource::BoundedCache::default()))
}

fn pwl_key_dynamic_bytes(key: &PwlCacheKey) -> usize {
    key.path.len()
}

fn pwl_waveform_cache_entry_bytes(
    key: &PwlCacheKey,
    waveform: &crate::device::pwl_file::PwlWaveform,
) -> usize {
    crate::resource::estimated_cache_entry_bytes::<
        PwlCacheKey,
        Arc<crate::device::pwl_file::PwlWaveform>,
    >(pwl_key_dynamic_bytes(key), waveform.retained_bytes())
}

impl VoltageSources {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        dc_value: Value,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(0.0);
        self.ac_phases.push(0.0);
        self.source_specs.push(None);
        self.pwl_waveforms.push(None);
        self.csc_indices.push([None; 4]);
        self.invalidate_constraint_projection();
    }

    /// Add voltage source with full AC and transient specification
    pub fn add_with_ac_and_spec(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
    ) {
        self.add_with_ac_spec_and_pwl_waveform(
            name,
            node_pos,
            node_neg,
            branch_idx,
            dc_value,
            ac_magnitude,
            ac_phase,
            source_spec,
            None,
        );
    }

    pub(crate) fn add_with_ac_spec_and_pwl_waveform(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        branch_idx: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
        pwl_waveform: Option<Arc<crate::device::pwl_file::PwlWaveform>>,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.branch_indices.push(branch_idx);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
        self.source_specs.push(source_spec);
        self.pwl_waveforms.push(pwl_waveform);
        self.csc_indices.push([None; 4]);
        self.invalidate_constraint_projection();
    }

    /// Set transient context used to resolve waveform defaults.
    pub fn set_transient_context(&mut self, tstep: Value, tstop: Value) {
        self.set_transient_context_with_dialect(
            tstep,
            tstop,
            crate::config::SpiceDialect::BestAvailable,
        );
    }

    /// Set transient context used to resolve waveform defaults for a SPICE dialect.
    pub fn set_transient_context_with_dialect(
        &mut self,
        tstep: Value,
        tstop: Value,
        dialect: crate::config::SpiceDialect,
    ) {
        self.set_transient_context_with_dialect_and_limits(
            tstep,
            tstop,
            dialect,
            crate::resource::ResourceLimits::default(),
        );
    }

    pub(crate) fn set_transient_context_with_dialect_and_limits(
        &mut self,
        tstep: Value,
        tstop: Value,
        dialect: crate::config::SpiceDialect,
        resource_limits: crate::resource::ResourceLimits,
    ) {
        let step = if tstep.is_finite() && tstep > 0.0 {
            tstep
        } else {
            1e-12
        };
        let stop = if tstop.is_finite() && tstop > 0.0 {
            tstop
        } else {
            1e99
        };
        self.transient_context = Some(TransientSourceContext {
            tstep: step,
            tstop: stop,
            dialect,
            resource_limits,
        });
    }

    /// Clear transient context and use static waveform defaults.
    pub fn clear_transient_context(&mut self) {
        self.transient_context = None;
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    /// Return the exact complex excitation stamped into this source's AC
    /// branch equation.
    ///
    /// Keep the inactive-source cutoff here so the matrix right-hand side and
    /// the post-solve ideal-constraint projection cannot disagree at the
    /// cutoff boundary or reconstruct a phasor with different low bits.
    pub(crate) fn ac_excitation(&self, index: usize) -> Complex64 {
        let magnitude = self.ac_magnitudes[index];
        if magnitude.abs() <= 1.0e-15 {
            Complex64::new(0.0, 0.0)
        } else {
            Complex64::from_polar(magnitude, self.ac_phases[index])
        }
    }

    pub(crate) fn freeze_transient_source_at_time(
        &mut self,
        name: &str,
        time: Value,
    ) -> Option<Value> {
        let index = self
            .names
            .iter()
            .position(|source_name| source_name.eq_ignore_ascii_case(name))?;
        let value = match self.source_specs[index].as_ref() {
            Some(spec) => Self::evaluate_source_at_time_with_context_and_pwl(
                spec,
                time,
                self.transient_context,
                self.pwl_waveforms[index].as_deref(),
            ),
            None => self.dc_values[index],
        };
        self.dc_values[index] = value;
        // Freezing is a whole-source contract for periodic initialization,
        // not merely removal of its time-domain waveform.  HB treats a
        // source with no specification but a non-zero AC phasor as a carrier
        // drive, so retaining these sidecar fields would silently reactivate
        // a selected DcAcTransient modulation source during the frozen solve.
        self.ac_magnitudes[index] = 0.0;
        self.ac_phases[index] = 0.0;
        self.source_specs[index] = None;
        self.pwl_waveforms[index] = None;
        Some(value)
    }

    /// Iterate transient specifications with their canonical source names and
    /// circuit-owned PWL snapshots. Keeping the name coupled to the same SoA
    /// ordinal lets analysis clients select authored sources without reparsing
    /// or accidentally reading a changed external waveform file.
    pub(crate) fn transient_specs_named_with_pwl(
        &self,
    ) -> impl Iterator<
        Item = (
            &str,
            &crate::netlist::SourceSpec,
            Option<&crate::device::pwl_file::PwlWaveform>,
        ),
    > {
        self.names
            .iter()
            .zip(&self.source_specs)
            .zip(&self.pwl_waveforms)
            .filter_map(|((name, spec), waveform)| {
                spec.as_ref()
                    .map(|spec| (name.as_str(), spec, waveform.as_deref()))
            })
    }

    /// Link indices to StaticMatrix for O(1) stamping
    pub fn link_indices(&mut self, matrix: &StaticMatrix, get_branch_idx: impl Fn(usize) -> usize) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = get_branch_idx(self.branch_indices[i]);

            // br->np and np->br
            if np > 0 {
                self.csc_indices[i][0] = matrix.get_index(br - 1, np - 1);
                self.csc_indices[i][1] = matrix.get_index(np - 1, br - 1);
            }
            // br->nn and nn->br
            if nn > 0 {
                self.csc_indices[i][2] = matrix.get_index(br - 1, nn - 1);
                self.csc_indices[i][3] = matrix.get_index(nn - 1, br - 1);
            }
        }
        if matches!(
            self.constraint_projection,
            VoltageConstraintProjectionState::Uninitialized
        ) {
            let num_nodes = self
                .node_pos
                .iter()
                .chain(&self.node_neg)
                .copied()
                .max()
                .unwrap_or(0);
            let _ = self.finalize_constraint_projection(num_nodes);
        }
    }

    pub(crate) fn invalidate_constraint_projection(&mut self) {
        self.constraint_projection = VoltageConstraintProjectionState::Uninitialized;
    }

    /// Finalize the independent voltage-source graph after node remapping.
    ///
    /// A connected source component is projected as one affine system. Ideal
    /// source loops are rejected because their branch currents are not unique;
    /// silently reverting to pairwise snapping would make the result depend on
    /// source insertion order.
    pub(crate) fn finalize_constraint_projection(
        &mut self,
        num_nodes: usize,
    ) -> Result<(), CircuitError> {
        match self.build_constraint_projection(num_nodes) {
            Ok(projection) => {
                self.constraint_projection = VoltageConstraintProjectionState::Ready(projection);
                Ok(())
            }
            Err(error) => {
                let message = match error {
                    CircuitError::InvalidComponent(message) => message,
                    other => other.to_string(),
                };
                self.constraint_projection =
                    VoltageConstraintProjectionState::Invalid(message.clone());
                Err(CircuitError::InvalidComponent(message))
            }
        }
    }

    fn build_constraint_projection(
        &self,
        num_nodes: usize,
    ) -> Result<VoltageConstraintProjection, CircuitError> {
        let source_count = self.names.len();
        if self.node_pos.len() != source_count
            || self.node_neg.len() != source_count
            || self.branch_indices.len() != source_count
            || self.dc_values.len() != source_count
            || self.ac_magnitudes.len() != source_count
            || self.ac_phases.len() != source_count
            || self.source_specs.len() != source_count
            || self.pwl_waveforms.len() != source_count
            || self.csc_indices.len() != source_count
        {
            return Err(CircuitError::InvalidComponent(
                "independent voltage-source storage is internally inconsistent".to_string(),
            ));
        }

        let mut parents = (0..=num_nodes).collect::<Vec<_>>();
        let mut ranks = vec![0_u8; num_nodes + 1];
        let mut constraints = Vec::with_capacity(source_count);
        let mut adjacency = vec![Vec::<(NodeId, usize, Value)>::new(); num_nodes + 1];

        fn root(parents: &mut [usize], mut node: usize) -> usize {
            let mut representative = node;
            while parents[representative] != representative {
                representative = parents[representative];
            }
            while parents[node] != node {
                let next = parents[node];
                parents[node] = representative;
                node = next;
            }
            representative
        }

        for source_index in 0..source_count {
            let node_pos = self.node_pos[source_index];
            let node_neg = self.node_neg[source_index];
            if node_pos > num_nodes || node_neg > num_nodes {
                return Err(CircuitError::InvalidComponent(format!(
                    "independent voltage source '{}' references node outside the solved system",
                    self.names[source_index]
                )));
            }

            let root_pos = root(&mut parents, node_pos);
            let root_neg = root(&mut parents, node_neg);
            if root_pos == root_neg {
                return Err(CircuitError::InvalidComponent(format!(
                    "independent voltage source '{}' closes a singular ideal-source loop; its branch current is not uniquely determined",
                    self.names[source_index]
                )));
            }
            if ranks[root_pos] < ranks[root_neg] {
                parents[root_pos] = root_neg;
            } else {
                parents[root_neg] = root_pos;
                if ranks[root_pos] == ranks[root_neg] {
                    ranks[root_pos] = ranks[root_pos].saturating_add(1);
                }
            }

            constraints.push(VoltageConstraintTopology { node_pos, node_neg });
            // V(pos) = V(neg) + target, and conversely.
            adjacency[node_neg].push((node_pos, source_index, 1.0));
            adjacency[node_pos].push((node_neg, source_index, -1.0));
        }

        let mut components = Vec::new();
        let mut visited = vec![false; num_nodes + 1];
        let mut component_index = vec![usize::MAX; num_nodes + 1];
        for root_node in 0..=num_nodes {
            if visited[root_node] || adjacency[root_node].is_empty() {
                continue;
            }

            visited[root_node] = true;
            let mut nodes = vec![VoltageConstraintProjectionNode {
                node: root_node,
                parent: None,
                relative: Cell::new(0.0),
                signed_target: Cell::new(0.0),
                projected: Cell::new(0.0),
            }];
            component_index[root_node] = 0;
            let mut queue = VecDeque::from([root_node]);
            while let Some(node) = queue.pop_front() {
                let parent_index = component_index[node];
                for &(neighbor, source_index, target_sign) in &adjacency[node] {
                    if visited[neighbor] {
                        continue;
                    }
                    visited[neighbor] = true;
                    component_index[neighbor] = nodes.len();
                    nodes.push(VoltageConstraintProjectionNode {
                        node: neighbor,
                        parent: Some(VoltageConstraintProjectionParent {
                            node_index: parent_index,
                            source_index,
                            target_sign,
                        }),
                        relative: Cell::new(0.0),
                        signed_target: Cell::new(0.0),
                        projected: Cell::new(0.0),
                    });
                    queue.push_back(neighbor);
                }
            }
            components.push(VoltageConstraintProjectionComponent {
                grounded: root_node == 0,
                nodes,
            });
        }

        Ok(VoltageConstraintProjection {
            constraints,
            components,
        })
    }

    fn floating_constraint_common_mode(
        solution: &[Value],
        component: &VoltageConstraintProjectionComponent,
    ) -> Result<Value, CircuitError> {
        let mut scale: Value = 0.0;
        for entry in &component.nodes {
            let raw = solution[entry.node - 1];
            let sample = raw - entry.relative.get();
            if !sample.is_finite() {
                return Err(CircuitError::InvalidComponent(
                    "independent voltage-source projection overflowed while preserving a floating component common mode"
                        .to_string(),
                ));
            }
            scale = scale.max(sample.abs());
        }
        if scale == 0.0 {
            return Ok(0.0);
        }

        let mut sum = 0.0;
        let mut compensation = 0.0;
        for entry in &component.nodes {
            let normalized = (solution[entry.node - 1] - entry.relative.get()) / scale;
            let adjusted = normalized - compensation;
            let next = sum + adjusted;
            compensation = (next - sum) - adjusted;
            sum = next;
        }
        let common_mode = sum / component.nodes.len() as Value * scale;
        if !common_mode.is_finite() {
            return Err(CircuitError::InvalidComponent(
                "independent voltage-source projection produced a non-finite floating common mode"
                    .to_string(),
            ));
        }
        Ok(common_mode)
    }

    fn project_constraint_components(
        &self,
        solution: &mut [Value],
        mut target_value: impl FnMut(usize) -> Option<Value>,
    ) -> Result<bool, CircuitError> {
        if self.names.is_empty() {
            return Ok(false);
        }
        let projection = match &self.constraint_projection {
            VoltageConstraintProjectionState::Ready(projection) => projection,
            VoltageConstraintProjectionState::Invalid(message) => {
                return Err(CircuitError::InvalidComponent(message.clone()));
            }
            VoltageConstraintProjectionState::Uninitialized => {
                return Err(CircuitError::InvalidComponent(
                    "independent voltage-source constraint topology was not finalized".to_string(),
                ));
            }
        };

        if projection.constraints.len() != self.names.len()
            || self.node_pos.len() != self.names.len()
            || self.node_neg.len() != self.names.len()
            || self.dc_values.len() != self.names.len()
            || self.source_specs.len() != self.names.len()
            || self.pwl_waveforms.len() != self.names.len()
            || projection
                .constraints
                .iter()
                .zip(&self.node_pos)
                .zip(&self.node_neg)
                .any(|((constraint, &node_pos), &node_neg)| {
                    constraint.node_pos != node_pos || constraint.node_neg != node_neg
                })
        {
            return Err(CircuitError::InvalidComponent(
                "independent voltage-source topology or value storage changed after finalization"
                    .to_string(),
            ));
        }

        // Stage every projected value before publishing any of them. A bad
        // source, truncated/non-finite solution, or arithmetic overflow thus
        // leaves the caller's complete candidate untouched.
        for component in &projection.components {
            let root = &component.nodes[0];
            root.relative.set(0.0);
            root.signed_target.set(0.0);
            if root.node > 0 {
                let Some(&raw) = solution.get(root.node - 1) else {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage-source projection requires missing node {}",
                        root.node
                    )));
                };
                if !raw.is_finite() {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage-source projection received a non-finite value at node {}",
                        root.node
                    )));
                }
            }

            for node_index in 1..component.nodes.len() {
                let entry = &component.nodes[node_index];
                let parent = entry
                    .parent
                    .expect("finalized non-root projection node has a parent");
                let Some(target) = target_value(parent.source_index) else {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage source '{}' has incomplete value storage",
                        self.names
                            .get(parent.source_index)
                            .map(String::as_str)
                            .unwrap_or("<unknown>")
                    )));
                };
                if !target.is_finite() {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage source '{}' evaluated to a non-finite value",
                        self.names[parent.source_index]
                    )));
                }
                let signed_target = parent.target_sign * target;
                let relative = component.nodes[parent.node_index].relative.get() + signed_target;
                if !signed_target.is_finite() || !relative.is_finite() {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage source '{}' overflowed its component projection",
                        self.names[parent.source_index]
                    )));
                }
                let Some(&raw) = solution.get(entry.node - 1) else {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage-source projection requires missing node {}",
                        entry.node
                    )));
                };
                if !raw.is_finite() {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage-source projection received a non-finite value at node {}",
                        entry.node
                    )));
                }
                entry.signed_target.set(signed_target);
                entry.relative.set(relative);
            }

            let root_voltage = if component.grounded {
                0.0
            } else {
                Self::floating_constraint_common_mode(solution, component)?
            };
            root.projected.set(root_voltage);
            for node_index in 1..component.nodes.len() {
                let entry = &component.nodes[node_index];
                let parent = entry.parent.expect("validated projection parent");
                let projected =
                    component.nodes[parent.node_index].projected.get() + entry.signed_target.get();
                if !projected.is_finite() {
                    return Err(CircuitError::InvalidComponent(format!(
                        "independent voltage-source projection overflowed at node {}",
                        entry.node
                    )));
                }
                entry.projected.set(projected);
            }
        }

        let mut changed = false;
        for component in &projection.components {
            for entry in &component.nodes {
                if entry.node == 0 {
                    continue;
                }
                let value = &mut solution[entry.node - 1];
                let projected = entry.projected.get();
                changed |= projection_changed(*value, projected);
                *value = projected;
            }
        }
        Ok(changed)
    }

    /// Stamp all voltage sources using pre-baked CSC indices
    #[inline]
    pub fn stamp_all_direct(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);
            let v = self.dc_values[i];

            // Stamp matrix entries using pre-baked indices
            if let Some(idx) = self.csc_indices[i][0] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][1] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][2] {
                matrix.stamp_direct(idx, -1.0);
            }
            if let Some(idx) = self.csc_indices[i][3] {
                matrix.stamp_direct(idx, -1.0);
            }

            rhs[br - 1] = v;
        }
    }

    /// Stamp voltage sources with scaled values (for source stepping)
    #[inline]
    pub fn stamp_all_direct_scaled(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        scale: Value,
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);
            let v = self.dc_values[i] * scale;

            if let Some(idx) = self.csc_indices[i][0] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][1] {
                matrix.stamp_direct(idx, 1.0);
            }
            if let Some(idx) = self.csc_indices[i][2] {
                matrix.stamp_direct(idx, -1.0);
            }
            if let Some(idx) = self.csc_indices[i][3] {
                matrix.stamp_direct(idx, -1.0);
            }

            rhs[br - 1] = v;
        }
    }

    /// Stamp all voltage sources
    #[inline]
    pub fn stamp_all(&self, matrix: &mut TripletMatrix, rhs: &mut [Value]) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let br = self.branch_indices[i];
            let v = self.dc_values[i];

            // MNA stamp: add branch equation V(n+) - V(n-) = Vs
            if br > 0 && np > 0 {
                matrix.push(br - 1, np - 1, 1.0);
                matrix.push(np - 1, br - 1, 1.0);
            }
            if br > 0 && nn > 0 {
                matrix.push(br - 1, nn - 1, -1.0);
                matrix.push(nn - 1, br - 1, -1.0);
            }
            if br > 0 {
                rhs[br - 1] = v;
            }
        }
    }

    /// Update voltage source RHS values for transient analysis at time t
    ///
    /// Evaluates time-varying sources (PULSE, SIN, PWL, EXP) at the given time
    /// and updates the RHS vector. Matrix structure is unchanged.
    #[inline]
    pub fn update_transient_rhs(
        &self,
        rhs: &mut [Value],
        time: Value,
        get_branch_idx: impl Fn(usize) -> usize,
    ) {
        let context = self.transient_context;
        for i in 0..self.names.len() {
            let br = get_branch_idx(self.branch_indices[i]);

            let v = match &self.source_specs[i] {
                Some(spec) => Self::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    time,
                    context,
                    self.pwl_waveforms[i].as_deref(),
                ),
                None => self.dc_values[i], // DC only
            };

            rhs[br - 1] = v;
        }
    }

    /// Maximum absolute change expected from time-varying sources over [t0, t1].
    #[inline]
    pub fn max_expected_delta(&self, t0: Value, t1: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .zip(&self.pwl_waveforms)
            .filter_map(|(spec, waveform)| spec.as_ref().map(|spec| (spec, waveform)))
            .map(|(spec, waveform)| {
                (Self::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    t1,
                    context,
                    waveform.as_deref(),
                ) - Self::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    t0,
                    context,
                    waveform.as_deref(),
                ))
                .abs()
            })
            .fold(0.0, Value::max)
    }

    /// Tightest Xyce device-provided timestep ceiling at `time`.
    ///
    /// Xyce 7.10's VSRC device advertises dynamic ceilings for PULSE and SIN
    /// waveforms. Current sources deliberately do not participate because
    /// Xyce's ISRC device does not advertise device maximum timesteps.
    pub(crate) fn xyce_max_timestep_at(&self, time: Value) -> Option<Value> {
        let context = self
            .transient_context
            .filter(|ctx| ctx.dialect == crate::config::SpiceDialect::Xyce)?;

        self.source_specs
            .iter()
            .filter_map(|spec| spec.as_ref())
            .filter_map(|spec| Self::xyce_source_max_timestep_at(spec, time, Some(context)))
            .filter(|step| step.is_finite() && *step > 0.0)
            .reduce(Value::min)
    }

    fn xyce_source_max_timestep_at(
        spec: &crate::netlist::SourceSpec,
        time: Value,
        context: Option<TransientSourceContext>,
    ) -> Option<Value> {
        use crate::netlist::SourceSpec;

        match spec {
            SourceSpec::Distortion { inner, .. } | SourceSpec::RfPort { inner, .. } => {
                Self::xyce_source_max_timestep_at(inner, time, context)
            }
            SourceSpec::DcTransient { transient, .. }
            | SourceSpec::DcAcTransient { transient, .. } => {
                Self::xyce_source_max_timestep_at(transient, time, context)
            }
            SourceSpec::Pulse {
                delay,
                rise,
                fall,
                width,
                period,
                width_defaults_to_zero,
                ..
            } => {
                let (delay, _, _, _, period) = Self::resolve_pulse_timing(
                    *delay,
                    *rise,
                    *fall,
                    *width,
                    *period,
                    *width_defaults_to_zero,
                    context,
                );
                Some(0.1 * if time < delay { delay } else { period })
            }
            SourceSpec::Sin { frequency, .. } => {
                let frequency = Self::resolve_sin_frequency(*frequency, context);
                Some(0.1 / frequency)
            }
            _ => None,
        }
    }

    #[inline]
    pub fn max_dc_to_transient_delta(&self, time: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .enumerate()
            .filter_map(|(idx, spec)| spec.as_ref().map(|spec| (idx, spec)))
            .map(|(idx, spec)| {
                (Self::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    time,
                    context,
                    self.pwl_waveforms[idx].as_deref(),
                ) - self.dc_values[idx])
                    .abs()
            })
            .fold(0.0, Value::max)
    }

    pub(crate) fn load_pwl_waveform_cached_with_limits(
        path: &str,
        time_scale: Value,
        value_scale: Value,
        time_offset: Value,
        value_offset: Value,
        resource_limits: crate::resource::ResourceLimits,
    ) -> Result<Arc<crate::device::pwl_file::PwlWaveform>, crate::device::pwl_file::PwlFileError>
    {
        let key = PwlCacheKey::new(path, time_scale, value_scale, time_offset, value_offset);

        if let Some(file_len) = key.file_len {
            let requested = usize::try_from(file_len).unwrap_or(usize::MAX);
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::ExternalDataBytes,
                requested,
                resource_limits.max_external_data_bytes,
            )?;
        }

        let max_cache_bytes = resource_limits.max_shared_cache_bytes;
        let cached = pwl_waveform_cache().read().ok().and_then(|cache| {
            (cache.retained_bytes() <= max_cache_bytes)
                .then(|| cache.get_cloned(&key))
                .flatten()
        });
        let cached = cached.or_else(|| {
            let mut cache = pwl_waveform_cache().write().ok()?;
            cache.enforce_limit(max_cache_bytes);
            cache.get_cloned(&key)
        });
        if let Some(wf) = cached {
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::ExternalDataValues,
                wf.len().saturating_mul(2),
                resource_limits.max_external_data_values,
            )?;
            return Ok(wf);
        }

        let waveform = crate::device::pwl_file::load_pwl_file_with_limits(path, resource_limits)?
            .with_scaling(time_scale, value_scale, time_offset, value_offset);
        let waveform = Arc::new(waveform);

        if let Ok(mut cache) = pwl_waveform_cache().write() {
            let retained_bytes = pwl_waveform_cache_entry_bytes(&key, &waveform);
            let entry =
                cache.insert_or_get(key, Arc::clone(&waveform), retained_bytes, max_cache_bytes);
            crate::resource::ResourceLimitError::ensure(
                crate::resource::ResourceKind::ExternalDataValues,
                entry.len().saturating_mul(2),
                resource_limits.max_external_data_values,
            )?;
            return Ok(entry);
        }

        Ok(waveform)
    }

    fn log_pwl_error_once(key: PwlCacheKey, msg: &str, max_cache_bytes: usize) {
        if let Ok(mut logged) = pwl_error_log_cache().write() {
            logged.enforce_limit(max_cache_bytes);
            if logged.get(&key).is_none() {
                let retained_bytes = crate::resource::estimated_cache_entry_bytes::<PwlCacheKey, ()>(
                    pwl_key_dynamic_bytes(&key),
                    0,
                );
                logged.insert_or_get(key, (), retained_bytes, max_cache_bytes);
                log::warn!("{}", msg);
            }
            return;
        }
        log::warn!("{}", msg);
    }

    /// Value of an independent source at a point in time, under a dialect.
    ///
    /// Waveform previews and source inspectors need the engine's own
    /// interpretation of `PULSE`/`SIN`/`PWL`, including dialect differences.
    pub fn evaluate_source_spec_at_time_with_dialect(
        spec: &crate::netlist::SourceSpec,
        time: Value,
        tstep: Value,
        tstop: Value,
        dialect: crate::config::SpiceDialect,
    ) -> Value {
        let step = if tstep.is_finite() && tstep > 0.0 {
            tstep
        } else {
            1e-12
        };
        let stop = if tstop.is_finite() && tstop > 0.0 {
            tstop
        } else {
            1e99
        };
        Self::evaluate_source_at_time_with_context(
            spec,
            time,
            Some(TransientSourceContext {
                tstep: step,
                tstop: stop,
                dialect,
                resource_limits: crate::resource::ResourceLimits::default(),
            }),
        )
    }

    #[inline]
    fn pulse_step_default(context: Option<TransientSourceContext>) -> Value {
        context.map(|ctx| ctx.tstep).unwrap_or(1e-12).max(1e-18)
    }

    #[inline]
    fn pulse_stop_default(context: Option<TransientSourceContext>) -> Value {
        context.map(|ctx| ctx.tstop).unwrap_or(1e99).max(1e-18)
    }

    #[inline]
    fn pulse_dialect(context: Option<TransientSourceContext>) -> crate::config::SpiceDialect {
        context
            .map(|ctx| ctx.dialect)
            .unwrap_or(crate::config::SpiceDialect::BestAvailable)
    }

    #[inline]
    fn sin_frequency_default(context: Option<TransientSourceContext>) -> Value {
        context
            .map(|ctx| ctx.tstop)
            .filter(|tstop| tstop.is_finite() && *tstop > 0.0)
            .map(|tstop| 1.0 / tstop)
            .unwrap_or(1e3)
    }

    #[inline]
    fn resolve_sin_frequency(frequency: Value, context: Option<TransientSourceContext>) -> Value {
        if frequency.is_finite() && frequency != 0.0 {
            frequency
        } else {
            Self::sin_frequency_default(context)
        }
    }

    /// ngspice's analysis-scaled frequency defaults for SFFM/AM: an omitted
    /// frequency becomes `cycles / tstop` (vsrcload.c uses 5 and 500).
    #[inline]
    fn modulated_frequency_default(
        cycles: Value,
        context: Option<TransientSourceContext>,
    ) -> Value {
        context
            .map(|ctx| ctx.tstop)
            .filter(|tstop| tstop.is_finite() && *tstop > 0.0)
            .map(|tstop| cycles / tstop)
            .unwrap_or(cycles * 1e3)
    }

    /// Xyce SFFM defaults omitted FC and FS to one cycle over the transient
    /// stop time.
    #[inline]
    fn xyce_modulated_frequency_default(context: Option<TransientSourceContext>) -> Value {
        context
            .map(|ctx| ctx.tstop)
            .filter(|tstop| tstop.is_finite() && *tstop > 0.0)
            .map(|tstop| 1.0 / tstop)
            .unwrap_or(0.0)
    }

    #[inline]
    fn resolve_pulse_timing(
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        width_defaults_to_zero: bool,
        context: Option<TransientSourceContext>,
    ) -> (Value, Value, Value, Value, Value) {
        Self::resolve_pulse_timing_with_defaults(
            delay,
            rise,
            fall,
            width,
            period,
            width_defaults_to_zero,
            Self::pulse_step_default(context),
            Self::pulse_stop_default(context),
            Self::pulse_dialect(context),
        )
    }

    /// Resolve PULSE timing fields against explicit tstep/tstop defaults.
    /// Shared with breakpoint scheduling so accepted timesteps land on the
    /// same edges the waveform actually produces.
    #[inline]
    #[allow(clippy::too_many_arguments)]
    /// `PULSE` timing with dialect defaults substituted for omitted fields.
    pub fn resolve_pulse_timing_with_defaults(
        delay: Value,
        rise: Value,
        fall: Value,
        width: Value,
        period: Value,
        width_defaults_to_zero: bool,
        step_default: Value,
        stop_default: Value,
        dialect: crate::config::SpiceDialect,
    ) -> (Value, Value, Value, Value, Value) {
        let period_was_omitted = period.is_nan();
        let width_was_omitted = width.is_nan();
        let xyce_defaults = matches!(dialect, crate::config::SpiceDialect::Xyce);
        let ngspice_defaults = matches!(dialect, crate::config::SpiceDialect::Ngspice);
        // Only Xyce resolves an omitted PER to the transient stop. ngspice 46
        // resolves it to TR + PW + TF, so the waveform repeats for the rest of
        // the run rather than firing once. Measured against ngspice 46 on
        // `tests/ngspice/general/rtlinv.cir`, whose source is
        // `pulse(0 5 2ns 2ns 2ns 80ns)`: v(1) rises at 3ns, falls at 85ns,
        // and rises again at 88ns and 172ns — a period of tr+pw+tf = 84ns,
        // not a single pulse held low to tstop.
        let stop_time_defaults = xyce_defaults;

        let td = if delay.is_finite() {
            delay.max(0.0)
        } else {
            0.0
        };
        let tr = if rise.is_nan() { step_default } else { rise };
        let tf = if fall.is_nan() { step_default } else { fall };
        let pw = if width.is_nan() && xyce_defaults {
            stop_default
        } else if width.is_nan() && ngspice_defaults && !width_defaults_to_zero {
            stop_default
        } else if width.is_nan() {
            0.0
        } else {
            width
        };
        let per = if period.is_nan() { 0.0 } else { period };

        let tr = if tr.is_finite() && tr > 0.0 {
            tr
        } else {
            step_default
        };
        let tf = if tf.is_finite() && tf > 0.0 {
            tf
        } else {
            step_default
        };
        let pw = if pw.is_finite() && pw >= 0.0 {
            pw
        } else if width_was_omitted && xyce_defaults {
            stop_default
        } else {
            0.0
        };
        let per = if period_was_omitted {
            if stop_time_defaults {
                stop_default
            } else {
                tr + pw + tf
            }
        } else if per.is_finite() && per > 0.0 {
            per
        } else {
            tr + pw + tf
        };

        (td, tr, tf, pw, per)
    }

    /// ngspice's EXP timing defaults (vsrcload.c): TD1, TAU1, and TAU2
    /// fall back to the transient step when omitted *or zero*, TD2 to
    /// TD1 + step.
    #[inline]
    fn resolve_exp_timing(
        td1: Value,
        tau1: Value,
        td2: Value,
        tau2: Value,
        context: Option<TransientSourceContext>,
    ) -> (Value, Value, Value, Value) {
        let step = Self::pulse_step_default(context);
        let td1 = if td1.is_finite() && td1 != 0.0 {
            td1
        } else {
            step
        };
        let tau1 = if tau1.is_finite() && tau1 != 0.0 {
            tau1
        } else {
            step
        };
        let td2 = if td2.is_finite() && td2 != 0.0 {
            td2
        } else {
            td1 + step
        };
        let tau2 = if tau2.is_finite() && tau2 != 0.0 {
            tau2
        } else {
            step
        };
        (td1, tau1, td2, tau2)
    }

    #[inline]
    fn evaluate_source_at_time_with_context(
        spec: &crate::netlist::SourceSpec,
        time: Value,
        context: Option<TransientSourceContext>,
    ) -> Value {
        Self::evaluate_source_at_time_with_context_and_pwl(spec, time, context, None)
    }

    #[inline]
    fn evaluate_source_at_time_with_context_and_pwl(
        spec: &crate::netlist::SourceSpec,
        time: Value,
        context: Option<TransientSourceContext>,
        pwl_waveform: Option<&crate::device::pwl_file::PwlWaveform>,
    ) -> Value {
        use crate::netlist::SourceSpec;
        use std::f64::consts::PI;

        match spec {
            SourceSpec::Distortion { inner, .. } => {
                Self::evaluate_source_at_time_with_context_and_pwl(
                    inner,
                    time,
                    context,
                    pwl_waveform,
                )
            }
            SourceSpec::RfPort { inner, .. } => Self::evaluate_source_at_time_with_context_and_pwl(
                inner,
                time,
                context,
                pwl_waveform,
            ),
            SourceSpec::Dc(v) => *v,
            SourceSpec::Ac { .. } => 0.0, // AC sources are DC=0 in transient
            // TRNOISE expands into a PWL sample train before circuit
            // construction; an unexpanded spec is zero-mean by definition.
            SourceSpec::TrNoise { .. } => 0.0,
            SourceSpec::DcAc { dc_value, .. } => *dc_value,
            SourceSpec::DcTransient { transient, .. } => {
                Self::evaluate_source_at_time_with_context_and_pwl(
                    transient,
                    time,
                    context,
                    pwl_waveform,
                )
            }
            SourceSpec::DcAcTransient { transient, .. } => {
                Self::evaluate_source_at_time_with_context_and_pwl(
                    transient,
                    time,
                    context,
                    pwl_waveform,
                )
            }
            SourceSpec::Pulse {
                v1,
                v2,
                delay,
                rise,
                fall,
                width,
                period,
                phase,
                width_defaults_to_zero,
            } => {
                let (delay, rise, fall, width, period) = Self::resolve_pulse_timing(
                    *delay,
                    *rise,
                    *fall,
                    *width,
                    *period,
                    *width_defaults_to_zero,
                    context,
                );
                if time < delay {
                    return *v1;
                }
                let phase_time = if period.is_finite() && period > 0.0 {
                    let phase_cycles = (phase / 360.0).rem_euclid(1.0);
                    if phase_cycles > 0.0 {
                        phase_cycles * period - period
                    } else {
                        0.0
                    }
                } else {
                    0.0
                };
                let t_rel = time - delay + phase_time;
                let t = if period.is_finite() && period > 0.0 && t_rel > period {
                    t_rel - period * (t_rel / period).floor()
                } else {
                    t_rel
                };
                if t <= 0.0 || t >= rise + width + fall {
                    *v1
                } else if t < rise {
                    v1 + (v2 - v1) * t / rise
                } else if t < rise + width {
                    *v2
                } else if t < rise + width + fall {
                    v2 + (v1 - v2) * (t - rise - width) / fall
                } else {
                    *v1
                }
            }
            SourceSpec::Sin {
                offset,
                amplitude,
                frequency,
                delay,
                damping,
                phase,
            } => {
                let frequency = Self::resolve_sin_frequency(*frequency, context);
                if time < *delay {
                    // ngspice holds VO + VA*sin(PHASE) before the delay,
                    // not the bare offset (vsrcload.c).
                    offset + amplitude * phase.sin()
                } else {
                    let t = time - delay;
                    offset
                        + amplitude
                            * (-damping * t).exp()
                            * (2.0 * PI * frequency * t + phase).sin()
                }
            }
            SourceSpec::Pwl {
                points,
                delay,
                repeat_from,
            } => Self::evaluate_pwl_points(points, time, *delay, *repeat_from),
            SourceSpec::PwlFile {
                path,
                time_scale,
                value_scale,
                time_offset,
                value_offset,
                delay,
                repeat_from,
            } => {
                if let Some(waveform) = pwl_waveform {
                    return if time < *delay {
                        0.0
                    } else {
                        waveform.value_at_repeating(time - *delay, *repeat_from)
                    };
                }
                let key =
                    PwlCacheKey::new(path, *time_scale, *value_scale, *time_offset, *value_offset);
                let resource_limits = context
                    .map(|context| context.resource_limits)
                    .unwrap_or_default();
                match Self::load_pwl_waveform_cached_with_limits(
                    path,
                    *time_scale,
                    *value_scale,
                    *time_offset,
                    *value_offset,
                    resource_limits,
                ) {
                    Ok(waveform) => {
                        if time < *delay {
                            0.0
                        } else {
                            waveform.value_at_repeating(time - *delay, *repeat_from)
                        }
                    }
                    Err(err) => {
                        let message = format!("failed to load PWL file '{path}': {err}");
                        Self::log_pwl_error_once(
                            key,
                            &message,
                            resource_limits.max_shared_cache_bytes,
                        );
                        *value_offset
                    }
                }
            }
            SourceSpec::Pat {
                vhi,
                vlo,
                delay,
                rise,
                fall,
                sample,
                data,
                repeat_count,
            } => Self::evaluate_pat_source(
                *vhi,
                *vlo,
                *delay,
                *rise,
                *fall,
                *sample,
                data,
                *repeat_count,
                time,
            ),
            SourceSpec::Exp {
                v1,
                v2,
                td1,
                tau1,
                td2,
                tau2,
            } => {
                let (td1, tau1, td2, tau2) =
                    Self::resolve_exp_timing(*td1, *tau1, *td2, *tau2, context);
                if time <= td1 {
                    *v1
                } else if time <= td2 {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                } else {
                    v1 + (v2 - v1) * (1.0 - (-(time - td1) / tau1).exp())
                        - (v2 - v1) * (1.0 - (-(time - td2) / tau2).exp())
                }
            }
            SourceSpec::Sffm {
                offset,
                amplitude,
                carrier_freq,
                modulation_index,
                signal_freq,
                delay,
                phase_modulation,
                phase_carrier,
            } => {
                if matches!(
                    Self::pulse_dialect(context),
                    crate::config::SpiceDialect::Xyce
                ) {
                    let fc = if carrier_freq.is_finite() {
                        *carrier_freq
                    } else {
                        Self::xyce_modulated_frequency_default(context)
                    };
                    let fs = if signal_freq.is_finite() {
                        *signal_freq
                    } else {
                        Self::xyce_modulated_frequency_default(context)
                    };
                    let mdi = if modulation_index.is_finite() {
                        *modulation_index
                    } else {
                        0.0
                    };
                    return *offset
                        + *amplitude
                            * ((2.0 * PI * fc * time) + mdi * (2.0 * PI * fs * time).sin()).sin();
                }

                // ngspice vsrcload.c SFFM semantics, including the exact
                // omitted-parameter defaults and the MDI clamp.
                let fc = if carrier_freq.is_finite() && *carrier_freq > 0.0 {
                    *carrier_freq
                } else {
                    Self::modulated_frequency_default(5.0, context)
                };
                let fm = if signal_freq.is_finite() && *signal_freq != 0.0 {
                    *signal_freq
                } else {
                    Self::modulated_frequency_default(500.0, context)
                };
                let mdi = if modulation_index.is_finite() {
                    modulation_index.clamp(0.0, fc / fm)
                } else {
                    90.0_f64.min(fc / fm)
                };
                let t = time - delay;
                if t <= 0.0 {
                    0.0
                } else {
                    let phasec = phase_carrier.to_radians();
                    let phasem = phase_modulation.to_radians();
                    offset
                        + amplitude
                            * ((2.0 * PI * fc * t + phasec)
                                + mdi * (2.0 * PI * fm * t + phasem).sin())
                            .sin()
                }
            }
            SourceSpec::Am {
                offset,
                modulation_offset,
                modulation_amplitude,
                modulating_freq,
                carrier_freq,
                delay,
                phase_modulation,
                phase_carrier,
            } => {
                // ngspice vsrcload.c AM semantics.
                let fm = if modulating_freq.is_finite() && *modulating_freq > 0.0 {
                    *modulating_freq
                } else {
                    Self::modulated_frequency_default(5.0, context)
                };
                let fc = if carrier_freq.is_finite() && *carrier_freq > 0.0 {
                    *carrier_freq
                } else {
                    Self::modulated_frequency_default(500.0, context)
                };
                let t = time - delay;
                if t <= 0.0 {
                    0.0
                } else {
                    let phasec = phase_carrier.to_radians();
                    let phasem = phase_modulation.to_radians();
                    offset
                        + (modulation_offset
                            + modulation_amplitude * (2.0 * PI * fm * t + phasem).sin())
                            * (2.0 * PI * fc * t + phasec).sin()
                }
            }
        }
    }

    /// Enforce voltage source constraints on solution vector after force-accept
    ///
    /// When Newton iteration fails to converge and we force-accept a solution,
    /// the voltage source node values may not satisfy V(n+) - V(n-) = Vs.
    /// This method corrects the solution vector to enforce this constraint
    /// for display purposes and to prevent drift.
    pub fn enforce_voltage_constraints(
        &self,
        solution: &mut [Value],
        time: Value,
    ) -> Result<bool, CircuitError> {
        self.project_constraint_components(solution, |source_index| {
            let dc_value = self.dc_values.get(source_index).copied()?;
            Some(match self.source_specs.get(source_index)? {
                Some(spec) => Self::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    time,
                    self.transient_context,
                    self.pwl_waveforms
                        .get(source_index)
                        .and_then(Option::as_deref),
                ),
                None => dc_value,
            })
        })
    }

    /// Enforce operating-point voltage-source constraints.
    ///
    /// Combined sources such as `DC 1.2 AC 1 SIN(...)` use their explicit DC
    /// value for OP/DC analyses; transient waveform evaluation is reserved for
    /// time-domain projection.
    pub fn enforce_dc_voltage_constraints(
        &self,
        solution: &mut [Value],
    ) -> Result<bool, CircuitError> {
        self.project_constraint_components(solution, |source_index| {
            self.dc_values.get(source_index).copied()
        })
    }

    /// Enforce the scaled DC voltage-source constraints used by source stepping.
    pub fn enforce_scaled_dc_voltage_constraints(
        &self,
        solution: &mut [Value],
        scale: Value,
    ) -> Result<bool, CircuitError> {
        let scale = if scale.is_finite() { scale } else { 1.0 };
        self.project_constraint_components(solution, |source_index| {
            self.dc_values.get(source_index).map(|value| value * scale)
        })
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn evaluate_pat_source(
        vhi: Value,
        vlo: Value,
        delay: Value,
        rise: Value,
        fall: Value,
        sample: Value,
        data: &str,
        repeat_count: i32,
        time: Value,
    ) -> Value {
        let Some((first_bit, last_bit, bit_count)) = Self::pat_data_shape(data) else {
            return 0.0;
        };
        if ![vhi, vlo, delay, rise, fall, sample, time]
            .into_iter()
            .all(Value::is_finite)
            || rise <= 0.0
            || fall <= 0.0
            || sample <= 0.0
        {
            return 0.0;
        }

        let first_value = if first_bit == b'1' { vhi } else { vlo };
        let first_plateau_time = if first_bit == b'1' {
            0.5 * rise
        } else {
            0.5 * fall
        };
        let pattern_duration = bit_count as Value * sample;
        let second_last_time = pattern_duration
            - if last_bit == b'1' {
                0.5 * fall
            } else {
                0.5 * rise
            };
        let second_last_value = if last_bit == b'1' { vhi } else { vlo };
        let last_value = if first_bit == last_bit {
            second_last_value
        } else {
            0.5 * (vhi - vlo)
        };

        let mut source_time = time - delay;
        if source_time <= first_plateau_time {
            return first_value;
        }

        if repeat_count >= 0
            && source_time >= repeat_count as Value * pattern_duration + second_last_time
        {
            return second_last_value;
        }

        if source_time > pattern_duration {
            source_time -= pattern_duration;
            source_time -= pattern_duration * (source_time / pattern_duration).floor();
            if source_time == 0.0 {
                return last_value;
            }
        } else if source_time == pattern_duration {
            return last_value;
        }

        Self::interpolate_pat_points(vhi, vlo, rise, fall, sample, data, source_time, last_value)
    }

    #[allow(clippy::too_many_arguments)]
    fn interpolate_pat_points(
        vhi: Value,
        vlo: Value,
        rise: Value,
        fall: Value,
        sample: Value,
        data: &str,
        time: Value,
        last_value: Value,
    ) -> Value {
        let mut previous: Option<(Value, Value)> = None;
        let mut result = last_value;
        let mut found = false;
        Self::visit_pat_points(
            vhi,
            vlo,
            rise,
            fall,
            sample,
            data,
            |point_time, point_value| {
                if found {
                    return;
                }
                if time < point_time {
                    result = if let Some((time1, value1)) = previous {
                        let dt = point_time - time1;
                        if dt == 0.0 {
                            point_value
                        } else {
                            (point_time - time) * value1 / dt + (time - time1) * point_value / dt
                        }
                    } else {
                        point_value
                    };
                    found = true;
                } else {
                    previous = Some((point_time, point_value));
                }
            },
        );
        result
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn visit_pat_points<F>(
        vhi: Value,
        vlo: Value,
        rise: Value,
        fall: Value,
        sample: Value,
        data: &str,
        mut visit: F,
    ) where
        F: FnMut(Value, Value),
    {
        let bytes = data.as_bytes();
        let Some((first_bit, last_bit, bit_count)) = Self::pat_data_shape(data) else {
            return;
        };

        if first_bit == b'1' {
            let initial = if last_bit == b'0' {
                0.5 * (vhi - vlo)
            } else {
                vhi
            };
            visit(0.0, initial);
            visit(0.5 * rise, vhi);
        } else {
            let initial = if last_bit == b'0' {
                vlo
            } else {
                0.5 * (vhi - vlo)
            };
            visit(0.0, initial);
            visit(0.5 * fall, vlo);
        }

        for bit_index in 2..=bit_count {
            let current = bytes[bit_index];
            let previous = bytes[bit_index - 1];
            if current == previous {
                continue;
            }
            let boundary = (bit_index - 1) as Value * sample;
            if current == b'0' {
                visit(boundary - 0.5 * fall, vhi);
                visit(boundary + 0.5 * fall, vlo);
            } else {
                visit(boundary - 0.5 * rise, vlo);
                visit(boundary + 0.5 * rise, vhi);
            }
        }

        let pattern_duration = bit_count as Value * sample;
        if last_bit == b'1' {
            visit(pattern_duration - 0.5 * fall, vhi);
            visit(
                pattern_duration,
                if first_bit == last_bit {
                    vhi
                } else {
                    0.5 * (vhi - vlo)
                },
            );
        } else {
            visit(pattern_duration - 0.5 * rise, vlo);
            visit(
                pattern_duration,
                if first_bit == last_bit {
                    vlo
                } else {
                    0.5 * (vhi - vlo)
                },
            );
        }
    }

    pub(crate) fn pat_pattern_duration(data: &str, sample: Value) -> Option<Value> {
        let (_, _, bit_count) = Self::pat_data_shape(data)?;
        (sample.is_finite() && sample > 0.0).then_some(bit_count as Value * sample)
    }

    fn pat_data_shape(data: &str) -> Option<(u8, u8, usize)> {
        let bytes = data.as_bytes();
        if bytes.len() < 2 || !matches!(bytes[0], b'B' | b'b') {
            return None;
        }
        if !bytes[1..].iter().all(|bit| matches!(bit, b'0' | b'1')) {
            return None;
        }
        Some((bytes[1], *bytes.last()?, bytes.len() - 1))
    }

    fn evaluate_pwl_points(
        points: &[(Value, Value)],
        time: Value,
        delay: Value,
        repeat_from: Option<Value>,
    ) -> Value {
        if points.is_empty() {
            return 0.0;
        }
        if time < delay {
            return 0.0;
        }
        let shifted_time = time - delay;
        if shifted_time <= points[0].0 {
            return points[0].1;
        }
        let time = Self::repeat_pwl_time(points, shifted_time, repeat_from);
        if time >= points[points.len() - 1].0 {
            return points[points.len() - 1].1;
        }
        for window in points.windows(2) {
            let (t1, v1) = window[0];
            let (t2, v2) = window[1];
            if time >= t1 && time < t2 {
                let dt = t2 - t1;
                if !dt.is_finite() || dt.abs() <= Value::EPSILON {
                    return v1;
                }
                return v1 + (v2 - v1) * (time - t1) / dt;
            }
        }
        points.last().map(|(_, value)| *value).unwrap_or(0.0)
    }

    fn repeat_pwl_time(
        points: &[(Value, Value)],
        time: Value,
        repeat_from: Option<Value>,
    ) -> Value {
        let Some(repeat_from) = repeat_from else {
            return time;
        };
        let Some(&(first, _)) = points.first() else {
            return time;
        };
        let Some(&(last, _)) = points.last() else {
            return time;
        };
        if !time.is_finite() || !repeat_from.is_finite() || time <= last {
            return time;
        }
        let repeat_start = repeat_from.max(first);
        if repeat_start >= last {
            return time;
        }
        let period = last - repeat_start;
        if !period.is_finite() || period <= Value::EPSILON {
            return time;
        }
        let elapsed = time - repeat_start;
        let remainder = elapsed.rem_euclid(period);
        let boundary_tolerance = Value::EPSILON * elapsed.abs().max(period).max(1.0);
        if remainder <= boundary_tolerance {
            return last;
        }
        repeat_start + remainder
    }
}

/// Current source storage (SoA)
#[derive(Debug, Default, Clone)]
pub struct CurrentSources {
    pub names: Vec<String>,
    pub node_pos: Vec<NodeId>,
    pub node_neg: Vec<NodeId>,
    pub dc_values: Vec<Value>,
    /// AC magnitude for HB/AC analysis
    pub ac_magnitudes: Vec<Value>,
    /// AC phase in radians for HB/AC analysis
    pub ac_phases: Vec<Value>,
    /// Full source specification for transient waveform evaluation
    pub source_specs: Vec<Option<crate::netlist::SourceSpec>>,
    /// Circuit-owned external PWL snapshots aligned with `source_specs`.
    pwl_waveforms: Vec<Option<Arc<crate::device::pwl_file::PwlWaveform>>>,
    /// Optional transient context used to resolve source defaults.
    transient_context: Option<TransientSourceContext>,
}

impl CurrentSources {
    #[inline]
    fn finite_dc_value(&self, index: usize) -> Value {
        let value = self.dc_values[index];
        if value.is_finite() { value } else { 0.0 }
    }

    pub fn index_by_name(&self, name: &str) -> Option<usize> {
        self.names
            .iter()
            .position(|source_name| source_name.eq_ignore_ascii_case(name))
    }

    pub fn value_at_time(&self, index: usize, time: Value) -> Value {
        let Some(dc_value) = self.dc_values.get(index).copied() else {
            return 0.0;
        };
        let dc_value = if dc_value.is_finite() { dc_value } else { 0.0 };
        match self.source_specs.get(index).and_then(Option::as_ref) {
            Some(spec) => VoltageSources::evaluate_source_at_time_with_context_and_pwl(
                spec,
                time,
                self.transient_context,
                self.pwl_waveforms[index].as_deref(),
            ),
            None => dc_value,
        }
    }

    pub fn values_at_time(&self, time: Value) -> Vec<Value> {
        (0..self.names.len())
            .map(|index| self.value_at_time(index, time))
            .collect()
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, name: String, node_pos: NodeId, node_neg: NodeId, dc_value: Value) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(0.0);
        self.ac_phases.push(0.0);
        self.source_specs.push(None);
        self.pwl_waveforms.push(None);
    }

    /// Add current source with AC and transient specification.
    pub fn add_with_ac_and_spec(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
    ) {
        self.add_with_ac_spec_and_pwl_waveform(
            name,
            node_pos,
            node_neg,
            dc_value,
            ac_magnitude,
            ac_phase,
            source_spec,
            None,
        );
    }

    pub(crate) fn add_with_ac_spec_and_pwl_waveform(
        &mut self,
        name: String,
        node_pos: NodeId,
        node_neg: NodeId,
        dc_value: Value,
        ac_magnitude: Value,
        ac_phase: Value,
        source_spec: Option<crate::netlist::SourceSpec>,
        pwl_waveform: Option<Arc<crate::device::pwl_file::PwlWaveform>>,
    ) {
        self.names.push(name);
        self.node_pos.push(node_pos);
        self.node_neg.push(node_neg);
        self.dc_values.push(dc_value);
        self.ac_magnitudes.push(ac_magnitude);
        self.ac_phases.push(ac_phase);
        self.source_specs.push(source_spec);
        self.pwl_waveforms.push(pwl_waveform);
    }

    /// Set transient context used to resolve waveform defaults.
    pub fn set_transient_context(&mut self, tstep: Value, tstop: Value) {
        self.set_transient_context_with_dialect(
            tstep,
            tstop,
            crate::config::SpiceDialect::BestAvailable,
        );
    }

    /// Set transient context used to resolve waveform defaults for a SPICE dialect.
    pub fn set_transient_context_with_dialect(
        &mut self,
        tstep: Value,
        tstop: Value,
        dialect: crate::config::SpiceDialect,
    ) {
        self.set_transient_context_with_dialect_and_limits(
            tstep,
            tstop,
            dialect,
            crate::resource::ResourceLimits::default(),
        );
    }

    pub(crate) fn set_transient_context_with_dialect_and_limits(
        &mut self,
        tstep: Value,
        tstop: Value,
        dialect: crate::config::SpiceDialect,
        resource_limits: crate::resource::ResourceLimits,
    ) {
        let step = if tstep.is_finite() && tstep > 0.0 {
            tstep
        } else {
            1e-12
        };
        let stop = if tstop.is_finite() && tstop > 0.0 {
            tstop
        } else {
            1e99
        };
        self.transient_context = Some(TransientSourceContext {
            tstep: step,
            tstop: stop,
            dialect,
            resource_limits,
        });
    }

    /// Clear transient context and use static waveform defaults.
    pub fn clear_transient_context(&mut self) {
        self.transient_context = None;
    }

    pub fn len(&self) -> usize {
        self.names.len()
    }

    pub fn is_empty(&self) -> bool {
        self.names.is_empty()
    }

    pub(crate) fn freeze_transient_source_at_time(
        &mut self,
        name: &str,
        time: Value,
    ) -> Option<Value> {
        let index = self.index_by_name(name)?;
        let value = self.value_at_time(index, time);
        self.dc_values[index] = value;
        // See the voltage-source implementation above: a frozen source must
        // not retain a separate AC/HB drive contract.
        self.ac_magnitudes[index] = 0.0;
        self.ac_phases[index] = 0.0;
        self.source_specs[index] = None;
        self.pwl_waveforms[index] = None;
        Some(value)
    }

    /// Iterate transient specifications with their canonical source names and
    /// circuit-owned PWL snapshots.
    pub(crate) fn transient_specs_named_with_pwl(
        &self,
    ) -> impl Iterator<
        Item = (
            &str,
            &crate::netlist::SourceSpec,
            Option<&crate::device::pwl_file::PwlWaveform>,
        ),
    > {
        self.names
            .iter()
            .zip(&self.source_specs)
            .zip(&self.pwl_waveforms)
            .filter_map(|((name, spec), waveform)| {
                spec.as_ref()
                    .map(|spec| (name.as_str(), spec, waveform.as_deref()))
            })
    }

    /// Stamp all current sources
    #[inline]
    pub fn stamp_all(&self, rhs: &mut [Value]) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let current = self.finite_dc_value(i);

            if np > 0 {
                rhs[np - 1] -= current;
            }
            if nn > 0 {
                rhs[nn - 1] += current;
            }
        }
    }

    /// Stamp current sources with scaled values (for source stepping)
    #[inline]
    pub fn stamp_all_scaled(&self, rhs: &mut [Value], scale: Value) {
        for i in 0..self.names.len() {
            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            let current = self.finite_dc_value(i) * scale;

            if np > 0 {
                rhs[np - 1] -= current;
            }
            if nn > 0 {
                rhs[nn - 1] += current;
            }
        }
    }

    /// Update RHS contribution of time-varying current sources at transient time.
    ///
    /// `stamp_dc_direct` already stamped DC values, so this applies only the
    /// delta between waveform and DC.
    #[inline]
    pub fn update_transient_rhs(&self, rhs: &mut [Value], time: Value) {
        for i in 0..self.names.len() {
            let Some(spec) = self.source_specs[i].as_ref() else {
                continue;
            };

            let value = VoltageSources::evaluate_source_at_time_with_context_and_pwl(
                spec,
                time,
                self.transient_context,
                self.pwl_waveforms[i].as_deref(),
            );
            let delta = value - self.finite_dc_value(i);
            if !delta.is_finite() || delta == 0.0 {
                continue;
            }

            let np = self.node_pos[i];
            let nn = self.node_neg[i];
            if np > 0 {
                rhs[np - 1] -= delta;
            }
            if nn > 0 {
                rhs[nn - 1] += delta;
            }
        }
    }

    /// Maximum absolute change expected from time-varying current sources over [t0, t1].
    #[inline]
    pub fn max_expected_delta(&self, t0: Value, t1: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .zip(&self.pwl_waveforms)
            .filter_map(|(spec, waveform)| spec.as_ref().map(|spec| (spec, waveform)))
            .map(|(spec, waveform)| {
                (VoltageSources::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    t1,
                    context,
                    waveform.as_deref(),
                ) - VoltageSources::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    t0,
                    context,
                    waveform.as_deref(),
                ))
                .abs()
            })
            .fold(0.0, Value::max)
    }

    #[inline]
    pub fn max_dc_to_transient_delta(&self, time: Value) -> Value {
        let context = self.transient_context;
        self.source_specs
            .iter()
            .enumerate()
            .filter_map(|(idx, spec)| spec.as_ref().map(|spec| (idx, spec)))
            .map(|(idx, spec)| {
                (VoltageSources::evaluate_source_at_time_with_context_and_pwl(
                    spec,
                    time,
                    context,
                    self.pwl_waveforms[idx].as_deref(),
                ) - self.finite_dc_value(idx))
                .abs()
            })
            .fold(0.0, Value::max)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::netlist::SourceSpec;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn assert_close(actual: Value, expected: Value) {
        let tolerance = expected.abs().max(1.0) * 1.0e-12;
        assert!(
            (actual - expected).abs() <= tolerance,
            "actual={actual:.17e} expected={expected:.17e} tolerance={tolerance:.17e}"
        );
    }

    #[test]
    fn circuit_owned_external_pwl_snapshot_does_not_reopen_the_source_file() {
        let waveform = Arc::new(
            crate::device::pwl_file::PwlWaveform::new(vec![(0.0, 1.0), (1.0, 3.0)])
                .expect("valid waveform"),
        );
        let spec = SourceSpec::PwlFile {
            path: "this-file-must-not-be-opened-during-evaluation.pwl".to_string(),
            time_scale: 1.0,
            value_scale: 1.0,
            time_offset: 0.0,
            value_offset: 0.0,
            delay: 0.0,
            repeat_from: None,
        };
        let mut sources = CurrentSources::new();
        sources.add_with_ac_spec_and_pwl_waveform(
            "I1".to_string(),
            1,
            0,
            1.0,
            0.0,
            0.0,
            Some(spec),
            Some(waveform),
        );

        assert_close(sources.value_at_time(0, 0.5), 2.0);
        assert_close(sources.max_expected_delta(0.25, 0.75), 1.0);
        assert_close(sources.max_dc_to_transient_delta(0.5), 1.0);
    }

    #[test]
    fn pwl_loader_honors_zero_shared_cache_retention() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let path = std::env::temp_dir().join(format!("rspice-pwl-cache-{unique}.csv"));
        std::fs::write(&path, "0 0\n1 1\n").expect("write PWL cache fixture");
        let path_text = path.to_string_lossy().into_owned();
        let mut limits = crate::resource::ResourceLimits::default();
        limits.max_shared_cache_bytes = 0;

        let waveform = VoltageSources::load_pwl_waveform_cached_with_limits(
            &path_text, 1.0, 1.0, 0.0, 0.0, limits,
        )
        .expect("zero-retention policy still returns the parsed waveform");
        assert_eq!(waveform.len(), 2);
        let key = PwlCacheKey::new(&path_text, 1.0, 1.0, 0.0, 0.0);
        let cache = pwl_waveform_cache().read().expect("read PWL cache");
        assert!(cache.get(&key).is_none());

        let _ = std::fs::remove_file(path);
    }

    fn transient_context(tstep: Value, tstop: Value) -> Option<TransientSourceContext> {
        Some(TransientSourceContext {
            tstep,
            tstop,
            dialect: crate::config::SpiceDialect::BestAvailable,
            resource_limits: crate::resource::ResourceLimits::default(),
        })
    }

    fn ngspice_transient_context(tstep: Value, tstop: Value) -> Option<TransientSourceContext> {
        Some(TransientSourceContext {
            tstep,
            tstop,
            dialect: crate::config::SpiceDialect::Ngspice,
            resource_limits: crate::resource::ResourceLimits::default(),
        })
    }

    fn xyce_transient_context(tstep: Value, tstop: Value) -> Option<TransientSourceContext> {
        Some(TransientSourceContext {
            tstep,
            tstop,
            dialect: crate::config::SpiceDialect::Xyce,
            resource_limits: crate::resource::ResourceLimits::default(),
        })
    }

    #[test]
    fn dc_constraint_projection_uses_dc_value_for_combined_transient_source() {
        let mut sources = VoltageSources::new();
        sources.add_with_ac_and_spec(
            "vin".to_string(),
            1,
            0,
            1,
            1.44,
            0.1,
            0.0,
            Some(SourceSpec::DcAcTransient {
                dc_value: 1.44,
                ac_magnitude: 0.1,
                ac_phase: 0.0,
                transient: Box::new(SourceSpec::Sin {
                    offset: 0.0,
                    amplitude: 1.0,
                    frequency: 1.0e5,
                    delay: 0.0,
                    damping: 0.0,
                    phase: 0.0,
                }),
            }),
        );
        sources
            .finalize_constraint_projection(1)
            .expect("single-source projection topology finalizes");

        let mut dc_solution = vec![0.0];
        assert!(
            sources
                .enforce_dc_voltage_constraints(&mut dc_solution)
                .expect("DC projection succeeds")
        );
        assert_close(dc_solution[0], 1.44);

        let mut transient_solution = vec![1.44];
        assert!(
            sources
                .enforce_voltage_constraints(&mut transient_solution, 0.0)
                .expect("transient projection succeeds")
        );
        assert_close(transient_solution[0], 0.0);
    }

    #[test]
    fn grounded_source_stack_projects_every_constraint_exactly() {
        for reverse_order_and_orientation in [false, true] {
            let mut sources = VoltageSources::new();
            if reverse_order_and_orientation {
                sources.add("vmon".to_string(), 2, 1, 2, 0.0);
                sources.add("vdrive".to_string(), 1, 0, 1, -4.0);
            } else {
                sources.add("vdrive".to_string(), 1, 0, 1, -4.0);
                sources.add("vmon".to_string(), 1, 2, 2, 0.0);
            }
            sources
                .finalize_constraint_projection(2)
                .expect("stacked source topology finalizes");

            let mut solution = [-3.999_999_999_998_709, -4.000_000_000_001];
            assert!(
                sources
                    .enforce_dc_voltage_constraints(&mut solution)
                    .expect("stack projection succeeds")
            );
            assert_eq!(solution[0].to_bits(), (-4.0_f64).to_bits());
            assert_eq!(solution[1].to_bits(), (-4.0_f64).to_bits());
            assert_eq!((solution[0] - solution[1]).to_bits(), 0.0_f64.to_bits());
        }
    }

    #[test]
    fn grounded_source_chain_projects_in_linear_forest_order() {
        let mut sources = VoltageSources::new();
        sources.add("v3".to_string(), 3, 2, 3, -0.5);
        sources.add("v1".to_string(), 1, 0, 1, 1.0);
        sources.add("v2".to_string(), 2, 1, 2, 2.0);
        sources
            .finalize_constraint_projection(3)
            .expect("three-source tree finalizes");

        let mut solution = [0.9, 3.2, 2.4];
        sources
            .enforce_dc_voltage_constraints(&mut solution)
            .expect("chain projection succeeds");

        assert_eq!(
            solution.map(Value::to_bits),
            [1.0, 3.0, 2.5].map(Value::to_bits)
        );
    }

    #[test]
    fn floating_source_stack_preserves_least_squares_common_mode() {
        let mut sources = VoltageSources::new();
        sources.add("v12".to_string(), 2, 1, 1, 2.0);
        sources.add("v23".to_string(), 3, 2, 2, -0.5);
        sources
            .finalize_constraint_projection(3)
            .expect("floating source tree finalizes");

        let raw = [10.0, 13.0, 11.0];
        let expected_common_mode = (raw[0] + (raw[1] - 2.0) + (raw[2] - 1.5)) / 3.0;
        let mut solution = raw;
        sources
            .enforce_dc_voltage_constraints(&mut solution)
            .expect("floating projection succeeds");

        assert_close(solution[0], expected_common_mode);
        assert_close(solution[1] - solution[0], 2.0);
        assert_close(solution[2] - solution[1], -0.5);
    }

    #[test]
    fn cached_source_tree_uses_current_transient_and_scaled_dc_targets() {
        let mut sources = VoltageSources::new();
        sources.add_with_ac_and_spec(
            "vdrive".to_string(),
            1,
            0,
            1,
            2.0,
            0.0,
            0.0,
            Some(SourceSpec::Sin {
                offset: 1.0,
                amplitude: 2.0,
                frequency: 1.0,
                delay: 0.0,
                damping: 0.0,
                phase: 0.0,
            }),
        );
        sources.add("vstack".to_string(), 2, 1, 2, 3.0);
        sources
            .finalize_constraint_projection(2)
            .expect("dynamic source tree finalizes");

        let mut solution = [0.0, 0.0];
        sources
            .enforce_voltage_constraints(&mut solution, 0.0)
            .expect("initial transient projection succeeds");
        assert_eq!(solution.map(Value::to_bits), [1.0, 4.0].map(Value::to_bits));

        sources
            .enforce_voltage_constraints(&mut solution, 0.25)
            .expect("later transient projection succeeds");
        assert_close(solution[0], 3.0);
        assert_close(solution[1], 6.0);

        sources
            .enforce_scaled_dc_voltage_constraints(&mut solution, 0.5)
            .expect("scaled DC projection succeeds");
        assert_eq!(solution.map(Value::to_bits), [1.0, 2.5].map(Value::to_bits));
    }

    #[test]
    fn component_projection_evaluates_each_source_once() {
        let mut sources = VoltageSources::new();
        sources.add("v1".to_string(), 1, 0, 1, 1.0);
        sources.add("v2".to_string(), 2, 1, 2, 2.0);
        sources.add("v3".to_string(), 3, 2, 3, 3.0);
        sources
            .finalize_constraint_projection(3)
            .expect("source tree finalizes");

        let evaluations = [Cell::new(0_usize), Cell::new(0), Cell::new(0)];
        let mut solution = [0.0; 3];
        sources
            .project_constraint_components(&mut solution, |source_index| {
                evaluations[source_index].set(evaluations[source_index].get() + 1);
                sources.dc_values.get(source_index).copied()
            })
            .expect("component projection succeeds");

        assert_eq!(evaluations.map(|count| count.get()), [1, 1, 1]);
    }

    #[test]
    fn invalid_projection_input_never_partially_mutates_solution() {
        let mut sources = VoltageSources::new();
        sources.add("v1".to_string(), 1, 0, 1, 1.0);
        sources.add("v2".to_string(), 2, 1, 2, 2.0);
        sources
            .finalize_constraint_projection(2)
            .expect("source tree finalizes");

        let mut truncated = [7.0];
        let before = truncated.map(Value::to_bits);
        assert!(
            sources
                .enforce_dc_voltage_constraints(&mut truncated)
                .is_err()
        );
        assert_eq!(truncated.map(Value::to_bits), before);

        let mut non_finite = [7.0, Value::NAN];
        let before = non_finite.map(Value::to_bits);
        assert!(
            sources
                .enforce_dc_voltage_constraints(&mut non_finite)
                .is_err()
        );
        assert_eq!(non_finite.map(Value::to_bits), before);

        sources.dc_values[1] = Value::INFINITY;
        let mut bad_target = [7.0, 8.0];
        let before = bad_target.map(Value::to_bits);
        assert!(
            sources
                .enforce_dc_voltage_constraints(&mut bad_target)
                .is_err()
        );
        assert_eq!(bad_target.map(Value::to_bits), before);
    }

    #[test]
    fn ideal_source_loops_are_typed_topology_errors() {
        let mut parallel = VoltageSources::new();
        parallel.add("v1".to_string(), 1, 0, 1, 1.0);
        parallel.add("vparallel".to_string(), 1, 0, 2, 1.0);
        let error = parallel
            .finalize_constraint_projection(1)
            .expect_err("parallel ideal sources have non-unique branch currents");
        let message = error.to_string();
        assert!(message.contains("vparallel"));
        assert!(message.contains("singular ideal-source loop"));

        let mut self_loop = VoltageSources::new();
        self_loop.add("vself".to_string(), 1, 1, 1, 0.0);
        let error = self_loop
            .finalize_constraint_projection(1)
            .expect_err("self-loop ideal source has a non-unique branch current");
        assert!(error.to_string().contains("vself"));
    }

    #[test]
    fn topology_mutation_requires_explicit_projection_refinalization() {
        let mut sources = VoltageSources::new();
        sources.add("v1".to_string(), 1, 0, 1, 1.0);
        sources
            .finalize_constraint_projection(2)
            .expect("initial source topology finalizes");
        sources.node_pos[0] = 2;

        let mut solution = [9.0, 8.0];
        let before = solution.map(Value::to_bits);
        let error = sources
            .enforce_dc_voltage_constraints(&mut solution)
            .expect_err("stale topology is rejected");
        assert!(error.to_string().contains("changed after finalization"));
        assert_eq!(solution.map(Value::to_bits), before);

        sources.invalidate_constraint_projection();
        sources
            .finalize_constraint_projection(2)
            .expect("mutated topology refinalizes");
        sources
            .enforce_dc_voltage_constraints(&mut solution)
            .expect("refinalized topology projects");
        assert_eq!(solution[1].to_bits(), 1.0_f64.to_bits());
    }

    #[test]
    fn xyce_voltage_pulse_advertises_dynamic_device_max_timestep() {
        let mut sources = VoltageSources::new();
        sources.add_with_ac_and_spec(
            "vin".to_string(),
            1,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::Pulse {
                v1: 0.0,
                v2: 1.0,
                delay: 10.0e-9,
                rise: 1.0e-9,
                fall: 1.0e-9,
                width: 10.0e-9,
                period: 30.0e-9,
                phase: 0.0,
                width_defaults_to_zero: false,
            }),
        );
        sources.set_transient_context_with_dialect(
            1.0e-9,
            100.0e-9,
            crate::config::SpiceDialect::Xyce,
        );

        assert_close(
            sources.xyce_max_timestep_at(9.0e-9).expect("pre-delay cap"),
            1.0e-9,
        );
        assert_close(
            sources
                .xyce_max_timestep_at(10.0e-9)
                .expect("post-delay cap"),
            3.0e-9,
        );
    }

    #[test]
    fn voltage_pulse_device_max_timestep_is_xyce_only() {
        let mut sources = VoltageSources::new();
        sources.add_with_ac_and_spec(
            "vin".to_string(),
            1,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::Pulse {
                v1: 0.0,
                v2: 1.0,
                delay: 10.0e-9,
                rise: 1.0e-9,
                fall: 1.0e-9,
                width: 10.0e-9,
                period: 30.0e-9,
                phase: 0.0,
                width_defaults_to_zero: false,
            }),
        );

        sources.set_transient_context_with_dialect(
            1.0e-9,
            100.0e-9,
            crate::config::SpiceDialect::Ngspice,
        );
        assert_eq!(sources.xyce_max_timestep_at(0.0), None);

        sources.set_transient_context(1.0e-9, 100.0e-9);
        assert_eq!(sources.xyce_max_timestep_at(0.0), None);
    }

    #[test]
    fn xyce_voltage_sine_advertises_period_fraction_device_max_timestep() {
        let mut sources = VoltageSources::new();
        sources.add_with_ac_and_spec(
            "vin".to_string(),
            1,
            0,
            1,
            0.0,
            0.0,
            0.0,
            Some(SourceSpec::Sin {
                offset: 0.0,
                amplitude: 1.0,
                frequency: 20.0e6,
                delay: 0.0,
                damping: 0.0,
                phase: 0.0,
            }),
        );
        sources.set_transient_context_with_dialect(
            1.0e-9,
            100.0e-9,
            crate::config::SpiceDialect::Xyce,
        );

        assert_close(sources.xyce_max_timestep_at(0.0).expect("sine cap"), 5.0e-9);

        sources.source_specs[0] = Some(SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: Value::NAN,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        });
        assert_close(
            sources
                .xyce_max_timestep_at(0.0)
                .expect("default-frequency sine cap"),
            10.0e-9,
        );

        sources.source_specs[0] = Some(SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: 0.0,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        });
        assert_close(
            sources
                .xyce_max_timestep_at(0.0)
                .expect("zero-frequency sine cap"),
            10.0e-9,
        );
    }

    #[test]
    fn sin_omitted_frequency_defaults_to_inverse_stop_time() {
        let spec = SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: Value::NAN,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };

        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            2.5e-9,
            transient_context(1.0e-9, 10.0e-9),
        );

        assert_close(value, 1.0);
    }

    #[test]
    fn sin_zero_frequency_defaults_to_inverse_stop_time() {
        let spec = SourceSpec::Sin {
            offset: 0.0,
            amplitude: 1.0,
            frequency: 0.0,
            delay: 0.0,
            damping: 0.0,
            phase: 0.0,
        };

        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            2.5e-9,
            transient_context(1.0e-9, 10.0e-9),
        );

        assert_close(value, 1.0);
    }

    #[test]
    fn exp_omitted_timing_resolves_to_ngspice_tstep_defaults() {
        // EXP(0 1): TD1=TAU1=TAU2=tstep, TD2=TD1+tstep (vsrcload.c).
        let spec = SourceSpec::Exp {
            v1: 0.0,
            v2: 1.0,
            td1: Value::NAN,
            tau1: Value::NAN,
            td2: Value::NAN,
            tau2: Value::NAN,
        };
        let ctx = transient_context(1.0e-9, 10.0e-9);

        // Holds V1 through TD1.
        let early = VoltageSources::evaluate_source_at_time_with_context(&spec, 0.5e-9, ctx);
        assert_close(early, 0.0);

        // Rising region: V1 + (V2-V1)*(1 - exp(-(t-TD1)/TAU1)).
        let rising = VoltageSources::evaluate_source_at_time_with_context(&spec, 1.5e-9, ctx);
        assert_close(rising, 1.0 - (-0.5f64).exp());

        // Decaying region adds the TD2 term.
        let decaying = VoltageSources::evaluate_source_at_time_with_context(&spec, 3.0e-9, ctx);
        assert_close(decaying, (1.0 - (-2.0f64).exp()) - (1.0 - (-1.0f64).exp()));
    }

    #[test]
    fn exp_explicit_zero_timing_also_resolves_to_defaults() {
        // ngspice treats an explicit 0.0 for TD1/TAU1/TD2/TAU2 exactly
        // like an omitted value.
        let spec = SourceSpec::Exp {
            v1: 0.0,
            v2: 1.0,
            td1: 0.0,
            tau1: 0.0,
            td2: 0.0,
            tau2: 0.0,
        };
        let ctx = transient_context(1.0e-9, 10.0e-9);
        let rising = VoltageSources::evaluate_source_at_time_with_context(&spec, 1.5e-9, ctx);
        assert_close(rising, 1.0 - (-0.5f64).exp());
    }

    #[test]
    fn sin_holds_phase_value_before_delay() {
        // SIN with PHASE=90deg holds VO + VA*sin(PHASE) until TD.
        let spec = SourceSpec::Sin {
            offset: 1.0,
            amplitude: 2.0,
            frequency: 1.0e3,
            delay: 5.0e-9,
            damping: 0.0,
            phase: std::f64::consts::FRAC_PI_2,
        };
        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            1.0e-9,
            transient_context(1.0e-9, 10.0e-9),
        );
        assert_close(value, 3.0);
    }

    #[test]
    fn pulse_width_omitted_after_explicit_rise_and_fall_defaults_to_zero() {
        let spec = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 1.0e-9,
            rise: 2.0e-9,
            fall: 3.0e-9,
            width: Value::NAN,
            period: Value::NAN,
            phase: 0.0,
            width_defaults_to_zero: true,
        };

        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            3.5e-9,
            transient_context(0.5e-9, 20.0e-9),
        );

        assert_close(value, 5.0 / 6.0);
    }

    #[test]
    fn pulse_omitted_period_repeats_after_waveform_duration_by_default() {
        let spec = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 10.0e-6,
            rise: 1.0e-6,
            fall: 1.0e-6,
            width: 100.0e-3,
            period: Value::NAN,
            phase: 0.0,
            width_defaults_to_zero: false,
        };

        let value = VoltageSources::evaluate_source_at_time_with_context(
            &spec,
            100.01208e-3,
            transient_context(0.5e-6, 400.0e-3),
        );

        assert_close(value, 0.08);
    }

    #[test]
    /// An omitted PER repeats the waveform, it does not hold the source at V1
    /// until the transient stop.
    ///
    /// Measured against ngspice 46 for exactly this source,
    /// `pulse(0 1 10u 1u 1u 100m)` over `tran 200u 400m`: v(1) rises at
    /// 10.65 us, falls at 100.0117 ms, and then rises again at 100.0130 ms,
    /// 200.0150 ms and 300.0170 ms. That is a period of TR + PW + TF, so the
    /// second cycle begins at TD + PER = 100.012 ms and the source is already
    /// climbing 0.08 us later.
    fn ngspice_pulse_omitted_period_repeats_after_the_waveform_duration() {
        let spec = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 10.0e-6,
            rise: 1.0e-6,
            fall: 1.0e-6,
            width: 100.0e-3,
            period: Value::NAN,
            phase: 0.0,
            width_defaults_to_zero: false,
        };

        let ctx = ngspice_transient_context(0.5e-6, 400.0e-3);
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 100.0115e-3, ctx),
            0.5,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 100.01208e-3, ctx),
            0.08,
        );
        // Second and third cycles land where ngspice put them.
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 150.0e-3, ctx),
            1.0,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 250.0e-3, ctx),
            1.0,
        );
    }

    #[test]
    fn ngspice_two_level_pulse_omitted_width_defaults_to_stop_time() {
        let spec = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 0.0,
            rise: Value::NAN,
            fall: Value::NAN,
            width: Value::NAN,
            period: Value::NAN,
            phase: 0.0,
            width_defaults_to_zero: false,
        };

        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(
                &spec,
                7.0,
                ngspice_transient_context(0.1, 7.0),
            ),
            1.0,
        );
    }

    #[test]
    fn xyce_pulse_omitted_period_defaults_to_transient_stop_time() {
        let spec = SourceSpec::Pulse {
            v1: 0.0,
            v2: 1.0,
            delay: 10.0e-6,
            rise: 1.0e-6,
            fall: 1.0e-6,
            width: 100.0e-3,
            period: Value::NAN,
            phase: 0.0,
            width_defaults_to_zero: false,
        };

        let ctx = xyce_transient_context(0.5e-6, 400.0e-3);
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 100.0115e-3, ctx),
            0.5,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 100.01208e-3, ctx),
            0.0,
        );
    }

    #[test]
    fn pulse_phase_shifts_waveform_like_ngspice_xspice_mode() {
        let spec = SourceSpec::Pulse {
            v1: -1.0,
            v2: 1.0,
            delay: 0.0,
            rise: 1.0e-5,
            fall: 1.0e-5,
            width: 5.0e-4,
            period: 1.0e-3,
            phase: 45.0,
            width_defaults_to_zero: false,
        };

        let ctx = transient_context(2.0e-5, 2.0e-3);
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 0.0, ctx),
            -1.0,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 8.85e-4, ctx),
            1.0,
        );
    }

    #[test]
    fn pwl_delay_and_repeat_match_xyce_source_time_semantics() {
        let spec = SourceSpec::Pwl {
            points: vec![(0.0, 2.0), (2.0, 4.0), (4.0, 0.0)],
            delay: 1.0,
            repeat_from: Some(2.0),
        };

        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 0.5, None),
            0.0,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 1.0, None),
            2.0,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 3.5, None),
            3.0,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 5.5, None),
            3.0,
        );
        assert_close(
            VoltageSources::evaluate_source_at_time_with_context(&spec, 7.0, None),
            0.0,
        );
    }

    #[test]
    fn current_source_pwl_without_dc_uses_zero_transient_baseline() {
        let mut sources = CurrentSources::new();
        sources.add_with_ac_and_spec(
            "is".to_string(),
            1,
            0,
            Value::NAN,
            0.0,
            0.0,
            Some(SourceSpec::Pwl {
                points: vec![(0.0, 0.0), (1.0e-6, 1.0e-3)],
                delay: 0.0,
                repeat_from: None,
            }),
        );

        let mut rhs = vec![0.0];
        sources.stamp_all(&mut rhs);
        assert_eq!(rhs[0], 0.0);

        sources.update_transient_rhs(&mut rhs, 0.5e-6);
        assert_close(rhs[0], -0.5e-3);
        assert_close(sources.max_dc_to_transient_delta(0.5e-6), 0.5e-3);
    }
}
